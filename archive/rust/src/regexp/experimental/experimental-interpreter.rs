// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod experimental {
    pub mod experimental_interpreter;
}

use std::{cell::RefCell, rc::Rc};

//use crate::objects::string::String; // Assuming a String type exists
//use crate::regexp::experimental::experimental::IsRegExpWord; // Assuming this function exists
//use crate::sandbox::check::StackLimitCheck; // Assuming StackLimitCheck exists

const K_UNDEFINED_REGISTER_VALUE: i32 = -1;
const K_UNDEFINED_MATCH_INDEX_VALUE: i32 = -1;
const K_UNDEFINED_CLOCK_VALUE: u64 = u64::MAX; // Assuming u64::MAX is a suitable representation

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RegExpAssertionType {
    START_OF_INPUT,
    END_OF_INPUT,
    START_OF_LINE,
    END_OF_LINE,
    BOUNDARY,
    NON_BOUNDARY,
}

fn satisfies_assertion<Character: Copy + PartialEq>(
    assertion_type: RegExpAssertionType,
    context: &[Character],
    position: usize,
) -> bool {
    assert!(position <= context.len());
    assert!(position >= 0);

    match assertion_type {
        RegExpAssertionType::START_OF_INPUT => position == 0,
        RegExpAssertionType::END_OF_INPUT => position == context.len(),
        RegExpAssertionType::START_OF_LINE => {
            if position == 0 {
                return true;
            }
            // Assuming unibrow::IsLineTerminator is implemented
            //unibrow::IsLineTerminator(context[position - 1])
            false // Placeholder
        }
        RegExpAssertionType::END_OF_LINE => {
            if position == context.len() {
                return true;
            }
            // Assuming unibrow::IsLineTerminator is implemented
            //unibrow::IsLineTerminator(context[position])
            false // Placeholder
        }
        RegExpAssertionType::BOUNDARY => {
            if context.is_empty() {
                return false;
            } else if position == 0 {
                // Assuming IsRegExpWord is implemented
                //IsRegExpWord(context[position])
                false // Placeholder
            } else if position == context.len() {
                // Assuming IsRegExpWord is implemented
                //IsRegExpWord(context[position - 1])
                false // Placeholder
            } else {
                // Assuming IsRegExpWord is implemented
                //IsRegExpWord(context[position - 1]) != IsRegExpWord(context[position])
                false // Placeholder
            }
        }
        RegExpAssertionType::NON_BOUNDARY => {
            !satisfies_assertion(RegExpAssertionType::BOUNDARY, context, position)
        }
    }
}

