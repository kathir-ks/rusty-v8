// Converted from V8 C++ source files:
// Header: string-escape-analysis-reducer.h
// Implementation: string-escape-analysis-reducer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/compiler/turboshaft/string-escape-analysis-reducer.h
pub mod string_escape_analysis_reducer {
    use crate::compiler::turboshaft::assembler::Assembler;
    use crate::compiler::turboshaft::graph::Graph;
    use crate::compiler::turboshaft::index::OpIndex;
    use crate::compiler::turboshaft::operations::{
        FrameStateData, FrameStateInfo, FrameStateOp, Operation, Opcode, StringConcatOp, StringLengthOp,
    };
    use crate::compiler::turboshaft::sidetable::FixedOpIndexSidetable;
    use crate::compiler::turboshaft::snapshot_table::V8;
    use crate::zone::zone_containers::ZoneVector;

    use std::cell::RefCell;
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;
    use std::rc::Rc;

    use crate::execution::embedder_state::Context;
    use crate::execution::isolate::Function;
    use crate::execution::messages::Handle;
    use crate::execution::v8threads::Local;

    use crate::compiler::escape_analysis_reducer::EscapeAnalysisReducer;
    use crate::compiler::turboshaft::copying_phase::Block;
    use crate::compiler::turboshaft::deopt_data::{CreateArgumentsType, Instr, MachineType};
    use crate::compiler::turboshaft::machine_lowering_phase::StringEscapeAnalysisReducer;
    use crate::compiler::turboshaft::runtime_call_descriptors::Map;

    use super::{BlockIndex, V};

