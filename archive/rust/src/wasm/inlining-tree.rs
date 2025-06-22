// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header should only be included if WebAssembly is enabled.
// #[cfg(not(feature = "webassembly"))]
// compile_error!("This module should only be included if WebAssembly is enabled.");

use std::cmp::{max, min};
use std::collections::{BinaryHeap, HashMap};
use std::sync::Mutex;

// Mock zone allocator.  In a real implementation, this would use a zone.
type ZoneAllocator<T> = Vec<T>;

// Mock Zone trait.
trait ZoneAllocatable<T> {
    fn new_in_zone(zone: &mut ZoneAllocator<T>, value: T) -> &mut T;
    fn allocate_vector_in_zone<U>(zone: &mut ZoneAllocator<Vec<U>>, size: usize) -> &mut Vec<U>;
}

impl<T> ZoneAllocatable<T> for T {
    fn new_in_zone(zone: &mut ZoneAllocator<T>, value: T) -> &mut T {
        zone.push(value);
        zone.last_mut().unwrap()
    }

    fn allocate_vector_in_zone<U>(zone: &mut ZoneAllocator<Vec<U>>, size: usize) -> &mut Vec<U> {
        zone.push(vec![U::default(); size]);
        zone.last_mut().unwrap()
    }
}

// Mock implementation for WasmModule and related structs
#[derive(Default, Clone)]
pub struct WasmModule {
    pub functions: Vec<WasmFunction>,
    pub num_small_functions: usize,
    pub num_declared_functions: usize,
    pub num_imported_functions: u32,
    pub type_feedback: TypeFeedbackContainer,
}

#[derive(Default, Clone)]
pub struct WasmFunction {
    pub code: WasmCode,
}

#[derive(Default, Clone)]
pub struct WasmCode {
    pub length: usize,
}

#[derive(Default, Clone)]
pub struct TypeFeedbackContainer {
    pub feedback_for_function: HashMap<u32, FunctionTypeFeedback>,
    pub mutex: Mutex<()>, // Mock Mutex
}

#[derive(Default, Clone)]
pub struct FunctionTypeFeedback {
    pub feedback_vector: CallSiteFeedbackVector,
    pub call_targets: Vec<u32>,
}

#[derive(Default, Clone)]
pub struct CallSiteFeedbackVector {
    pub vec: Vec<CallSiteFeedback>,
}

impl CallSiteFeedbackVector {
    pub fn as_vector(&self) -> Vec<CallSiteFeedback> {
        self.vec.clone()
    }
}

#[derive(Default, Clone)]
pub struct CallSiteFeedback {
    num_cases: u32,
    has_non_inlineable_targets: bool,
    call_count_per_case: Vec<u32>,
    function_index_per_case: Vec<u32>,
}

impl CallSiteFeedback {
    pub fn num_cases(&self) -> usize {
        self.num_cases as usize
    }
    pub fn has_non_inlineable_targets(&self) -> bool {
        self.has_non_inlineable_targets
    }
    pub fn call_count(&self, the_case: usize) -> i32 {
        self.call_count_per_case[the_case] as i32
    }
    pub fn function_index(&self, the_case: usize) -> u32 {
        self.function_index_per_case[the_case]
    }
}

// Mock implementation of is_asmjs_module function
pub fn is_asmjs_module(_module: &WasmModule) -> bool {
    false
}

// Mock v8_flags.
pub struct V8Flags {
    pub wasm_inlining_factor: i32,
    pub wasm_inlining_min_budget: usize,
    pub wasm_inlining_budget: f64,
    pub wasm_inlining_max_size: usize,
    pub wasm_inlining_ignore_call_counts: bool,
    pub trace_wasm_inlining: bool,
}

