// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/base/logging.h is not directly translatable - using println! for now
// src/base/vector.h is replaced by std::vec::Vec
// src/utils/utils.h is not directly translatable - using std lib equivalents

use std::{
    any::Any,
    cell::RefCell,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    rc::Rc,
};

// Missing dependency Graph
#[allow(dead_code)]
struct Graph {}

// Missing dependency Operation
#[allow(dead_code)]
struct Operation {
    opcode: Opcode,
}

impl Operation {
    fn is<T: Any>(&self) -> bool {
        todo!()
    }

    fn cast<T: Any>(&self) -> &T {
        todo!()
    }

    fn effects(&self) -> Effects {
        todo!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
enum Opcode {
    kCatchBlockBegin,
    kComment,
    kDeoptimizeIf,
    kPendingLoopPhi,
    Other,
}

// Missing dependency MayThrow trait/function
fn may_throw(opcode: Opcode) -> bool {
    opcode == Opcode::Other
}

// Missing dependency Effects
struct Effects {
    repetition_is_eliminatable_: bool,
}

impl Effects {
    fn repetition_is_eliminatable(&self) -> bool {
        self.repetition_is_eliminatable_
    }
}

// Missing dependency Block
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct BlockIndex(usize);

// Missing dependency OpIndex
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct OpIndex(usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Block {
    index: BlockIndex,
    depth: usize,
    dominator: Option<BlockIndex>,
}

impl Block {
    fn index(&self) -> BlockIndex {
        self.index
    }

    fn depth(&self) -> usize {
        self.depth
    }

    fn get_dominator(&self) -> Option<BlockIndex> {
        self.dominator
    }
}

// Missing dependency Assembler
#[allow(dead_code)]
struct Assembler<'a> {
    output_graph: Graph,
    current_block: RefCell<Option<&'a Block>>,
    phase_zone: Rc<Zone>,
    input_graph: Graph,
}

impl<'a> Assembler<'a> {
    fn output_graph(&mut self) -> &mut Graph {
        &mut self.output_graph
    }
    fn current_block(&self) -> &Block {
        self.current_block.borrow().unwrap()
    }
    fn phase_zone(&self) -> Rc<Zone> {
        self.phase_zone.clone()
    }
    fn input_graph(&self) -> &Graph {
        &self.input_graph
    }
}

// Missing dependency fast_hash_combine
fn fast_hash_combine(seed: BlockIndex, value: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    value.hash(&mut hasher);
    hasher.finish() as usize
}

// Missing dependency operation_to_opcode_v
fn operation_to_opcode_v<T>() -> Opcode {
    Opcode::Other
}

// Missing dependency PendingLoopPhiOp
struct PendingLoopPhiOp {}

// Missing dependency DeoptimizeIfOp
struct DeoptimizeIfOp {}

// Missing dependency PhiOp
struct PhiOp {}

impl PhiOp {
    fn equals_for_gvn(&self, other: &Self) -> bool {
        true
    }
}

// Missing dependency TURBOSHAFT_REDUCER_BOILERPLATE
macro_rules! turboshaft_reducer_boilerplate {
    ($name:ident) => {
        fn reducer_name(&self) -> &'static str {
            stringify!($name)
        }

        fn should_skip_optimization_step(&self) -> bool {
            false
        }

        fn asm(&mut self) -> &mut Assembler {
            &mut self.asm_
        }
    };
}

// Missing dependency REDUCER_LIST_CONTAINS
macro_rules! reducer_list_contains {
    ($reducer_list:ty, $reducer:ty) => {
        true
    };
}

// Missing dependency next_is_bottom_of_assembler_stack
macro_rules! next_is_bottom_of_assembler_stack {
    ($next:ty) => {
        true
    };
}

// Missing dependency next_reducer_is
macro_rules! next_reducer_is {
    ($next:ty, $reducer:ty) => {
        false
    };
}

// Missing dependency RemoveLast trait/function
trait RemoveLast {
    fn remove_last(&mut self, _op_idx: OpIndex) {}
}

// Missing dependency Zone
struct Zone {}

impl Zone {
    fn new_vector<T>(&self, capacity: usize) -> ZoneVector<T> {
        ZoneVector {
            zone: Rc::new(self),
            data: Vec::with_capacity(capacity),
        }
    }
}

// Missing dependency ZoneVector
struct ZoneVector<T> {
    zone: Rc<Zone>,
    data: Vec<T>,
}

impl<T> ZoneVector<T> {
    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn back(&self) -> Option<&T> {
        self.data.last()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        self.data.resize(new_len, value);
    }

    fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    fn end(&self) -> *const T {
        unsafe { self.data.as_ptr().add(self.data.len()) }
    }
    fn begin(&self) -> *const T {
        self.data.as_ptr()
    }
}

impl<T> Deref for ZoneVector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for ZoneVector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> Extend<T> for ZoneVector<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
}

impl<T> IntoIterator for ZoneVector<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

struct ScopeCounter {
    scopes_: RefCell<i32>,
}

impl ScopeCounter {
    fn new() -> Self {
        ScopeCounter {
            scopes_: RefCell::new(0),
        }
    }
    fn enter(&self) {
        *self.scopes_.borrow_mut() += 1;
    }
    fn leave(&self) {
        *self.scopes_.borrow_mut() -= 1;
    }
    fn is_active(&self) -> bool {
        *self.scopes_.borrow() > 0
    }
}

// In rare cases of intentional duplication of instructions, we need to disable
// value numbering. This scope manages that.
struct DisableValueNumbering<'a, R> {
    scopes_: Option<&'a ScopeCounter>,
    _phantom: PhantomData<R>,
}

impl<'a, R> DisableValueNumbering<'a, R> {
    fn new<Reducer>(reducer: &'a mut Reducer) -> Self
    where
        Reducer: ValueNumberingReducerTrait,
    {
        if reducer_list_contains!(<Reducer as ValueNumberingReducerTrait>::ReducerList, ValueNumberingReducer<Next<Reducer>>>) {
            let scopes_ = Some(reducer.gvn_disabled_scope());
            scopes_.unwrap().enter();
            DisableValueNumbering {
                scopes_,
                _phantom: PhantomData,
            }
        } else {
            DisableValueNumbering {
                scopes_: None,
                _phantom: PhantomData,
            }
        }
    }
}

impl<'a, R> Drop for DisableValueNumbering<'a, R> {
    fn drop(&mut self) {
        if let Some(scopes_) = self.scopes_ {
            scopes_.leave();
        }
    }
}

trait ValueNumberingReducerTrait {
    type ReducerList;
    fn gvn_disabled_scope(&mut self) -> &ScopeCounter;
}

trait Next<T> {}

struct ValueNumberingReducer<NextReducer> {
    asm_: RefCell<Assembler<'static>>, // Using a static lifetime here because of missing lifetimes.
    dominator_path_: RefCell<ZoneVector<Block>>,
    table_: RefCell<ZoneVector<Entry>>,
    mask_: RefCell<usize>,
    entry_count_: RefCell<usize>,
    depths_heads_: RefCell<ZoneVector<*mut Entry>>,
    disabled_scope_: ScopeCounter,
    _phantom: PhantomData<NextReducer>,
}

impl<NextReducer> ValueNumberingReducer<NextReducer> {
    fn new(asm: Assembler<'static>) -> Self {
        let table_size = std::cmp::max(128, asm.input_graph().op_id_capacity() / 2);
        let table_size = table_size.next_power_of_two();
        let mask_ = table_size - 1;
        ValueNumberingReducer {
            asm_: RefCell::new(asm),
            dominator_path_: RefCell::new(
                Assembler::<'static>::phase_zone(&asm_).new_vector(0),
            ),
            table_: RefCell::new(
                Assembler::<'static>::phase_zone(&asm_).new_vector(table_size),
            ),
            mask_: RefCell::new(mask_),
            entry_count_: RefCell::new(0),
            depths_heads_: RefCell::new(
                Assembler::<'static>::phase_zone(&asm_).new_vector(0),
            ),
            disabled_scope_: ScopeCounter::new(),
            _phantom: PhantomData,
        }
    }
}

impl<NextReducer> ValueNumberingReducer<NextReducer> {
    fn reducer_name(&self) -> &'static str {
        "ValueNumbering"
    }
}

impl<NextReducer> ValueNumberingReducer<NextReducer>
where
    NextReducer: RemoveLast,
{
    fn should_skip_optimization_step(&self) -> bool {
        false
    }
}

impl<NextReducer> ValueNumberingReducer<NextReducer>
where
    NextReducer: RemoveLast,
{
    fn asm(&self) -> std::cell::RefMut<'_, Assembler<'static>> {
        self.asm_.borrow_mut()
    }
}

