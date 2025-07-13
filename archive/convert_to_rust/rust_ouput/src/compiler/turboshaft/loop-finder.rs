// Converted from V8 C++ source files:
// Header: loop-finder.h
// Implementation: loop-finder.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr;
use crate::V8_EXPORT_PRIVATE;
use crate::AbortReason;

pub struct LoopFinder {
    phase_zone_: Box<Zone>,
    input_graph_: *const Graph,
    loop_headers_: FixedBlockSidetable<*const Block>,
    loop_header_info_: ZoneUnorderedMap<*const Block, LoopInfo>,
    queue_: ZoneVector<*const Block>,
}

impl LoopFinder {
    pub struct LoopInfo {
        pub start: *const Block,
        pub end: *const Block,
        pub has_inner_loops: bool,
        pub block_count: usize,
        pub op_count: usize,
    }

    impl Default for LoopInfo {
        fn default() -> Self {
            LoopInfo {
                start: ptr::null(),
                end: ptr::null(),
                has_inner_loops: false,
                block_count: 0,
                op_count: 0,
            }
        }
    }

    pub fn new(phase_zone: &mut Zone, input_graph: *const Graph) -> Self {
        let block_count = unsafe { (*input_graph).block_count() };
        let loop_headers_vec = vec![ptr::null(); block_count];
        let mut loop_headers_ = FixedBlockSidetable::new(block_count, ptr::null());

        let mut new_loop_finder = LoopFinder {
            phase_zone_: Box::new(phase_zone.clone()), // Deep copy the zone
            input_graph_: input_graph,
            loop_headers_: loop_headers_,
            loop_header_info_: ZoneUnorderedMap::new(),
            queue_: ZoneVector::new(),
        };
        new_loop_finder.run();
        new_loop_finder
    }

    pub fn loop_headers(&self) -> &ZoneUnorderedMap<*const Block, LoopInfo> {
        &self.loop_header_info_
    }

    pub fn get_loop_header(&self, block: *const Block) -> *const Block {
        self.loop_headers_.get(unsafe { (*block).index() })
    }

    pub fn get_loop_info(&self, block: *const Block) -> LoopInfo {
        assert!(unsafe { (*block).is_loop() });
        match self.loop_header_info_.get(block) {
            Some(info) => info.clone(),
            None => panic!("Loop info not found for block"),
        }
    }

    pub fn get_loop_body(&self, loop_header: *const Block) -> ZoneSet<*const Block, BlockCmp> {
        assert!(!self.get_loop_info(loop_header).has_inner_loops);
        let mut body = ZoneSet::new();
        body.insert(loop_header);

        let mut queue = ZoneVector::new();
        unsafe {
          queue.push_back((*loop_header).last_predecessor());
        }
        while !queue.is_empty() {
            let curr = queue.pop_back().unwrap();
            if body.contains(curr) {
                continue;
            }
            body.insert(curr);

            let mut pred = unsafe { (*curr).last_predecessor() };
            while !pred.is_null() {
                if pred == loop_header {
                    pred = unsafe { (*pred).neighboring_predecessor() };
                    continue;
                }

              queue.push_back(pred);
              pred = unsafe { (*pred).neighboring_predecessor() };
            }
        }

        body
    }

    fn run(&mut self) {
        let blocks = unsafe { (*self.input_graph_).blocks() };
        for block in blocks.iter().rev() {
            if block.is_loop() {
                let info = self.visit_loop(block);
                self.loop_header_info_.insert(block, info);
            }
        }
    }