impl Default for V8Flags {
    fn default() -> Self {
        V8Flags {
            wasm_inlining_factor: 5,
            wasm_inlining_min_budget: 100,
            wasm_inlining_budget: 1000.0,
            wasm_inlining_max_size: 1000,
            wasm_inlining_ignore_call_counts: false,
            trace_wasm_inlining: false,
        }
    }
}

pub static mut v8_flags: V8Flags = V8Flags::default();

/// Represents a tree of inlining decisions.
/// A node in the tree represents a function frame, and `function_calls`
/// represent all direct/call_ref/call_indirect function calls in this frame.
/// Each element of `function_calls` is itself a `Vec` of `InliningTree`s,
/// corresponding to the different speculative candidates for a
/// call_ref/call_indirect; for a direct call, it has a single element.
/// If a transitive element of `function_calls` has its `is_inlined` field set,
/// it should be inlined into the caller.
/// We have this additional data structure for Turboshaft, since nodes in the
/// Turboshaft IR aren't easily expanded incrementally, so all the inlining
/// decisions are already made before graph building on this abstracted form of
/// the code.
#[derive(Debug)]
pub struct InliningTree {
    data: *mut Data, // Raw pointer to shared data.
    function_index: u32,
    call_count: i32,
    wire_byte_size: i32,
    is_inlined: bool,
    feedback_found: bool,
    function_calls: Vec<CasesPerCallSite>,
    has_non_inlineable_targets: Vec<bool>,
    depth: u32,
    caller_index: u32,
    feedback_slot: i32,
    case_num: i32,
}

type CasesPerCallSite = Vec<*mut InliningTree>;

#[derive(Debug)]
struct Data {
    zone: *mut ZoneAllocator<InliningTree>, //Raw pointer to the zone
    module: *const WasmModule, // Raw pointer to the wasm module
    max_growth_factor: f64,
    budget_cap: usize,
    topmost_caller_index: u32,
}

impl Data {
    fn new(
        zone: *mut ZoneAllocator<InliningTree>,
        module: *const WasmModule,
        topmost_caller_index: u32,
    ) -> Data {
        unsafe {
            let scaled = BudgetScaleFactor(&(*module));
            let k_turboshaft_adjustment = 2;
            let high_growth = v8_flags.wasm_inlining_factor + k_turboshaft_adjustment;
            let k_lowest_useful_value = 2;
            let low_growth = max(k_lowest_useful_value, high_growth - 3);
            let max_growth_factor = low_growth as f64 * (1.0 - scaled) + high_growth as f64 * scaled;
            let k_turboshaft_correction_factor = 1.4;
            let high_cap = v8_flags.wasm_inlining_budget * k_turboshaft_correction_factor;
            let low_cap = high_cap / 10.0;
            let budget_cap = low_cap * (1.0 - scaled) + high_cap * scaled;

            Data {
                zone: zone,
                module: module,
                max_growth_factor,
                budget_cap: budget_cap as usize,
                topmost_caller_index,
            }
        }
    }
}

impl InliningTree {
    /// Maximum number of inlined functions.
    pub const K_MAX_INLINED_COUNT: usize = 60;

    /// Limit the nesting depth of inlining. Inlining decisions are based on call
    /// counts. A small function with high call counts that is called recursively
    /// would be inlined until all budget is used.
    /// TODO(14108): This still might not lead to ideal results. Other options
    /// could be explored like penalizing nested inlinees.
    pub const K_MAX_INLINING_NESTING_DEPTH: u32 = 7;

    /// Creates the root of the inlining tree.
    pub fn create_root(
        zone: &mut ZoneAllocator<InliningTree>,
        module: &WasmModule,
        function_index: u32,
    ) -> *mut InliningTree {
        unsafe {
            let data = Data::new_in_zone(zone,Data::new(zone, module, function_index));
            let tree = InliningTree::new_in_zone(
                zone,
                InliningTree {
                    data: data,
                    function_index,
                    call_count: 0,
                    wire_byte_size: 0, // `0` causes the root node to always get expanded
                                        // regardless of budget.
                    is_inlined: false,
                    feedback_found: false,
                    function_calls: Vec::new(),
                    has_non_inlineable_targets: Vec::new(),
                    depth: 0,
                    caller_index: u32::MAX,
                    feedback_slot: -1,
                    case_num: -1,
                },
            );
            (*tree).fully_expand(zone);
            tree
        }
    }

