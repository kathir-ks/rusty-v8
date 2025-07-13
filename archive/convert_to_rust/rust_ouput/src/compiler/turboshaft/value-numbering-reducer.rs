// Converted from V8 C++ source files:
// Header: value-numbering-reducer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod turboshaft {
use std::cell::RefCell;
use std::rc::Rc;

use crate::base::logging;
use crate::compiler::turboshaft::assembler::Assembler;
use crate::compiler::turboshaft::fast_hash::fast_hash_combine;
use crate::compiler::turboshaft::graph::{
    Block, BlockIndex, Opcode, OpIndex, Operation, OperationBuffer,
};
use crate::compiler::turboshaft::operations::{
    DeoptimizeIfOp, MayThrow, OperationTrait, PendingLoopPhiOp, PhiOp,
};
use crate::compiler::turboshaft::reducer_traits::next_reducer_is;
use crate::compiler::turboshaft::types::{Type, ValueType, Zone};
use crate::utils::utils;
use crate::zone::zone_containers::ZoneVector;
use crate::v8::internal::compiler::turboshaft::TypeInferenceReducer;

pub struct ScopeCounter {
    scopes_: RefCell<i32>,
}

impl ScopeCounter {
    pub fn new() -> Self {
        ScopeCounter {
            scopes_: RefCell::new(0),
        }
    }
    pub fn enter(&self) {
        *self.scopes_.borrow_mut() += 1;
    }
    pub fn leave(&self) {
        *self.scopes_.borrow_mut() -= 1;
    }
    pub fn is_active(&self) -> bool {
        *self.scopes_.borrow() > 0
    }
}

pub struct DisableValueNumbering<'a, Reducer> {
    scopes_: Option<&'a ScopeCounter>,
    reducer: &'a Reducer,
}

impl<'a, Reducer> DisableValueNumbering<'a, Reducer> {
    pub fn new(reducer: &'a Reducer) -> Self {
        if std::any::TypeId::of::<Reducer>() == std::any::TypeId::of::<ValueNumberingReducer<()>>() {
            // Assuming Reducer has a method gvn_disabled_scope()
            let scopes_ = Some(reducer.gvn_disabled_scope());
            scopes_.unwrap().enter();
            DisableValueNumbering { scopes_, reducer }
        } else {
            DisableValueNumbering { scopes_: None, reducer }
        }
    }
}

impl<'a, Reducer> Drop for DisableValueNumbering<'a, Reducer> {
    fn drop(&mut self) {
        if let Some(scopes_) = self.scopes_ {
            scopes_.leave();
        }
    }
}

pub trait ValueNumberingReducerNext {
    fn bind(&mut self, block: *mut Block);
    fn remove_last(&mut self, index: OpIndex);
    fn current_block_index(&self) -> BlockIndex;
}

pub struct ValueNumberingReducer<Next: ValueNumberingReducerNext> {
    next: Next,
    dominator_path_: RefCell<Vec<*mut Block>>,
    table_: RefCell<Vec<Entry>>,
    mask_: RefCell<usize>,
    entry_count_: RefCell<usize>,
    depths_heads_: RefCell<Vec<*mut Entry>>,
    disabled_scope_: ScopeCounter,
    phase_zone: *mut Zone,
    output_graph: RefCell<OperationBuffer>,
    current_block: RefCell<*mut Block>,
}

impl<Next: ValueNumberingReducerNext> ValueNumberingReducer<Next> {
    pub fn new(next: Next, zone: *mut Zone, operation_buffer: OperationBuffer) -> Self {
        let initial_capacity = std::cmp::max(128, operation_buffer.op_id_capacity() / 2);
        let initial_capacity_rounded = initial_capacity.next_power_of_two();
        let table = (0..initial_capacity_rounded)
            .map(|_| Entry {
                value: OpIndex { index: 0 },
                block: BlockIndex { index: 0 },
                hash: 0,
                depth_neighboring_entry: std::ptr::null_mut(),
            })
            .collect();

        ValueNumberingReducer {
            next,
            dominator_path_: RefCell::new(Vec::new()),
            table_: RefCell::new(table),
            mask_: RefCell::new(initial_capacity_rounded - 1),
            entry_count_: RefCell::new(0),
            depths_heads_: RefCell::new(Vec::new()),
            disabled_scope_: ScopeCounter::new(),
            phase_zone: zone,
            output_graph: RefCell::new(operation_buffer),
            current_block: RefCell::new(std::ptr::null_mut()),
        }
    }

    pub fn bind(&self, block: *mut Block) {
        self.next.bind(block);
        self.reset_to_block(block);
        self.dominator_path_.borrow_mut().push(block);
        self.depths_heads_.borrow_mut().push(std::ptr::null_mut());
    }

    fn reset_to_block(&self, block: *mut Block) {
        unsafe {
            let mut target = (*block).GetDominator();
            while !self.dominator_path_.borrow().is_empty()
                && target != std::ptr::null_mut()
                && self.dominator_path_.borrow().last().copied().unwrap() != target
            {
                if (*(self.dominator_path_.borrow().last().copied().unwrap())).Depth()
                    > (*target).Depth()
                {
                    self.clear_current_depth_entries();
                } else if (*(self.dominator_path_.borrow().last().copied().unwrap())).Depth()
                    < (*target).Depth()
                {
                    target = (*target).GetDominator();
                } else {
                    self.clear_current_depth_entries();
                    target = (*target).GetDominator();
                }
            }
        }
    }