    fn visit_loop(&mut self, header: &Block) -> LoopInfo {
        let backedge = header.last_predecessor();
        unsafe {
            assert!((*backedge).last_operation(*self.input_graph_).is::<GotoOp>());
            let goto_op = (*backedge).last_operation(*self.input_graph_).cast::<GotoOp>();
            assert_eq!(goto_op.destination, header);
            assert!(backedge.index().id() >= header.index().id());
        }

        let mut info = LoopInfo::default();
        info.op_count = header.op_count_upper_bound();
        info.start = header;
        info.end = backedge;
        info.block_count = 1;

        self.queue_.clear();
        self.queue_.push_back(backedge);

        while !self.queue_.is_empty() {
            let curr = self.queue_.pop_back().unwrap();
            if curr as *const Block == header {
                continue;
            }

            if self.loop_headers_.get(unsafe { (*curr).index() }) != ptr::null() {
                let curr_parent = self.loop_headers_.get(unsafe { (*curr).index() });
                if curr_parent as *const Block == header {
                    continue;
                } else {
                    self.queue_.push_back(curr_parent);
                    info.has_inner_loops = true;
                    continue;
                }
            }

            info.block_count += 1;
            info.op_count += unsafe { (*curr).op_count_upper_bound() };
            self.loop_headers_.set(unsafe { (*curr).index() }, header);

            let mut pred_start = unsafe { (*curr).last_predecessor() };
            if unsafe { (*curr).is_loop() } {
                assert!(!pred_start.is_null());
                pred_start = unsafe { (*pred_start).neighboring_predecessor() };
                info.has_inner_loops = true;
            }

            let mut pred = pred_start;
            while !pred.is_null() {
              self.queue_.push_back(pred);
              pred = unsafe { (*pred).neighboring_predecessor() };
            }
        }

        info
    }

}

impl LoopFinder {
    pub struct BlockCmp {}
}

impl LoopFinder {
    pub fn source(&self, _broker: &JSHeapBroker) -> Node {
        todo!()
    }
}

impl LoopFinder {
    pub fn visited(&self, _op: &Operation) -> bool {
        todo!()
    }
}
impl LoopFinder {
    pub fn graph(&self) -> &Box<dyn Any> {
        todo!()
    }
}

impl BlockCmp for LoopFinder {
    fn compare(&self, a: *const Block, b: *const Block) -> std::cmp::Ordering {
        let a_id = unsafe { (*a).index().id() };
        let b_id = unsafe { (*b).index().id() };
        a_id.cmp(&b_id)
    }
}

pub trait BlockCmp {
    fn compare(&self, a: *const Block, b: *const Block) -> std::cmp::Ordering;
}

#[derive(Debug, Clone)]
pub struct Zone {
    name: String,
}

impl Zone {
    pub fn new(name: String) -> Self {
        Zone { name }
    }
}

#[derive(Debug, Clone)]
pub struct ZoneVector<T> {
    data: Vec<T>,
}

impl<T> ZoneVector<T> {
    pub fn new() -> Self {
        ZoneVector { data: Vec::new() }
    }