    /// This should stay roughly in sync with the full logic below, but not rely
    /// on having observed any call counts. Since it therefore can't simulate
    /// regular behavior accurately anyway, it may be a very coarse approximation.
    pub fn no_liftoff_budget(module: &WasmModule, func_index: u32) -> i32 {
        let wirebytes = module.functions[func_index].code.length;
        let scaled = BudgetScaleFactor(module);
        let k_turboshaft_adjustment = 2;
        unsafe {
            let high_growth = v8_flags.wasm_inlining_factor + k_turboshaft_adjustment;
            let k_lowest_useful_value = 2;
            let low_growth = max(k_lowest_useful_value, high_growth - 3);
            let max_growth_factor = low_growth as f64 * (1.0 - scaled) + high_growth as f64 * scaled;
            max(
                v8_flags.wasm_inlining_min_budget as i32,
                (max_growth_factor * wirebytes as f64) as i32,
            )
        }
    }

    /// Calculates the score of the inlining tree.
    pub fn score(&self) -> i64 {
        let count_factor: i64 = 2;
        let size_factor: i64 = 3;
        self.call_count as i64 * count_factor - self.wire_byte_size as i64 * size_factor
    }

    /// Returns the function calls of the inlining tree.
    pub fn function_calls(&self) -> &Vec<CasesPerCallSite> {
        &self.function_calls
    }

    /// Returns the non-inlineable targets of the inlining tree.
    pub fn has_non_inlineable_targets(&self) -> &Vec<bool> {
        &self.has_non_inlineable_targets
    }

    /// Returns whether feedback was found for this node.
    pub fn feedback_found(&self) -> bool {
        self.feedback_found
    }

    /// Returns whether the function is inlined.
    pub fn is_inlined(&self) -> bool {
        self.is_inlined
    }

    /// Returns the function index of the inlining tree.
    pub fn function_index(&self) -> u32 {
        self.function_index
    }
}

impl InliningTree {
    fn new_in_zone(zone: &mut ZoneAllocator<InliningTree>, value: InliningTree) -> *mut InliningTree {
        zone.push(value);
        zone.as_mut_ptr().add(zone.len() - 1)
    }
}

impl InliningTree {

    fn budget_scale_factor(module: &WasmModule) -> f64 {
        BudgetScaleFactor(module)
    }