impl<NextReducer> ValueNumberingReducer<NextReducer>
where
    NextReducer: RemoveLast,
{
    fn bind(&self, block: &Block) {
        // Missing: Next::Bind(block);
        self.reset_to_block(block);
        self.dominator_path_.borrow_mut().push(*block);
        self.depths_heads_.borrow_mut().push(std::ptr::null_mut());
    }

    // Resets {table_} up to the first dominator of {block} that it contains.
    fn reset_to_block(&self, block: &Block) {
        let mut target = block.get_dominator();

        while !self.dominator_path_.borrow().is_empty() && target.is_some()
            && self.dominator_path_.borrow().back().unwrap() != target.map(|x| *x).unwrap()
        {
            if self.dominator_path_.borrow().back().unwrap().depth() > target.map(|x| *x).unwrap().depth() {
                self.clear_current_depth_entries();
            } else if self.dominator_path_.borrow().back().unwrap().depth() < target.map(|x| *x).unwrap().depth() {
                target = target.map(|t| {
                    let mut temp = *t;
                    temp.get_dominator()
                }).flatten();
            } else {
                // {target} and {dominator_path.back} have the same depth but are not
                // equal, so we go one level up for both.
                self.clear_current_depth_entries();
                target = target.map(|t| {
                    let mut temp = *t;
                    temp.get_dominator()
                }).flatten();
            }
        }
    }

    fn will_gvn_op<Op>(&self, op: &Op) -> bool {
        let entry = self.find(op);
        !entry.is_empty()
    }
}

impl<NextReducer> ValueNumberingReducer<NextReducer>
where
    NextReducer: RemoveLast,
{
    fn gvn_disabled_scope(&self) -> &ScopeCounter {
        &self.disabled_scope_
    }
}

// TODO(dmercadier): Once the mapping from Operations to Blocks has been added
// to turboshaft, remove the `block` field from the `Entry` structure.
#[derive(Clone, Copy)]
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