    pub struct StringEscapeAnalyzer<'a> {
        graph_: &'a Graph,
        zone_: *mut Zone, // Use a raw pointer to Zone
        escaping_operations_and_frame_states_to_reconstruct_: FixedOpIndexSidetable<bool>,
        maybe_non_escaping_string_concats_: ZoneVector<V<String>>,
        maybe_to_reconstruct_frame_states_: ZoneVector<V<FrameState>>,
        max_frame_state_input_count_: u32,
    }

    impl<'a> StringEscapeAnalyzer<'a> {
        pub fn new(graph: &'a Graph, phase_zone: *mut Zone) -> Self {
            StringEscapeAnalyzer {
                graph_: graph,
                zone_: phase_zone,
                escaping_operations_and_frame_states_to_reconstruct_: FixedOpIndexSidetable::new(graph.op_id_count(), false, graph),
                maybe_non_escaping_string_concats_: ZoneVector::new(),
                maybe_to_reconstruct_frame_states_: ZoneVector::new(),
                max_frame_state_input_count_: 0,
            }
        }

        pub fn run(&mut self) {
            for processed in (0..self.graph_.block_count()).rev() {
                let block_index = processed as BlockIndex;
                let block = self.graph_.get(block_index).clone();
                self.process_block(&block);
            }

            self.reprocess_string_concats();

            self.compute_frame_states_to_reconstruct();
        }

        pub fn is_escaping(&self, idx: OpIndex) -> bool {
            assert!(!self.graph_.get(idx).is::<FrameStateOp>());
            self.escaping_operations_and_frame_states_to_reconstruct_.get(idx)
        }

        pub fn should_reconstruct_frame_state(&self, idx: V<FrameState>) -> bool {
            self.escaping_operations_and_frame_states_to_reconstruct_.get(idx)
        }

        fn process_block(&mut self, block: &Block) {
            for index in self.graph_.operation_indices(block).iter().rev() {
                let op = self.graph_.get(*index).clone();
                match op.opcode {
                    Opcode::kFrameState => {
                        self.process_frame_state(
                            V::<FrameState>::cast(*index),
                            op.cast::<FrameStateOp>().clone(),
                        );
                    }
                    Opcode::kStringConcat => {
                        if self.is_escaping(*index) {
                            self.mark_all_inputs_as_escaping(&op);
                        } else {
                            self.maybe_non_escaping_string_concats_
                                .push_back(V::<String>::cast(*index));
                        }
                    }
                    Opcode::kStringLength => {}
                    _ => {
                        self.mark_all_inputs_as_escaping(&op);
                    }
                }
            }
        }

        fn process_frame_state(&mut self, index: V<FrameState>, framestate: FrameStateOp) {
            self.max_frame_state_input_count_ =
                std::cmp::max(self.max_frame_state_input_count_, framestate.input_count);

            for input_idx in framestate.inputs() {
                if self.graph_.get(input_idx).is::<StringConcatOp>() {
                    self.maybe_to_reconstruct_frame_states_.push_back(index);
                    break;
                }
            }

            let mut it = framestate.data.iterator(framestate.state_values());

            self.mark_next_frame_state_input_as_escaping(&mut it);

            self.mark_next_frame_state_input_as_escaping(&mut it);
        }

        fn mark_next_frame_state_input_as_escaping(&mut self, it: &mut FrameStateData::Iterator) {
            match it.current_instr() {
                Instr::kInput => {
                    let (type_, input) = it.consume_input();
                    self.mark_as_escaping(input);
                }
                Instr::kArgumentsElements => {
                    it.consume_arguments_elements();
                }
                Instr::kArgumentsLength => {
                    it.consume_arguments_length();
                }
                Instr::kRestLength => {
                    it.consume_rest_length();
                }
                Instr::kDematerializedObjectReference => {
                    it.consume_dematerialized_object_reference();
                }
                Instr::kDematerializedObject => {
                    it.consume_dematerialized_object();
                }
                Instr::kDematerializedStringConcat => {}
                Instr::kDematerializedStringConcatReference => {}
                Instr::kUnusedRegister => {
                    it.consume_unused_register();
                }
            }
        }

        fn mark_all_inputs_as_escaping(&mut self, op: &Operation) {
            for input in op.inputs() {
                if !self.graph_.get(input).is::<FrameStateOp>() {
                    self.mark_as_escaping(input);
                }
            }
        }

        fn recursively_mark_all_string_concat_inputs_as_escaping(
            &mut self,
            concat: &StringConcatOp,
        ) {
            let mut to_mark: Vec<&StringConcatOp> = Vec::new();
            to_mark.push(concat);

            while !to_mark.is_empty() {
                let curr = to_mark.pop().unwrap();

                for input_index in curr.inputs() {
                    let input = self.graph_.get(input_index);
                    if input.is::<StringConcatOp>() && !self.is_escaping(input_index) {
                        self.mark_as_escaping(input_index);
                        to_mark.push(input.cast::<StringConcatOp>());
                    }
                }
            }
        }

        fn reprocess_string_concats(&mut self) {
            if (self.maybe_non_escaping_string_concats_.len() as u32) + self.max_frame_state_input_count_
                > u32::MAX
            {
                for index in &self.maybe_non_escaping_string_concats_ {
                    self.mark_as_escaping(*index);
                }
            }

            for index in &self.maybe_non_escaping_string_concats_ {
                if self.is_escaping(*index) {
                    self.recursively_mark_all_string_concat_inputs_as_escaping(
                        &self.graph_.get(*index).cast::<StringConcatOp>().clone(),
                    );
                }
            }
        }

        fn compute_frame_states_to_reconstruct(&mut self) {
            for frame_state_idx in &self.maybe_to_reconstruct_frame_states_ {
                let frame_state = self.graph_.get(*frame_state_idx).cast::<FrameStateOp>().clone();
                let mut should_reconstruct = false;
                for input in frame_state.inputs() {
                    if self.graph_.get(input).is::<StringConcatOp>() && !self.is_escaping(input) {
                        should_reconstruct = true;
                        break;
                    }
                }
                if should_reconstruct {
                    self.recursive_mark_as_should_reconstruct(*frame_state_idx);
                }
            }
        }

        fn mark_as_escaping(&mut self, index: OpIndex) {
            assert!(!self.graph_.get(index).is::<FrameStateOp>());
            self.escaping_operations_and_frame_states_to_reconstruct_.set(index, true);
        }

        fn recursive_mark_as_should_reconstruct(&mut self, idx: V<FrameState>) {
            self.escaping_operations_and_frame_states_to_reconstruct_.set(idx, true);
            let mut frame_state = self.graph_.get(idx).cast::<FrameStateOp>().clone();

            while frame_state.inlined {
                let parent = frame_state.parent_frame_state();
                self.escaping_operations_and_frame_states_to_reconstruct_.set(parent, true);
                frame_state = self.graph_.get(parent).cast::<FrameStateOp>().clone();
            }
        }
    }

    pub struct StringEscapeAnalysisReducerImpl<Next> {
        next: Next,
        asm: Rc<RefCell<Assembler>>,
        analyzer_: RefCell<StringEscapeAnalyzer<'static>>, // Replaced Zone* with a Zone instance
        elided_strings_: RefCell<HashMap<V<String>, (ElidedStringPart, ElidedStringPart)>>,
        deduplicators_: RefCell<SparseOpIndexSideTable<Deduplicator>>,
    }

    impl<Next> StringEscapeAnalysisReducerImpl<Next> {
        pub fn new(next: Next, asm: Rc<RefCell<Assembler>>) -> Self {
            let graph = asm.borrow().input_graph();
            let phase_zone = asm.borrow().phase_zone();
             let analyzer = StringEscapeAnalyzer::new(asm.borrow().input_graph(), asm.borrow().phase_zone());

            Self {
                next,
                asm: asm.clone(),
                analyzer_: RefCell::new(analyzer),
                elided_strings_: RefCell::new(HashMap::new()),
                deduplicators_: RefCell::new(SparseOpIndexSideTable::new(asm.borrow().phase_zone(), graph)),
            }
        }

        pub fn analyze(&self) {
            if v8_flags::turboshaft_string_concat_escape_analysis() {
                self.analyzer_.borrow_mut().run();
            }
            //self.next.analyze();
        }

        pub fn reduce_input_graph_string_concat(
            &self,
            ig_index: V<String>,
            op: &StringConcatOp,
        ) -> V<String> {
             if !v8_flags::turboshaft_string_concat_escape_analysis() || !self.analyzer_.borrow().is_escaping(ig_index){
                return self.next.reduce_input_graph_string_concat(ig_index, op);
            }
            let mut elided_strings = self.elided_strings_.borrow_mut();

            let left = self.get_elided_string_input(op.left());
            let right = self.get_elided_string_input(op.right());
            elided_strings.insert(ig_index, (left, right));
            V::<String>::invalid()
        }

        pub fn reduce_input_graph_frame_state(
            &self,
            ig_index: V<FrameState>,
            frame_state: &FrameStateOp,
        ) -> V<FrameState> {
            if !v8_flags::turboshaft_string_concat_escape_analysis() || !self.analyzer_.borrow().should_reconstruct_frame_state(ig_index) {
                return self.next.reduce_input_graph_frame_state(ig_index, frame_state);
            }
            self.build_frame_state(frame_state, ig_index)
        }

        pub fn reduce_input_graph_string_length(
            &self,
            ig_index: V<Word32>,
            op: &StringLengthOp,
        ) -> V<Word32> {
             if !v8_flags::turboshaft_string_concat_escape_analysis(){
               return self.next.reduce_input_graph_string_length(ig_index, op);
            }

            let input_index = op.string();
            let input = self.asm.borrow().input_graph().get(input_index);

           if let Some(input) = input.try_cast::<StringConcatOp>(){
                if !self.analyzer_.borrow().is_escaping(input_index){
                  let untag =  self.asm.borrow().untag_smi(self.asm.borrow().map_to_new_graph(input.length()));
                   return untag;
                }
            }

              self.next.reduce_input_graph_string_length(ig_index, op)
        }

        fn build_frame_state(
            &self,
            input_frame_state: &FrameStateOp,
            ig_index: OpIndex,
        ) -> V<FrameState> {
            assert!(v8_flags::turboshaft_string_concat_escape_analysis());
            let info = &input_frame_state.data.frame_state_info;

            let mut builder = FrameStateData::Builder::new();
            let mut it = input_frame_state
                .data
                .iterator(input_frame_state.state_values());

             let mut deduplicators = self.deduplicators_.borrow_mut();
            let mut deduplicator;
              if input_frame_state.inlined {
                 let parent_ig_index = input_frame_state.parent_frame_state();
                let parent_ig_index_v = V::<FrameState>::from_usize(parent_ig_index.to_usize());
                builder.add_parent_frame_state(self.asm.borrow().map_to_new_graph(parent_ig_index));

                assert!(self.analyzer_.borrow().should_reconstruct_frame_state(parent_ig_index_v));
                 deduplicator = if let Some(dedup) = deduplicators.get(parent_ig_index) {
                    dedup.clone_for_zone(self.asm.borrow().phase_zone())
                 }else{
                    Deduplicator::new(self.asm.borrow().phase_zone())
                 };

            } else {
                deduplicator = Deduplicator::new(self.asm.borrow().phase_zone());
            }
           deduplicators.set(ig_index, deduplicator.clone());

            self.build_frame_state_input(&mut builder, &mut it, &deduplicator);

            for _i in 0..info.parameter_count() {
                self.build_frame_state_input(&mut builder, &mut it, &deduplicator);
            }

            self.build_frame_state_input(&mut builder, &mut it, &deduplicator);

            for _i in 0..info.local_count() {
                self.build_frame_state_input(&mut builder, &mut it, &deduplicator);
            }

            for _i in 0..info.stack_count() {
                self.build_frame_state_input(&mut builder, &mut it, &deduplicator);
            }

            let data = builder.allocate_frame_state_data(info, self.asm.borrow().graph_zone());

            self.asm.borrow().frame_state(
                builder.inputs(),
                builder.inlined(),
                data,
            )
        }

        fn build_frame_state_input(
            &self,
            builder: &mut FrameStateData::Builder,
            it: &mut FrameStateData::Iterator,
            deduplicator: &Deduplicator,
        ) {
            match it.current_instr() {
                Instr::kInput => {
                    let (type_, input) = it.consume_input();
                     let elided_strings = self.elided_strings_.borrow();
                    if elided_strings.contains_key(&V::<String>::from_usize(input.to_usize())) {
                        assert!(type_.is_tagged());
                        self.build_maybe_elided_string(
                            builder,
                            ElidedStringPart::Elided(V::<String>::from_usize(input.to_usize())),
                            deduplicator,
                        );
                    } else {
                        builder.add_input(type_, self.asm.borrow().map_to_new_graph(input));
                    }
                }
                Instr::kDematerializedObject => {
                    let (old_id, field_count) = it.consume_dematerialized_object();
                    builder.add_dematerialized_object(old_id, field_count);
                    for _i in 0..field_count {
                        self.build_frame_state_input(builder, it, deduplicator);
                    }
                }
                Instr::kDematerializedObjectReference => {
                    let old_id = it.consume_dematerialized_object_reference();
                    builder.add_dematerialized_object_reference(old_id);
                }
                Instr::kArgumentsElements => {
                    let type_ = it.consume_arguments_elements();
                    builder.add_arguments_elements(type_);
                }
                Instr::kArgumentsLength => {
                    it.consume_arguments_length();
                    builder.add_arguments_length();
                }
                Instr::kRestLength => {
                    it.consume_rest_length();
                    builder.add_rest_length();
                }
                Instr::kUnusedRegister => {
                    it.consume_unused_register();
                    builder.add_unused_register();
                }
                Instr::kDematerializedStringConcat | Instr::kDematerializedStringConcatReference => {
                    unreachable!();
                }
            }
        }

        fn build_maybe_elided_string(
            &self,
            builder: &mut FrameStateData::Builder,
            maybe_elided: ElidedStringPart,
            deduplicator: &Deduplicator,
        ) {
            if maybe_elided.is_elided() {
                let dup_id = deduplicator.get_duplicated_id_for_elided_string(maybe_elided);
                if dup_id.duplicated {
                    builder.add_dematerialized_string_concat_reference(dup_id.id);
                    return;
                }
                builder.add_dematerialized_string_concat(dup_id.id);
                 let elided_strings = self.elided_strings_.borrow();
                let inputs = elided_strings
                    .get(&maybe_elided.ig_index())
                    .unwrap();
                self.build_maybe_elided_string(builder, inputs.0, deduplicator);
                self.build_maybe_elided_string(builder, inputs.1, deduplicator);
            } else {
                builder.add_input(MachineType::AnyTagged(), maybe_elided.og_index());
            }
        }

        fn get_elided_string_input(&self, ig_index: OpIndex) -> ElidedStringPart {
             let elided_strings = self.elided_strings_.borrow();
            if elided_strings.contains_key(&V::<String>::from_usize(ig_index.to_usize())) {
                ElidedStringPart::Elided(V::<String>::from_usize(ig_index.to_usize()))
            } else {
                ElidedStringPart::NotElided(self.asm.borrow().map_to_new_graph(ig_index))
            }
        }
    }

    #[allow(non_camel_case_types)]
    pub trait StringEscapeAnalysisReducer<Next> {
        fn new(next: Next, asm: Rc<RefCell<Assembler>>) -> StringEscapeAnalysisReducerImpl<Next>;
    }

    impl<Next> StringEscapeAnalysisReducer<Next> for StringEscapeAnalysisReducerImpl<Next> {
        fn new(next: Next, asm: Rc<RefCell<Assembler>>) -> StringEscapeAnalysisReducerImpl<Next> {
            StringEscapeAnalysisReducerImpl::new(next, asm)
        }
    }

    pub struct ElidedStringPart {
        kind: Kind,
        data: Data,
    }

    union Data {
        og_index: V<String>,
        ig_index: V<String>,
    }

    impl ElidedStringPart {
        fn elided(ig_index: V<String>) -> Self {
            ElidedStringPart {
                kind: Kind::kElided,
                data: Data { ig_index },
            }
        }

        fn not_elided(og_index: V<String>) -> Self {
            ElidedStringPart {
                kind: Kind::kNotElided,
                data: Data { og_index },
            }
        }

        fn is_elided(&self) -> bool {
            self.kind == Kind::kElided
        }

        fn og_index(&self) -> V<String> {
            assert_eq!(self.kind, Kind::kNotElided);
            unsafe { self.data.og_index }
        }

        fn ig_index(&self) -> V<String> {
            assert_eq!(self.kind, Kind::kElided);
            unsafe { self.data.ig_index }
        }

        fn invalid() -> Self {
            ElidedStringPart {
                kind: Kind::kNotElided,
                data: Data {
                    og_index: V::<String>::invalid(),
                },
            }
        }

        fn new(kind: Kind, index: V<String>) -> Self {
            ElidedStringPart {
                kind,
                data: Data { og_index: index },
            }
        }

        fn clone(&self) -> Self {
            ElidedStringPart {
                kind: self.kind,
                data: match self.kind {
                    Kind::kElided => Data {
                        ig_index: unsafe { self.data.ig_index },
                    },
                    Kind::kNotElided => Data {
                        og_index: unsafe { self.data.og_index },
                    },
                },
            }
        }
    }

    impl PartialEq for ElidedStringPart {
        fn eq(&self, other: &Self) -> bool {
            if self.kind != other.kind {
                return false;
            }
            match self.kind {
                Kind::kElided => self.ig_index() == other.ig_index(),
                Kind::kNotElided => self.og_index() == other.og_index(),
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Kind {
        kNotElided,
        kElided,
    }

    #[derive(Clone)]
    struct Deduplicator {
        string_ids_: Rc<RefCell<ZoneVector<ElidedStringPart>>>,
         zone_: *mut Zone,
    }

    impl Deduplicator {
        fn new(zone: *mut Zone) -> Self {
            Deduplicator {
                string_ids_: Rc::new(RefCell::new(ZoneVector::new())),
                zone_: zone,
            }
        }

          fn clone_for_zone(&self, zone: *mut Zone) -> Self {
                Deduplicator {
                    string_ids_: Rc::clone(&self.string_ids_),
                    zone_: zone,
                }
            }

        fn get_duplicated_id_for_elided_string(&self, index: ElidedStringPart) -> DuplicatedId {
            let string_ids = self.string_ids_.borrow();
            for (id, elided_string) in string_ids.iter().enumerate() {
                if *elided_string == index {
                    return DuplicatedId {
                        id: id as u32,
                        duplicated: true,
                    };
                }
            }

            let mut string_ids_mut = self.string_ids_.borrow_mut();
            let new_id = string_ids_mut.len() as u32;
            string_ids_mut.push(index.clone());

            DuplicatedId {
                id: new_id,
                duplicated: false,
            }
        }
    }

    struct DuplicatedId {
        id: u32,
        duplicated: bool,
    }

    struct SparseOpIndexSideTable<T> {
        table: RefCell<HashMap<OpIndex, T>>,
         zone_: *mut Zone,
        graph: *const Graph
    }

    impl<T> SparseOpIndexSideTable<T> {
        fn new(zone: *mut Zone, graph: *const Graph) -> Self
        where
            T: Clone,
        {
            SparseOpIndexSideTable {
                table: RefCell::new(HashMap::new()),
                zone_: zone,
                graph: graph
            }
        }

          fn get(&self, index: OpIndex) -> Option<T>
        where
            T: Clone,
        {
            self.table.borrow().get(&index).cloned()
        }

        fn set(&self, index: OpIndex, value: T)
        where
            T: Clone,
        {
            self.table.borrow_mut().insert(index, value);
        }

        fn remove(&self, index: OpIndex) {
            self.table.borrow_mut().remove(&index);
        }

          fn clear(&self) {
            self.table.borrow_mut().clear();
        }

        fn contains(&self, index: OpIndex) -> bool {
             self.table.borrow().contains_key(&index)
        }
    }

    // Dummy implementations for traits/structs used in the code
    pub trait NextReducer {
        fn analyze(&self);
        fn reduce_input_graph_string_concat(&self, ig_index: V<String>, op: &StringConcatOp) -> V<String>;
        fn reduce_input_graph_frame_state(&self, ig_index: V<FrameState>, frame_state: &FrameStateOp) -> V<FrameState>;
        fn reduce_input_graph_string_length(&self, ig_index: V<Word32>, op: &StringLengthOp) -> V<Word32>;
    }

    pub struct DefaultNextReducer {}

    impl DefaultNextReducer {
        pub fn new() -> Self {
            DefaultNextReducer {}
        }
    }

    impl NextReducer for DefaultNextReducer {
        fn analyze(&self) {}
         fn reduce_input_graph_string_concat(&self, _ig_index: V<String>, _op: &StringConcatOp) -> V<String> {
            V::<String>::invalid()
         }
          fn reduce_input_graph_frame_state(&self, _ig_index: V<FrameState>, _frame_state: &FrameStateOp) -> V<FrameState> {
            V::<FrameState>::invalid()
        }
         fn reduce_input_graph_string_length(&self, _ig_index: V<Word32>, _op: &StringLengthOp) -> V<Word32> {
            V::<Word32>::invalid()
         }
    }

    pub mod v8_flags {
        pub fn turboshaft_string_concat_escape_analysis() -> bool {
            true
        }
    }

    // Zone stub
    pub struct Zone {}

    impl Zone {
        pub fn new() -> Self {
            Zone {}
        }
    }
    #[derive(Clone, Copy)]
    pub struct Word32{
       value: u32
    }

      impl Word32{
        pub fn new(value: u32) -> Self{
            Word32{value}
        }
    }
}