    /// Recursively expand the tree by expanding this node and children nodes etc.
    /// Nodes are prioritized by their `score`. Expansion continues until
    /// `K_MAX_INLINED_COUNT` nodes are expanded or `budget` (in wire-bytes size) is
    /// depleted.
    fn fully_expand(&mut self, zone: &mut ZoneAllocator<InliningTree>) {
        unsafe {
            assert_eq!(self.function_index, (*self.data).topmost_caller_index);
            let initial_wire_byte_size =
                (*(*self.data).module).functions[self.function_index as usize].code.length as usize;
            let mut inlined_wire_byte_count: usize = 0;
            let mut queue: BinaryHeap<InliningTreeWrapper> = BinaryHeap::new();
            queue.push(InliningTreeWrapper(self));
            let mut inlined_count = 0;
            let _mutex_guard = (*(*self.data).module).type_feedback.mutex.lock().unwrap(); //Mock mutex
            while !queue.is_empty() && inlined_count < InliningTree::K_MAX_INLINED_COUNT {
                let InliningTreeWrapper(top) = queue.pop().unwrap();
                if v8_flags.trace_wasm_inlining {
                    if top as *const _ != self as *const _ {
                        println!(
                            "[function {}: in function {}, considering call #{}, case #{}, to function {} (count={}, size={}, score={})... ]",
                            (*self.data).topmost_caller_index,
                            top.caller_index,
                            top.feedback_slot,
                            top.case_num,
                            top.function_index,
                            top.call_count,
                            top.wire_byte_size,
                            top.score()
                        );
                    } else {
                        println!(
                            "[function {}: expanding topmost caller... ]",
                            (*self.data).topmost_caller_index
                        );
                    }
                }

                if top.function_index < (*(*self.data).module).num_imported_functions {
                    if v8_flags.trace_wasm_inlining && top as *const _ != self as *const _ {
                        println!("imported function]");
                    }
                    continue;
                }

                if is_asmjs_module(&(*(*self.data).module)) {
                    if v8_flags.trace_wasm_inlining {
                        println!("cannot inline asm.js function]");
                    }
                    continue;
                }

                // Key idea: inlining hot calls is good, inlining big functions is bad,
                // so inline when a candidate is "hotter than it is big". Exception:
                // tiny candidates can get inlined regardless of their call count.
                if top as *const _ != self as *const _ && top.wire_byte_size >= 12
                    && !v8_flags.wasm_inlining_ignore_call_counts
                {
                    if top.call_count < top.wire_byte_size / 2 {
                        if v8_flags.trace_wasm_inlining {
                            println!("not called often enough]");
                        }
                        continue;
                    }
                }

                if !top.small_enough_to_inline(initial_wire_byte_size, inlined_wire_byte_count) {
                    if v8_flags.trace_wasm_inlining && top as *const _ != self as *const _ {
                        println!("not enough inlining budget]");
                    }
                    continue;
                }

                if v8_flags.trace_wasm_inlining && top as *const _ != self as *const _ {
                    println!("decided to inline! ");
                }

                top.inline_func(zone);
                inlined_count += 1;

                // For tiny functions, inlining may actually decrease generated code size
                // because we have one less call and don't need to push arguments, etc.
                // Subtract a little bit from the code size increase, such that inlining
                // these tiny functions doesn't use up any of the budget.
                let k_one_less_call = 6; // Guesstimated savings per call.
                inlined_wire_byte_count += max(top.wire_byte_size as usize, k_one_less_call) - k_one_less_call;

                if !top.feedback_found() {
                    if v8_flags.trace_wasm_inlining {
                        println!("no feedback yet or no callees]");
                    }
                } else if top.depth < InliningTree::K_MAX_INLINING_NESTING_DEPTH {
                    if v8_flags.trace_wasm_inlining {
                        println!("queueing {} callee(s)]", top.function_calls.len());
                    }
                    for cases in &top.function_calls {
                        for call in cases {
                            if !call.is_null() {
                                queue.push(InliningTreeWrapper(*call));
                            }
                        }
                    }
                } else if v8_flags.trace_wasm_inlining {
                    println!("max inlining depth reached]");
                }
            }

            if v8_flags.trace_wasm_inlining && !queue.is_empty() {
                println!(
                    "[function {}: too many inlining candidates, stopping...]",
                    (*self.data).topmost_caller_index
                );
            }
        }
    }