impl<NextReducer> ValueNumberingReducer<NextReducer>
where
    NextReducer: RemoveLast,
{
    fn add_or_find<Op>(&self, op_idx: OpIndex) -> OpIndex {
        if self.is_disabled() {
            return op_idx;
        }

        let op = self.asm().output_graph.get(op_idx).cast::<Op>();
        if std::any::TypeId::of::<Op>() == std::any::TypeId::of::<PendingLoopPhiOp>() ||
         Operation::effects(op).repetition_is_eliminatable() &&
            std::any::TypeId::of::<Op>() != std::any::TypeId::of::<DeoptimizeIfOp>()
        {
            // GVNing DeoptimizeIf is safe, despite its lack of
            // repetition_is_eliminatable.
            return op_idx;
        }

        self.rehash_if_needed();

        let mut hash = 0;
        let entry = self.find::<Op>(op, &mut hash);
        if entry.is_empty() {
            // {op} is not present in the state, inserting it.
            let mut new_entry = Entry {
                value: op_idx,
                block: self.asm().current_block().index(),
                hash,
                depth_neighboring_entry: *self.depths_heads_.borrow().back().unwrap(),
            };
            let entry_ptr = unsafe { (entry as *const Entry).as_mut() }.unwrap();
            *entry_ptr = new_entry;
            *self.depths_heads_.borrow_mut().back_mut().unwrap() =
                entry as *mut Entry;
            *self.entry_count_.borrow_mut() += 1;
            return op_idx;
        } else {
            // {op} is already present, removing it from the graph and returning the
            // previous one.
            // Missing: Next::RemoveLast(op_idx);
            return entry.value;
        }
    }

    fn find<Op>(&self, op: &Op, hash_ret: &mut usize) -> &mut Entry {
        let same_block_only = std::any::TypeId::of::<Op>() == std::any::TypeId::of::<PhiOp>();
        let hash = self.compute_hash::<same_block_only, Op>(op);
        *hash_ret = hash;
        let start_index = hash & *self.mask_.borrow();
        for i in (start_index..*self.table_.borrow().len()).chain(0..start_index) {
            let entry = &mut self.table_.borrow_mut()[i];
            if entry.is_empty() {
                // We didn't find {op} in {table_}. Returning where it could be
                // inserted.
                return entry;
            }
            if entry.hash == hash {
                let entry_op = self.asm().output_graph.get(entry.value);
                if std::any::TypeId::of::<Op>() == std::any::TypeId::of::<Operation>() &&
                    entry_op.is::<Op>()
                    && (!same_block_only
                        || entry.block == self.asm().current_block().index())
                {
                    // && entry_op.Cast::<Op>().EqualsForGVN(op) {
                    return entry;
                }
            }
        }
        panic!("Infinite loop detected");
    }

    // Remove all of the Entries of the current depth.
    fn clear_current_depth_entries(&self) {
        let mut entry = *self.depths_heads_.borrow().back().unwrap();
        while !entry.is_null() {
            unsafe {
                (*entry).hash = 0;
                entry = (*entry).depth_neighboring_entry;
            }
            *self.depths_heads_.borrow_mut().back_mut().unwrap() = entry;
            *self.entry_count_.borrow_mut() -= 1;
        }

        self.depths_heads_.borrow_mut().pop();
        self.dominator_path_.borrow_mut().pop();
    }

    // If the table is too full, double its size and re-insert the old entries.
    fn rehash_if_needed(&self) {
        if self.table_.borrow().len() - (self.table_.borrow().len() / 4) > *self.entry_count_.borrow() {
            return;
        }

        let new_size = self.table_.borrow().len() * 2;
        let mut new_table = Assembler::<'static>::phase_zone(&self.asm().clone()).new_vector::<Entry>(new_size);
        let mut mask = new_size - 1;

        for depth_idx in 0..self.depths_heads_.borrow().len() {
            // It's important to fill the new hash by inserting data in increasing
            // depth order, in order to avoid holes when later calling
            // ClearCurrentDepthEntries. Consider for instance:
            //
            //  ---+------+------+------+----
            //     |  a1  |  a2  |  a3  |
            //  ---+------+------+------+----
            //
            // Where a1, a2 and a3 have the same hash. By construction, we know that
            // depth(a1) <= depth(a2) <= depth(a3). If, when re-hashing, we were to
            // insert them in another order, say:
            //
            //  ---+------+------+------+----
            //     |  a3  |  a1  |  a2  |
            //  ---+------+------+------+----
            //
            // Then, when we'll call ClearCurrentDepthEntries to remove entries from
            // a3's depth, we'll get this:
            //
            //  ---+------+------+------+----
            //     | null |  a1  |  a2  |
            //  ---+------+------+------+----
            //
            // And, when looking if a1 is in the hash, we'd find a "null" where we
            // expect it, and assume that it's not present. If, instead, we always
            // conserve the increasing depth order, then when removing a3, we'd get:
            //
            //  ---+------+------+------+----
            //     |  a1  |  a2  | null |
            //  ---+------+------+------+----
            //
            // Where we can still find a1 and a2.
            let mut entry = *self.depths_heads_.borrow()[depth_idx];
            *self.depths_heads_.borrow_mut().get_mut(depth_idx).unwrap() = std::ptr::null_mut();

            while !entry.is_null() {
                unsafe {
                    let hash = (*entry).hash;
                    let mut i = hash & mask;
                    loop {
                        if new_table[i].hash == 0 {
                            new_table[i] = *(*entry as *const Entry);
                            let next_entry = (*entry).depth_neighboring_entry;
                            new_table[i].depth_neighboring_entry = *self.depths_heads_.borrow().get(depth_idx).unwrap();
                            *self.depths_heads_.borrow_mut().get_mut(depth_idx).unwrap() = &mut new_table[i] as *mut Entry;
                            entry = next_entry;
                            break;
                        }
                        i = self.next_entry_index(i, mask);
                    }
                }
            }
        }
        *self.table_.borrow_mut() = new_table;
        *self.mask_.borrow_mut() = mask;
    }

    fn compute_hash<const SAME_BLOCK_ONLY: bool, Op>(&self, op: &Op) -> usize
    where
        Op: Hash,
    {
        let mut hasher = DefaultHasher::new();
        op.hash(&mut hasher);
        let mut hash = hasher.finish() as usize;
        if SAME_BLOCK_ONLY {
            hash = fast_hash_combine(self.asm().current_block().index(), hash);
        }
        if hash == 0 {
            return 1;
        }
        hash
    }

    fn next_entry_index(&self, index: usize, mask: usize) -> usize {
        (index + 1) & mask
    }

    fn is_disabled(&self) -> bool {
        self.disabled_scope_.is_active()
    }
}

trait OperationExt {
    fn hash_value(&self) -> usize;
    fn is_block_terminator(&self) -> bool;
}

impl<T: Hash> OperationExt for T {
    fn hash_value(&self) -> usize {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish() as usize
    }

    fn is_block_terminator(&self) -> bool {
        false
    }
}

impl Graph {
    fn next_operation_index(&self) -> OpIndex {
        todo!()
    }
    fn get(&self, _op_idx: OpIndex) -> &Operation {
        todo!()
    }

    fn op_id_capacity(&self) -> usize {
        todo!()
    }
}