    pub fn will_gvn_op<Op: OperationTrait>(&self, op: &Op) -> bool {
        let entry = self.find(op);
        !entry.is_empty()
    }

    pub fn gvn_disabled_scope(&self) -> &ScopeCounter {
        &self.disabled_scope_
    }

    struct Entry {
        value: OpIndex,
        block: BlockIndex,
        hash: usize,
        depth_neighboring_entry: *mut Entry,
    }

    impl Entry {
        fn is_empty(&self) -> bool {
            self.hash == 0
        }
    }

    fn add_or_find<Op: OperationTrait>(&self, op_idx: OpIndex) -> OpIndex {
        if self.is_disabled() {
            return op_idx;
        }

        let op = self.output_graph.borrow().get(op_idx).cast::<Op>();
        if std::any::TypeId::of::<Op>() == std::any::TypeId::of::<PendingLoopPhiOp>() || op.is_block_terminator()
            || (!op.effects().repetition_is_eliminatable() && std::any::TypeId::of::<Op>() != std::any::TypeId::of::<DeoptimizeIfOp>())
        {
            return op_idx;
        }
        self.rehash_if_needed();

        let mut hash = 0;
        let entry = self.find_internal::<Op>(&op, &mut hash);

        if entry.is_empty() {
            unsafe {
                let mut new_entry = Entry {
                    value: op_idx,
                    block: (*self.current_block.borrow()).index(),
                    hash,
                    depth_neighboring_entry: *self.depths_heads_.borrow().last().unwrap_unchecked(),
                };

                *self.depths_heads_.borrow_mut().last_mut().unwrap_unchecked() = &mut new_entry;

                *self.entry_count_.borrow_mut() += 1;
                return op_idx;
            }
        } else {
            self.next.remove_last(op_idx);
            return entry.value;
        }
    }

    fn find<Op: OperationTrait>(&self, op: &Op) -> &Entry {
        let mut hash = 0;
        self.find_internal::<Op>(op, &mut hash)
    }

    fn find_internal<Op: OperationTrait>(&self, op: &Op, hash_ret: &mut usize) -> &Entry {
        let same_block_only = std::any::TypeId::of::<Op>() == std::any::TypeId::of::<PhiOp>();

        let hash = self.compute_hash::<same_block_only, Op>(op);
        *hash_ret = hash;
        let start_index = hash & *self.mask_.borrow();

        for i in start_index.. {
            let i = i % self.table_.borrow().len();
            let entry = &self.table_.borrow()[i];

            if entry.is_empty() {
                return entry;
            }

            if entry.hash == hash {
                let entry_op = self.output_graph.borrow().get(entry.value);

                if entry_op.is::<Op>()
                    && (!same_block_only || unsafe { entry.block == (*self.current_block.borrow()).index() })
                    && entry_op.cast::<Op>().equals_for_gvn(op)
                {
                    return entry;
                }
            }

            if i == (start_index - 1) % self.table_.borrow().len() {
                break;
            }
        }

        &self.table_.borrow()[0]
    }

    fn clear_current_depth_entries(&self) {
        unsafe {
            let mut entry = *self.depths_heads_.borrow().last().unwrap_unchecked();

            while entry != std::ptr::null_mut() {
                (*entry).hash = 0;
                let next_entry = (*entry).depth_neighboring_entry;
                (*entry).depth_neighboring_entry = std::ptr::null_mut();
                entry = next_entry;
                *self.entry_count_.borrow_mut() -= 1;
            }
            self.depths_heads_.borrow_mut().pop();
            self.dominator_path_.borrow_mut().pop();
        }
    }

    fn rehash_if_needed(&self) {
        if *self.table_.borrow().len() - (*self.table_.borrow().len() / 4) > *self.entry_count_.borrow() {
            return;
        }

        let mut new_table: Vec<Entry> = (0..self.table_.borrow().len() * 2)
            .map(|_| Entry {
                value: OpIndex { index: 0 },
                block: BlockIndex { index: 0 },
                hash: 0,
                depth_neighboring_entry: std::ptr::null_mut(),
            })
            .collect();

        let mask = self.mask_.borrow().clone();
        *self.mask_.borrow_mut() = new_table.len() - 1;

        for depth_idx in 0..self.depths_heads_.borrow().len() {
            let mut entry = *self.depths_heads_.borrow()[depth_idx];
            *self.depths_heads_.borrow_mut().get_mut(depth_idx).unwrap() = std::ptr::null_mut();

            while entry != std::ptr::null_mut() {
                unsafe {
                    for i in ((*entry).hash & mask).. {
                        let i = i % new_table.len();
                        if new_table[i].hash == 0 {
                            new_table[i] = *entry;
                            let next_entry = (*entry).depth_neighboring_entry;
                            new_table[i].depth_neighboring_entry =
                                *self.depths_heads_.borrow()[depth_idx];
                            *self.depths_heads_.borrow_mut().get_mut(depth_idx).unwrap() =
                                &mut new_table[i];
                            entry = next_entry;
                            break;
                        }
                    }
                }
            }
        }
        *self.table_.borrow_mut() = new_table;
    }

    fn compute_hash<const SAME_BLOCK_ONLY: bool, Op: OperationTrait>(&self, op: &Op) -> usize {
        let mut hash = op.hash_value();
        if SAME_BLOCK_ONLY {
            unsafe {
                hash = fast_hash_combine((*self.current_block.borrow()).index(), hash);
            }
        }
        if hash == 0 {
            return 1;
        }
        hash
    }

    fn is_disabled(&self) -> bool {
        self.disabled_scope_.is_active()
    }
}
}