// Placeholder enum for RegExpInstruction
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RegExpInstructionOpcode {
    FILTER_CHILD,
    FILTER_GROUP,
    FILTER_QUANTIFIER,
    FILTER_LOOKAROUND,
    SET_QUANTIFIER_TO_CLOCK,
    CLEAR_REGISTER,
    SET_REGISTER_TO_CP,
    ASSERTION,
    FORK,
    JMP,
    ACCEPT,
    BEGIN_LOOP,
    END_LOOP,
    START_LOOKAROUND,
    END_LOOKAROUND,
    WRITE_LOOKAROUND_TABLE,
    READ_LOOKAROUND_TABLE,
    CONSUME_RANGE,
    RANGE_COUNT,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RegExpInstruction {
    pub opcode: RegExpInstructionOpcode,
    pub payload: RegExpInstructionPayload,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RegExpInstructionPayload {
    Pc(i32),
    Group { group_id: i32 },
    Quantifier { quantifier_id: i32 },
    Lookaround { lookaround_id: i32, is_positive: bool, index: i32},
    AssertionType(RegExpAssertionType),
    NumRanges(i32),
    ConsumeRange { min: u16, max: u16 },
    None,
}

// Dummy implementation of IsFilter
impl RegExpInstruction {
    pub fn is_filter(&self) -> bool {
        match self.opcode {
            RegExpInstructionOpcode::FILTER_CHILD
            | RegExpInstructionOpcode::FILTER_GROUP
            | RegExpInstructionOpcode::FILTER_QUANTIFIER
            | RegExpInstructionOpcode::FILTER_LOOKAROUND => true,
            _ => false,
        }
    }
}

fn to_instruction_vector(raw_bytes: &[u8]) -> Vec<RegExpInstruction> {
    let inst_num = raw_bytes.len() / std::mem::size_of::<RegExpInstruction>();
    assert_eq!(
        std::mem::size_of::<RegExpInstruction>() * inst_num,
        raw_bytes.len()
    );

    let mut instructions: Vec<RegExpInstruction> = Vec::with_capacity(inst_num);

    for i in 0..inst_num {
        let start = i * std::mem::size_of::<RegExpInstruction>();
        let end = (i + 1) * std::mem::size_of::<RegExpInstruction>();
        let instruction_bytes = &raw_bytes[start..end];

        let instruction: RegExpInstruction = unsafe {
            std::ptr::read(instruction_bytes.as_ptr() as *const RegExpInstruction)
        };
        instructions.push(instruction);
    }

    instructions
}

fn to_character_vector_u8(str_val: &[u8]) -> Vec<u8> {
    str_val.to_vec()
}

fn to_character_vector_uc16(str_val: &[u16]) -> Vec<u16> {
    str_val.to_vec()
}

struct FilterGroups<'a> {
    pc_: usize,
    max_clock_: u64,
    pc_stack_: Vec<usize>,
    max_clock_stack_: Vec<u64>,
    bytecode_: &'a [RegExpInstruction],
}

impl<'a> FilterGroups<'a> {
    fn new(pc: usize, bytecode: &'a [RegExpInstruction]) -> Self {
        FilterGroups {
            pc_: pc,
            max_clock_: 0,
            pc_stack_: Vec::new(),
            max_clock_stack_: Vec::new(),
            bytecode_: bytecode,
        }
    }

    fn up(&mut self) {
        if let Some(pc) = self.pc_stack_.pop() {
            self.pc_ = pc;
            self.max_clock_ = self.max_clock_stack_.pop().unwrap();
        }
    }

    fn increment_pc(&mut self) {
        if self.is_at_node_end() {
            self.up();
        } else {
            self.pc_ += 1;
        }
    }

    fn is_at_node_end(&self) -> bool {
        self.pc_ + 1 == self.bytecode_.len()
            || self.bytecode_[self.pc_ + 1].opcode != RegExpInstructionOpcode::FILTER_CHILD
    }

    fn run(
        &mut self,
        registers_: &mut [i32],
        quantifiers_clocks_: &mut [u64],
        capture_clocks_: &mut [u64],
        lookaround_clocks: &mut Option<&mut [u64]>,
        filtered_registers_: &mut [i32],
    ) -> Vec<i32> {
        self.pc_stack_.push(self.pc_);
        self.max_clock_stack_.push(self.max_clock_);

        while !self.pc_stack_.is_empty() {
            let instr = self.bytecode_[self.pc_];
            match instr.opcode {
                RegExpInstructionOpcode::FILTER_CHILD => {
                    if !self.is_at_node_end() {
                        self.pc_stack_.push(self.pc_ + 1);
                        self.max_clock_stack_.push(self.max_clock_);
                    }

                    self.pc_ = match instr.payload {
                        RegExpInstructionPayload::Pc(pc) => pc as usize,
                        _ => panic!("Expected PC payload"),
                    };
                }

                RegExpInstructionOpcode::FILTER_GROUP => {
                    let group_id = match instr.payload {
                        RegExpInstructionPayload::Group { group_id } => group_id as usize,
                        _ => panic!("Expected Group payload"),
                    };
                    let register_id = 2 * group_id;

                    if capture_clocks_[register_id] >= self.max_clock_
                        && capture_clocks_[register_id] != K_UNDEFINED_CLOCK_VALUE
                    {
                        filtered_registers_[register_id] = registers_[register_id];
                        filtered_registers_[register_id + 1] = registers_[register_id + 1];
                        self.increment_pc();
                    } else {
                        self.up();
                    }
                }

                RegExpInstructionOpcode::FILTER_QUANTIFIER => {
                    let quantifier_id = match instr.payload {
                        RegExpInstructionPayload::Quantifier { quantifier_id } => quantifier_id as usize,
                        _ => panic!("Expected Quantifier payload"),
                    };

                    if quantifiers_clocks_[quantifier_id] >= self.max_clock_ {
                        self.max_clock_ = quantifiers_clocks_[quantifier_id];
                        self.increment_pc();
                    } else {
                        self.up();
                    }
                }

                RegExpInstructionOpcode::FILTER_LOOKAROUND => {
                  let lookaround_id = match instr.payload {
                    RegExpInstructionPayload::Lookaround { lookaround_id, .. } => lookaround_id as usize,
                    _ => panic!("Expected Lookaround payload"),
                  };

                    if let Some(lookaround_clocks_) = lookaround_clocks.as_mut() {
                        if lookaround_clocks_[lookaround_id] >= self.max_clock_ {
                            self.increment_pc();
                        } else {
                            self.up();
                        }
                    } else {
                        // Handle the case where lookaround_clocks is None (optional)
                        self.up(); // Or some other appropriate action
                    }
                }

                _ => panic!("Unreachable"),
            }
        }

        filtered_registers_.to_vec()
    }
}

// Placeholder for Isolate, CallOrigin, TrustedByteArray, String, and RegExp
// You'll need to define these types appropriately for your environment.

//type Isolate = ();
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CallOrigin {
    kFromJs,
    kFromRuntime,
}
//type TrustedByteArray = Vec<u8>;
//type String = Vec<u8>; // Placeholder.  String should be a proper string type.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InternalRegExpResult {
    Success,
    Exception,
    Retry,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RegExpLookaroundType {
    LOOKAHEAD,
    LOOKBEHIND,
}

struct NfaInterpreter<Character: Copy + PartialEq> {
    //isolate_: *mut Isolate,
    call_origin_: CallOrigin,
    bytecode_object_: Vec<u8>,
    bytecode_: Vec<RegExpInstruction>,
    register_count_per_match_: usize,
    quantifier_count_: usize,
    input_object_: Vec<u8>,
    input_: Vec<Character>,
    input_index_: usize,
    clock: u64,
    pc_last_input_index_: Vec<LastInputIndex>,
    active_threads_: Vec<InterpreterThread>,
    blocked_threads_: Vec<InterpreterThread>,
    register_array_allocator_: Vec<i32>,
    lookaround_match_index_array_allocator_: Option<Vec<i32>>,
    lookaround_clock_array_allocator_: Option<Vec<u64>>,
    quantifier_array_allocator_: Option<Vec<u64>>,
    capture_clock_array_allocator_: Option<Vec<u64>>,
    best_match_thread_: Option<InterpreterThread>,
    lookarounds_: Vec<Lookaround>,
    lookaround_table_: Option<Vec<Vec<bool>>>,
    lookbehind_table_: Option<Vec<bool>>,
    only_captureless_lookbehinds_: bool,
    reverse_: bool,
    current_lookaround_: i32,
    filter_groups_pc_: Option<usize>,
    memory_consumption_per_thread_: usize,
    //zone_: *mut Zone,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ConsumedCharacter {
  DidConsume,
  DidNotConsume,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct InterpreterThread {
    pc: usize,
    register_array_begin: usize,
    lookaround_match_index_array_begin: usize,
    quantifier_clock_array_begin: usize,
    captures_clock_array_begin: usize,
    lookaround_clock_array_begin: usize,
    consumed_since_last_quantifier: ConsumedCharacter,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Lookaround {
  match_pc: i32,
  capture_pc: i32,
  type_: RegExpLookaroundType,
}

impl<Character: Copy + PartialEq> NfaInterpreter<Character> {
  fn new(
    //isolate: *mut Isolate,
    call_origin: CallOrigin,
    bytecode: Vec<u8>,
    register_count_per_match: usize,
    input: Vec<u8>,
    input_index: usize,
    //zone: *mut Zone,
  ) -> Self {
    let bytecode_ = to_instruction_vector(&bytecode);
    let input_ = if std::any::TypeId::of::<Character>() == std::any::TypeId::of::<u8>() {
      // Assuming that Character is u8
      unsafe {
        std::mem::transmute::<Vec<u8>, Vec<Character>>(input.clone())
      }
    } else if std::any::TypeId::of::<Character>() == std::any::TypeId::of::<u16>() {
      // Assuming that Character is u16
      let input_u16: Vec<u16> = input.chunks(2).map(|chunk| {
        u16::from_le_bytes([chunk[0], chunk[1]])
      }).collect();

      unsafe {
        std::mem::transmute::<Vec<u16>, Vec<Character>>(input_u16)
      }
    } else {
      panic!("Unsupported character type");
    };

    let mut interpreter = NfaInterpreter {
        //isolate_: isolate,
        call_origin_: call_origin,
        bytecode_object_: bytecode.clone(),
        bytecode_: bytecode_.clone(),
        register_count_per_match_: register_count_per_match,
        quantifier_count_: 0,
        input_object_: input.clone(),
        input_: input_.clone(),
        input_index_: input_index,
        clock: 0,
        pc_last_input_index_: vec![LastInputIndex::new(-1, -1); bytecode_.len()],
        active_threads_: Vec::new(),
        blocked_threads_: Vec::new(),
        register_array_allocator_: vec![0; register_count_per_match],
        lookaround_match_index_array_allocator_: None,
        lookaround_clock_array_allocator_: None,
        quantifier_array_allocator_: None,
        capture_clock_array_allocator_: None,
        best_match_thread_: None,
        lookarounds_: Vec::new(),
        lookaround_table_: None,
        lookbehind_table_: None,
        only_captureless_lookbehinds_: true,
        reverse_: false,
        current_lookaround_: -1,
        filter_groups_pc_: None,
        memory_consumption_per_thread_: 0,
        //zone_: zone,
    };

    // Iterate over the bytecode to find the PC of the filtering
    // instructions and lookarounds, and the number of quantifiers.
    let mut lookaround: Option<Lookaround> = None;
    let mut in_lookaround = false;
    let mut lookaround_index = 0;

    for i in 0..(bytecode_.len() - 1) {
        let inst = bytecode_[i];

        if inst.opcode == RegExpInstructionOpcode::START_LOOKAROUND {
            assert!(lookaround.is_none());
            in_lookaround = true;

            // Stores the partial information for a lookaround. The rest will be
            // determined upon reaching a `WRITE_LOOKAROUND_TABLE` instruction.
            lookaround_index = match inst.payload {
                RegExpInstructionPayload::Lookaround { index, .. } => index as usize,
                _ => panic!("Expected Lookaround index"),
            };

            let lookaround_type = match inst.payload {
              RegExpInstructionPayload::Lookaround { type_: lookaround_type, .. } => match lookaround_type {
                  0 => RegExpLookaroundType::LOOKBEHIND,
                  1 => RegExpLookaroundType::LOOKAHEAD,
                  _ => panic!("Unexpected lookaround type value")
              },
              _ => panic!("Expected Lookaround type")
            };

            lookaround = Some(Lookaround{ match_pc: i as i32, capture_pc: -1, type_: lookaround_type});

            match lookaround_type {
              RegExpLookaroundType::LOOKAHEAD => {
                interpreter.only_captureless_lookbehinds_ = false;
              },
              _ => {}
            }
        }

        if inst.opcode == RegExpInstructionOpcode::SET_REGISTER_TO_CP && in_lookaround {
            interpreter.only_captureless_lookbehinds_ = false;
        }

        if inst.opcode == RegExpInstructionOpcode::WRITE_LOOKAROUND_TABLE {
            assert!(lookaround.is_some());

            // Fills the current lookaround data.
            let mut current_lookaround = lookaround.unwrap();
            current_lookaround.capture_pc = i as i32 + 1;

            // Since the lookarounds are not in order in the `lookarounds_` array,
            // we first fill it until it has the correct size.
            while interpreter.lookarounds_.len() <= lookaround_index {
                interpreter.lookarounds_.push(Lookaround {
                    match_pc: -1,
                    capture_pc: -1,
                    type_: RegExpLookaroundType::LOOKBEHIND,
                });
            }
            interpreter.lookarounds_[lookaround_index] = current_lookaround;
            lookaround = None;
        }

        if inst.opcode == RegExpInstructionOpcode::END_LOOKAROUND {
            in_lookaround = false;
        }

        // The first `FILTER_*` instruction encountered is the start of the
        // `FILTER_*` section.
        if interpreter.filter_groups_pc_.is_none() && inst.is_filter() {
            //DCHECK(v8_flags.experimental_regexp_engine_capture_group_opt);
            interpreter.filter_groups_pc_ = Some(i);
        }

        if inst.opcode == RegExpInstructionOpcode::SET_QUANTIFIER_TO_CLOCK {
            //DCHECK(v8_flags.experimental_regexp_engine_capture_group_opt);
            match inst.payload {
                RegExpInstructionPayload::Quantifier { quantifier_id } => {
                    interpreter.quantifier_count_ =
                        std::cmp::max(interpreter.quantifier_count_, (quantifier_id + 1) as usize);
                }
                _ => panic!("Expected Quantifier payload"),
            }
        }
    }

    // Iniitializes the lookaround truth table and required allocators.
    if interpreter.only_captureless_lookbehinds_ {
        interpreter.lookbehind_table_ = Some(vec![false; interpreter.lookarounds_.len()]);
    } else {
        //DCHECK(v8_flags.experimental_regexp_engine_capture_group_opt);

        interpreter.lookaround_clock_array_allocator_ = Some(Vec::new());
        interpreter.lookaround_match_index_array_allocator_ = Some(Vec::new());

        interpreter.lookaround_table_ = Some((0..interpreter.lookarounds_.len()).map(|_| vec![false; input_.len() + 1]).collect());
    }

    // Precomputes the memory consumption of a single thread, to be used by
    // `CheckMemoryConsumption()`.
    //if v8_flags.experimental_regexp_engine_capture_group_opt {
    interpreter.quantifier_array_allocator_ = Some(Vec::new());
    interpreter.capture_clock_array_allocator_ = Some(Vec::new());

    interpreter.memory_consumption_per_thread_ =
            register_count_per_match * std::mem::size_of::<i32>() +  // RegisterArray
            interpreter.quantifier_count_ * std::mem::size_of::<u64>() +     // QuantifierClockArray
            register_count_per_match * std::mem::size_of::<u64>() +  // CaptureClockArray
            interpreter.lookarounds_.len() * std::mem::size_of::<u64>() +  // LookaroundClockArray
            interpreter.lookarounds_.len() * std::mem::size_of::<i32>() +  // LookaroundMatchIndexArray
            std::mem::size_of::<InterpreterThread>();
    //}

    interpreter
  }

  fn find_matches(
        &mut self,
        output_registers: &mut [i32],
        output_register_count: usize,
    ) -> Result<usize, InternalRegExpResult> {
        let max_match_num = output_register_count / self.register_count_per_match_;

        if !self.only_captureless_lookbehinds_ {
            let err_code = self.fill_lookaround_table()?;
            if err_code != InternalRegExpResult::Success {
                return Err(err_code);
            }
        }

        let mut match_num = 0;
        while match_num != max_match_num {
            let err_code = self.find_next_match()?;
            if err_code != InternalRegExpResult::Success {
                return Err(err_code);
            }

            if !self.found_match() {
                break;
            }

            let best_match_thread = self.best_match_thread_.unwrap();

            let mut registers = Vec::new();

            let err_code = self.get_filtered_registers(best_match_thread, &mut registers)?;
            if err_code != InternalRegExpResult::Success {
                return Err(err_code);
            }

            let registers_slice = registers.as_slice();
            for (i, &val) in registers_slice.iter().enumerate() {
                output_registers[match_num * self.register_count_per_match_ + i] = val;
            }
            //output_registers =
            //    std::copy(registers.begin(), registers.end(), output_registers);

            match_num += 1;

            let match_begin = registers[0] as usize;
            let match_end = registers[1] as usize;
            assert!(match_begin <= match_end);
            let match_length = match_end - match_begin;
            if match_length != 0 {
                self.set_input_index(match_end);
            } else if match_end == self.input_.len() {
                // Zero-length match, input exhausted.
                self.set_input_index(match_end);
                break;
            } else {
                // Zero-length match, more input.  We don't want to report more matches
                // here endlessly, so we advance by 1.
                self.set_input_index(match_end + 1);

                // TODO(mbid,v8:10765): If we're in unicode mode, we have to advance to
                // the next codepoint, not to the next code unit. See also
                // `RegExpUtils::AdvanceStringIndex`.
                //static_assert(!ExperimentalRegExp::kSupportsUnicode);
            }
        }

        Ok(match_num)
    }

    fn fill_lookaround_table(&mut self) -> Result<InternalRegExpResult, InternalRegExpResult> {
      if self.lookarounds_.is_empty() {
          return Ok(InternalRegExpResult::Success);
      }

      self.pc_last_input_index_.iter_mut().for_each(|x| *x = LastInputIndex::new(-1, -1));

      let old_input_index = self.input_index_;

      for i in (0..self.lookarounds_.len()).rev() {
          // Clean up left-over data from last iteration.
          self.blocked_threads_.clear();

          self.active_threads_.clear();

          self.current_lookaround_ = i as i32;
          self.reverse_ = self.lookarounds_[i].type_ == RegExpLookaroundType::LOOKAHEAD;
          self.input_index_ = if self.reverse_ {
              self.input_.len()
          } else {
              0
          };

          self.active_threads_.push(self.new_empty_thread(self.lookarounds_[i].match_pc as usize));

          let err_code = self.run_active_threads_to_end()?;
          if err_code != InternalRegExpResult::Success {
              return Ok(err_code);
          }
      }

      self.reverse_ = false;
      self.current_lookaround_ = -1;
      self.input_index_ = old_input_index;

      Ok(InternalRegExpResult::Success)
    }

    fn fill_lookaround_captures(&mut self, main_thread: InterpreterThread) -> Result<InternalRegExpResult, InternalRegExpResult> {
      assert!(self.best_match_thread_.is_some());

      if self.lookarounds_.is_empty() {
          return Ok(InternalRegExpResult::Success);
      }

      // We need to capture the lookarounds from parents to childrens, since we
      // need the index on which the lookaround was matched, and those indexes are
      // computed when the parent expression is captured.
      for i in 0..self.lookarounds_.len() {
          let lookaround_match_index_array = match &self.lookaround_match_index_array_allocator_ {
            Some(arr) => arr,
            None => panic!("No lookaround_match_index_array_allocator_"),
          };

          let lookaround_match_index = if lookaround_match_index_array.is_empty() {
              K_UNDEFINED_MATCH_INDEX_VALUE
          } else {
              K_UNDEFINED_MATCH_INDEX_VALUE
          };

          if lookaround_match_index == K_UNDEFINED_MATCH_INDEX_VALUE {
              continue;
          }

          let lookaround = self.lookarounds_[i];

          self.pc_last_input_index_.iter_mut().for_each(|x| *x = LastInputIndex::new(-1, -1));

          // Clean up left-over data from last iteration.
          self.blocked_threads_.clear();

          self.active_threads_.clear();

          self.best_match_thread_ = None;

          self.reverse_ = lookaround.type_ == RegExpLookaroundType::LOOKBEHIND;
          self.input_index_ = lookaround_match_index as usize;

          // We reuse the same thread as initial thread, to avoid having to merge
          // the new `best_match_thread_` with the previous results.

          let mut mutable_main_thread = main_thread;
          mutable_main_thread.pc = lookaround.capture_pc as usize;
          mutable_main_thread.consumed_since_last_quantifier = ConsumedCharacter::DidConsume;
          self.active_threads_.push(mutable_main_thread);

          let err_code = self.run_active_threads_to_end()?;
          if err_code != InternalRegExpResult::Success {
              return Ok(err_code);
          }

          // The lookaround has already been matched once on this position during
          // the match research.
          assert!(self.best_match_thread_.is_some());
      }

      Ok(InternalRegExpResult::Success)
    }

    fn run_active_threads_to_end(&mut self) -> Result<InternalRegExpResult, InternalRegExpResult> {
      // Run the initial thread, potentially forking new threads, until every
      // thread is blocked without further input.
      self.run_active_threads()?;

      // We stop if one of the following conditions hold:
      // - We have exhausted the entire input.
      // - We have found a match at some point, and there are no remaining
      //   threads with higher priority than the thread that produced the match.
      //   Threads with low priority have been aborted earlier, and the remaining
      //   threads are blocked here, so the latter simply means that
      //   `blocked_threads_` is empty.
      while (if self.reverse_ {
          0 < self.input_index_ && self.input_index_ <= self.input_.len()
      } else {
          0 <= self.input_index_ && self.input_index_ < self.input_.len()
      }) && !(self.found_match() && self.blocked_threads_.is_empty())
      {
          assert!(self.active_threads_.is_empty());

          if self.lookbehind_table_.is_some() {
              if let Some(ref mut lookbehind_table) = self.lookbehind_table_ {
                lookbehind_table.iter_mut().for_each(|x| *x = false);
              }
          }

          if self.reverse_ {
              self.input_index_ -= 1;
          }

          let input_char = self.input_[self.input_index_];

          if !self.reverse_ {
              self.input_index_ += 1;
          }

          //static constexpr int kTicksBetweenInterruptHandling = 64;
          //if (input_index_ % kTicksBetweenInterruptHandling == 0) {
          //    let err_code = self.handle_interrupts()?;
          //    if err_code != InternalRegExpResult::Success {
          //        return Ok(err_code);
          //    }
          //}

          // We unblock all blocked_threads_ by feeding them the input char.
          self.flush_blocked_threads(input_char);

          // Run all threads until they block or accept.
          self.run_active_threads()?;
      }

      Ok(InternalRegExpResult::Success)
    }

    //fn handle_interrupts(&self) -> Result<InternalRegExpResult, InternalRegExpResult> {
    //    Ok(InternalRegExpResult::Success) // Placeholder
    //}

    fn set_input_index(&mut self, new_input_index: usize) {
        assert!(new_input_index >= 0);
        assert!(new_input_index <= self.input_.len());

        self.input_index_ = new_input_index;
    }

    fn find_next_match(&mut self) -> Result<InternalRegExpResult, InternalRegExpResult> {
        assert!(self.active_threads_.is_empty());
        // TODO(mbid,v8:10765): Can we get around resetting `pc_last_input_index_`
        // here? As long as
        //
        //   pc_last_input_index_[pc] < input_index_
        //
        // for all possible program counters pc that are reachable without input
        // from pc = 0 and
        //
        //   pc_last_input_index_[k] <= input_index_
        //
        // for all k > 0 hold I think everything should be fine.  Maybe we can do
        // something about this in `SetInputIndex`.
        self.pc_last_input_index_.iter_mut().for_each(|x| *x = LastInputIndex::new(-1, -1));

        // Clean up left-over data from a previous call to FindNextMatch.
        self.blocked_threads_.clear();

        self.active_threads_.clear();

        if self.best_match_thread_.is_some() {
            self.best_match_thread_ = None;
        }

        self.active_threads_.push(self.new_empty_thread(0));

        if self.only_captureless_lookbehinds_ {
            for i in 0..self.lookarounds_.len() {
                self.active_threads_.push(self.new_empty_thread(self.lookarounds_[i].match_pc as usize));
            }
        }

        let err_code = self.run_active_threads_to_end()?;
        if err_code != InternalRegExpResult::Success {
            return Ok(err_code);
        }

        Ok(InternalRegExpResult::Success)
    }

    fn run_active_thread(&mut self, t: InterpreterThread) -> Result<InternalRegExpResult, InternalRegExpResult> {
      let mut mut_t = t;
      loop {
          assert!(mut_t.pc >= 0);
          assert!(mut_t.pc < self.bytecode_.len());

          self.clock += 1;

          assert!(self.clock > 0);

          if self.is_pc_processed(mut_t.pc, mut_t.consumed_since_last_quantifier) {
              return Ok(InternalRegExpResult::Success);
          }
          self.mark_pc_processed(mut_t.pc, mut_t.consumed_since_last_quantifier);

          let inst = self.bytecode_[mut_t.pc];

          match inst.opcode {
              RegExpInstructionOp