    /// Mark this function call as inline and initialize `function_calls` based
    /// on the `module.type_feedback`.
    fn inline_func(&mut self, zone: &mut ZoneAllocator<InliningTree>) {
        self.is_inlined = true;
        unsafe {
            let feedback_map = &(*(*self.data).module).type_feedback.feedback_for_function;
            let feedback_it = feedback_map.get(&self.function_index);
            if feedback_it.is_none() {
                return;
            }

            let feedback = feedback_it.unwrap();
            let type_feedback = feedback.feedback_vector.as_vector();
            if type_feedback.is_empty() {
                return; // No feedback yet.
            }
            assert_eq!(type_feedback.len(), feedback.call_targets.len());
            self.feedback_found = true;
            self.function_calls = vec![Vec::new(); type_feedback.len()];
            self.has_non_inlineable_targets = vec![false; type_feedback.len()];

            for i in 0..type_feedback.len() {
                self.function_calls[i] = vec![std::ptr::null_mut(); type_feedback[i].num_cases()];
                self.has_non_inlineable_targets[i] = type_feedback[i].has_non_inlineable_targets();

                for the_case in 0..type_feedback[i].num_cases() {
                    let callee_index = type_feedback[i].function_index(the_case);
                    let call_count = type_feedback[i].call_count(the_case);
                    let callee_code_length = (*(*self.data).module).functions[callee_index as usize].code.length;
                    let inlining_tree = InliningTree::new_in_zone(
                        zone,
                        InliningTree {
                            data: self.data,
                            function_index: callee_index,
                            call_count: call_count,
                            wire_byte_size: callee_code_length as i32,
                            is_inlined: false,
                            feedback_found: false,
                            function_calls: Vec::new(),
                            has_non_inlineable_targets: Vec::new(),
                            depth: self.depth + 1,
                            caller_index: self.function_index,
                            feedback_slot: i as i32,
                            case_num: the_case as i32,
                        },
                    );
                    self.function_calls[i][the_case] = inlining_tree;
                }
            }
        }
    }

    /// Returns true if there is still enough budget left to inline the current
    /// candidate given the initial graph size and the already inlined wire bytes.
    fn small_enough_to_inline(&self, initial_wire_byte_size: usize, inlined_wire_byte_count: usize) -> bool {
        unsafe {
            if self.wire_byte_size as usize > v8_flags.wasm_inlining_max_size {
                return false;
            }

            // For tiny functions, let's be a bit more generous.
            if self.wire_byte_size < 12 {
                let mut local_inlined_wire_byte_count = inlined_wire_byte_count;
                if local_inlined_wire_byte_count > 100 {
                    local_inlined_wire_byte_count -= 100;
                } else {
                    local_inlined_wire_byte_count = 0;
                }
            }
            let budget_small_function = max(
                v8_flags.wasm_inlining_min_budget,
                (*self.data).max_growth_factor * initial_wire_byte_size as f64,
            );

            let budget_large_function = max(
                (*self.data).budget_cap,
                initial_wire_byte_size as f64 * 1.1,
            );

            let total_size = initial_wire_byte_size + inlined_wire_byte_count + self.wire_byte_size as usize;

            if v8_flags.trace_wasm_inlining {
                println!(
                    "budget=min({}, {}), size {}->{} ",
                    budget_small_function,
                    budget_large_function,
                    (initial_wire_byte_size + inlined_wire_byte_count),
                    total_size
                );
            }

            total_size < min(budget_small_function as usize, budget_large_function as usize)
        }
    }
}

/// Helper struct to implement `PartialOrd` for `InliningTree*`.
#[derive(PartialEq, Eq)]
struct InliningTreeWrapper<'a>(*mut InliningTree);

impl<'a> Ord for InliningTreeWrapper<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            let t1 = self.0.as_ref().unwrap();
            let t2 = other.0.as_ref().unwrap();
            // Prefer callees with a higher score, and if the scores are equal,
            // those with a lower function index (to make the queue ordering strict).
            (t2.score(), t1.function_index()).cmp(&(t1.score(), t2.function_index()))
        }
    }
}

impl<'a> PartialOrd for InliningTreeWrapper<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Calculates budget scale factor.
fn BudgetScaleFactor(module: &WasmModule) -> f64 {
    let small_function_percentage =
        module.num_small_functions as f64 * 100.0 / module.num_declared_functions as f64;

    if small_function_percentage <= 25.0 {
        0.0
    } else if small_function_percentage >= 50.0 {
        1.0
    } else {
        (small_function_percentage - 25.0) / 25.0
    }
}