    pub fn push_back(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}

#[derive(Debug, Clone)]
pub struct ZoneUnorderedMap<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> ZoneUnorderedMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Copy,
    V: Clone,
{
    pub fn new() -> Self {
        ZoneUnorderedMap { data: HashMap::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<K, V> {
        self.data.iter()
    }
}

#[derive(Debug, Clone)]
pub struct ZoneSet<T, C> {
    data: HashSet<T>,
    comparator: PhantomData<C>,
}

impl<T, C> ZoneSet<T, C>
where
    T: std::cmp::Eq + std::hash::Hash + Copy,
{
    pub fn new() -> Self {
        ZoneSet {
            data: HashSet::new(),
            comparator: PhantomData,
        }
    }

    pub fn insert(&mut self, value: T) {
        self.data.insert(value);
    }

    pub fn contains(&self, value: &T) -> bool {
        self.data.contains(value)
    }

    pub fn remove(&mut self, value: &T) -> bool {
        self.data.remove(value)
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<T> {
        self.data.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

struct FixedBlockSidetable<T: Copy> {
    data: Vec<T>,
    default_value: T,
}

impl<T: Copy> FixedBlockSidetable<T> {
    fn new(block_count: usize, default_value: T) -> Self {
        FixedBlockSidetable {
            data: vec![default_value; block_count],
            default_value,
        }
    }

    fn get(&self, index: BlockIndex) -> T {
        if index.id() < self.data.len() {
            self.data[index.id()]
        } else {
            self.default_value
        }
    }

    fn set(&mut self, index: BlockIndex, value: T) -> bool {
        if index.id() < self.data.len() {
            self.data[index.id()] = value;
            true
        } else {
            false
        }
    }
}

pub struct Graph {
  blocks_: Vec<Block>
}

impl Graph {
  pub fn block_count(&self) -> usize {
    self.blocks_.len()
  }
  pub fn blocks(&self) -> &Vec<Block> {
    &self.blocks_
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockIndex {
    id_: usize,
}

impl BlockIndex {
    pub fn new(id: usize) -> Self {
        BlockIndex { id_: id }
    }
    pub fn id(&self) -> usize {
        self.id_
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Block {
  index_: BlockIndex,
  is_loop_: bool,
  last_predecessor_: *const Block,
  neighboring_predecessor_: *const Block,
  op_count_upper_bound_: usize,
  last_operation_: Operation
}

impl Block {
    pub fn index(&self) -> BlockIndex {
        self.index_
    }
    pub fn is_loop(&self) -> bool {
        self.is_loop_
    }

    pub fn last_predecessor(&self) -> *const Block {
      self.last_predecessor_
    }

    pub fn neighboring_predecessor(&self) -> *const Block {
      self.neighboring_predecessor_
    }

    pub fn op_count_upper_bound(&self) -> usize {
      self.op_count_upper_bound_
    }

    pub fn last_operation(&self, _input_graph_: *const Graph) -> Operation {
      self.last_operation_.clone()
    }
}

pub struct Operation {
    kind: OperationKind,
}

impl Operation {
    pub fn is<T>(&self) -> bool {
        match self.kind {
            OperationKind::GotoOp => true,
            _ => false,
        }
    }

    pub fn cast<T>(&self) -> GotoOp {
        match &self.kind {
            OperationKind::GotoOp => GotoOp { destination: ptr::null() },
            _ => panic!("Invalid cast"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OperationKind {
    GotoOp,
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GotoOp {
    destination: *const Block,
}

pub struct NeighboringPredecessorIterable<'a> {
  pred_start: *const Block,
  _marker: PhantomData<&'a Block>,
}

impl<'a> NeighboringPredecessorIterable<'a> {
  pub fn new(pred_start: *const Block) -> Self {
      NeighboringPredecessorIterable { pred_start, _marker: PhantomData }
  }
}

impl<'a> Iterator for NeighboringPredecessorIterable<'a> {
  type Item = *const Block;

  fn next(&mut self) -> Option<Self::Item> {
    if self.pred_start.is_null() {
      return None;
    }
    let current = self.pred_start;
    self.pred_start = unsafe { (*self.pred_start).neighboring_predecessor() };
    Some(current)
  }
}

pub mod base {
  pub struct Reversed<'a, T>(&'a Vec<T>);

  impl<'a, T> Reversed<'a, T> {
      pub fn new(vec: &'a Vec<T>) -> Self {
          Reversed(vec)
      }
  }

  impl<'a, T> IntoIterator for Reversed<'a, T> {
      type Item = &'a T;
      type IntoIter = std::iter::Rev<std::slice::Iter<'a, T>>;

      fn into_iter(self) -> Self::IntoIter {
          self.0.iter().rev()
      }
  }

  use std::ops::Deref;

  impl<'a, T> Deref for Reversed<'a, T> {
      type Target = Vec<T>;

      fn deref(&self) -> &Self::Target {
          self.0
      }
  }

  use std::vec::IntoIter;
}

fn NeighboringPredecessorIterable(pred_start: *const Block) -> NeighboringPredecessorIterable {
  NeighboringPredecessorIterable::new(pred_start)
}

pub struct JSHeapBroker {}

pub struct Node {}

pub struct MapRef {}

pub struct Code {}
pub struct MaybeIndirectHandle<T> {
    value: *mut T,
}

impl<T> MaybeIndirectHandle<T> {
    pub fn new(value: *mut T) -> Self {
        MaybeIndirectHandle { value }
    }
}
pub struct Isolate {}

pub struct Simulator {}

