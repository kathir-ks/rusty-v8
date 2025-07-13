// Converted from V8 C++ source files:
// Header: wasm-revec-reducer.h
// Implementation: wasm-revec-reducer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp;
use std::mem;
// use std::fmt;
// use std::fmt::Debug;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};
use crate::base::safe_conversions::From;
use crate::compiler::turboshaft::assembler::*;
use crate::compiler::turboshaft::operations::*;
use crate::compiler::turboshaft::phase::*;
use crate::compiler::turboshaft::use_map::*;
use crate::compiler::wasm_graph_assembler::*;
use crate::compiler::wasm_compiler_definitions::*;
use crate::compiler::turboshaft::opmasks::*;
use crate::wasm::simd_shuffle::*;
use crate::compiler::turboshaft::representations::*;
use crate::compiler::common_node_cache::*;
use crate::execution::isolate::*;
use crate::handles::*;
use crate::compiler::js_heap_broker::*;
use crate::wasm::wasm_module::*;
use crate::zone::*;
use crate::bigint::*;
use crate::deoptimizer::frame_description::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OpEffects {
    produces: EffectDimensions,
    consumes: EffectDimensions,
}

impl OpEffects {
    fn new() -> Self {
        OpEffects {
            produces: EffectDimensions::new(),
            consumes: EffectDimensions::new(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct EffectDimensions {
    memory: bool,
    control_flow: bool,
    other: bool,
}

impl EffectDimensions {
    fn new() -> Self {
        EffectDimensions {
            memory: false,
            control_flow: false,
            other: false,
        }
    }

    fn bits(&self) -> u8 {
        let mut bits: u8 = 0;
        if self.memory { bits |= 1; }
        if self.control_flow { bits |= 2; }
        if self.other { bits |= 4; }
        bits
    }
}

// trait Printable {
//     fn print(&self) -> String;
// }

// impl<T> Printable for Vec<T> where T: Debug {
//     fn print(&self) -> String {
//         format!("{:?}", self)
//     }
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Simd256LoadTransformOp_TransformKind {
  _8x8S,
  _8x16S,                    
  _8x8U,
  _8x16U,                    
  _16x4S,
  _16x8S,                   
  _16x4U,
  _16x8U,                   
  _32x2S,
  _32x4S,                   
  _32x2U,
  _32x4U,                   
  _8Splat,
  _8Splat1,                 
  _16Splat,
  _16Splat1,               
  _32Splat,
  _32Splat1,               
  _64Splat,
  _64Splat1,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Simd256UnaryOp_Kind {
    _S128Not,
    _S256Not,                                      
    _I8x16Abs,
    _I8x32Abs,                                    
    _I8x16Neg,
    _I8x32Neg,                                    
    _I16x8ExtAddPairwiseI8x16S,
    _I16x16ExtAddPairwiseI8x32S, 
    _I16x8ExtAddPairwiseI8x16U,
    _I16x16ExtAddPairwiseI8x32U, 
    _I32x4ExtAddPairwiseI16x8S,
    _I32x8ExtAddPairwiseI16x16S, 
    _I32x4ExtAddPairwiseI16x8U,
    _I32x8ExtAddPairwiseI16x16U, 
    _I16x8Abs,
    _I16x16Abs,                                   
    _I16x8Neg,
    _I16x16Neg,                                   
    _I32x4Abs,
    _I32x8Abs,                                    
    _I32x4Neg,
    _I32x8Neg,                                    
    _F32x4Abs,
    _F32x8Abs,                                    
    _F32x4Neg,
    _F32x8Neg,                                    
    _F32x4Sqrt,
    _F32x8Sqrt,                                  
    _F64x2Abs,
    _F64x4Abs,                                    
    _F64x2Neg,
    _F64x4Neg,                                    
    _F64x2Sqrt,
    _F64x4Sqrt,                                  
    _I32x4UConvertF32x4,
    _I32x8UConvertF32x8,                
    _I32x4SConvertF32x4,
    _I32x8SConvertF32x8,                
    _F32x4UConvertI32x4,
    _F32x8UConvertI32x8,                
    _F32x4SConvertI32x4,
    _F32x8SConvertI32x8,                
    _I32x4RelaxedTruncF32x4S,
    _I32x8RelaxedTruncF32x8S,      
    _I32x4RelaxedTruncF32x4U,
    _I32x8RelaxedTruncF32x8U,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Simd256BinopOp_Kind {
    _I8x16Eq,
    _I8x32Eq,                              
    _I8x16Ne,
    _I8x32Ne,                              
    _I8x16GtS,
    _I8x32GtS,                            
    _I8x16GtU,
    _I8x32GtU,                            
    _I8x16GeS,
    _I8x32GeS,                            
    _I8x16GeU,
    _I8x32GeU,                            
    _I16x8Eq,
    _I16x16Eq,                             
    _I16x8Ne,
    _I16x16Ne,                             
    _I16x8GtS,
    _I16x16GtS,                           
    _I16x8GtU,
    _I16x16GtU,                           
    _I16x8GeS,
    _I16x16GeS,                           
    _I16x8GeU,
    _I16x16GeU,                           
    _I32x4Eq,
    _I32x8Eq,                              
    _I32x4Ne,
    _I32x8Ne,                              
    _I32x4GtS,
    _I32x8GtS,                            
    _I32x4GtU,
    _I32x8GtU,                            
    _I32x4GeS,
    _I32x8GeS,                            
    _I32x4GeU,
    _I32x8GeU,                            
    _F32x4Eq,
    _F32x8Eq,                              
    _F32x4Ne,
    _F32x8Ne,                              
    _F32x4Lt,
    _F32x8Lt,                              
    _F32x4Le,
    _F32x8Le,                              
    _F64x2Eq,
    _F64x4Eq,                              
    _F64x2Ne,
    _F64x4Ne,                              
    _F64x2Lt,
    _F64x4Lt,                              
    _F64x2Le,
    _F64x4Le,                              
    _S128And,
    _S256And,                              
    _S128AndNot,
    _S256AndNot,                        
    _S128Or,
    _S256Or,                                
    _S128Xor,
    _S256Xor,                              
    _I8x16SConvertI16x8,
    _I8x32SConvertI16x16,       
    _I8x16UConvertI16x8,
    _I8x32UConvertI16x16,       
    _I8x16Add,
    _I8x32Add,                            
    _I8x16AddSatS,
    _I8x32AddSatS,                    
    _I8x16AddSatU,
    _I8x32AddSatU,                    
    _I8x16Sub,
    _I8x32Sub,                            
    _I8x16SubSatS,
    _I8x32SubSatS,                    
    _I8x16SubSatU,
    _I8x32SubSatU,                    
    _I8x16MinS,
    _I8x32MinS,                          
    _I8x16MinU,
    _I8x32MinU,                          
    _I8x16MaxS,
    _I8x32MaxS,                          
    _I8x16MaxU,
    _I8x32MaxU,                          
    _I8x16RoundingAverageU,
    _I8x32RoundingAverageU,  
    _I16x8SConvertI32x4,
    _I16x16SConvertI32x8,       
    _I16x8UConvertI32x4,
    _I16x16UConvertI32x8,       
    _I16x8Add,
    _I16x16Add,                           
    _I16x8AddSatS,
    _I16x16AddSatS,                   
    _I16x8AddSatU,
    _I16x16AddSatU,                   
    _I16x8Sub,
    _I16x16Sub,                           
    _I16x8SubSatS,
    _I16x16SubSatS,                   
    _I16x8SubSatU,
    _I16x16SubSatU,                   
    _I16x8Mul,
    _I16x16Mul,                           
    _I16x8MinS,
    _I16x16MinS,                         
    _I16x8MinU,
    _I16x16MinU,                         
    _I16x8MaxS,
    _I16x16MaxS,                         
    _I16x8MaxU,
    _I16x16MaxU,                         
    _I16x8RoundingAverageU,
    _I16x16RoundingAverageU, 
    _I32x4Add,
    _I32x8Add,                            
    _I32x4Sub,
    _I32x8Sub,                            
    _I32x4Mul,
    _I32x8Mul,                            
    _I32x4MinS,
    _I32x8MinS,                          
    _I32x4MinU,
    _I32x8MinU,                          
    _I32x4MaxS,
    _I32x8MaxS,                          
    _I32x4MaxU,
    _I32x8MaxU,                          
    _I32x4DotI16x8S,
    _I32x8DotI16x16S,               
    _I64x2Add,
    _I64x4Add,                            
    _I64x2Sub,
    _I64x4Sub,                            
    _I64x2Mul,
    _I64x4Mul,                            
    _I64x2Eq,
    _I64x4Eq,                              
    _I64x2Ne,
    _I64x4Ne,                              
    _I64x2GtS,
    _I64x4GtS,                            
    _I64x2GeS,
    _I64x4GeS,                            
    _F32x4Add,
    _F32x8Add,                            
    _F32x4Sub,
    _F32x8Sub,                            
    _F32x4Mul,
    _F32x8Mul,                            
    _F32x4Div,
    _F32x8Div,                            
    _F32x4Min,
    _F32x8Min,                            
    _F32x4Max,
    _F32x8Max,                            
    _F32x4Pmin,
    _F32x8Pmin,                          
    _F32x4Pmax,
    _F32x8Pmax,                          
    _F64x2Add,
    _F64x4Add,                            
    _F64x2Sub,
    _F64x4Sub,                            
    _F64x2Mul,
    _F64x4Mul,                            
    _F64x2Div,
    _F64x4Div,                            
    _F64x2Min,
    _F64x4Min,                            
    _F64x2Max,
    _F64x4Max,                            
    _F64x2Pmin,
    _F64x4Pmin,                          
    _F64x2Pmax,
    _F64x4Pmax,                          
    _F32x4RelaxedMin,
    _F32x8RelaxedMin,              
    _F32x4RelaxedMax,
    _F32x8RelaxedMax,              
    _F64x2RelaxedMin,
    _F64x4RelaxedMin,              
    _F64x2RelaxedMax,
    _F64x4RelaxedMax,              
    _I16x8DotI8x16I7x16S,
    _I16x16DotI8x32I7x32S,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Simd256ShiftOp_Kind {
    _I16x8Shl,
    _I16x16Shl,    
    _I16x8ShrS,
    _I16x16ShrS,  
    _I16x8ShrU,
    _I16x16ShrU,  
    _I32x4Shl,
    _I32x8Shl,     
    _I32x4ShrS,
    _I32x8ShrS,   
    _I32x4ShrU,
    _I32x8ShrU,   
    _I64x2Shl,
    _I64x4Shl,     
    _I64x2ShrU,
    _I64x4ShrU,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Simd256TernaryOp_Kind {
    _S128Select,
    _S256Select,                          
    _F32x4Qfma,
    _F32x8Qfma,                            
    _F32x4Qfms,
    _F32x8Qfms,                            
    _F64x2Qfma,
    _F64x4Qfma,                            
    _F64x2Qfms,
    _F64x4Qfms,                            
    _I8x16RelaxedLaneSelect,
    _I8x32RelaxedLaneSelect,  
    _I16x8RelaxedLaneSelect,
    _I16x16RelaxedLaneSelect, 
    _I32x4RelaxedLaneSelect,
    _I32x8RelaxedLaneSelect,  
    _I64x2RelaxedLaneSelect,
    _I64x4RelaxedLaneSelect,  
    _I32x4DotI8x16I7x16AddS,
    _I32x8DotI8x32I7x32AddS,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Simd256SplatOp_Kind {
    _I8x16,
    _I8x32,           
    _I16x8,
    _I16x16,          
    _I32x4,
    _I32x8,           
    _I64x2,
    _I64x4,           
    _F32x4,
    _F32x8,           
    _F64x2,
    _F64x4,
}

const K_SIMD256_SIZE: usize = 32;
const K_SIMD128_SIZE: usize = 16;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeGroup {
  indexes: [OpIndex; 2],
}

impl NodeGroup {
  // Current only support merge 2 Simd128 into Simd256
  const K_SIZE: usize = K_SIMD256_SIZE / K_SIMD128_SIZE;
  
  pub fn new(a: OpIndex, b: OpIndex) -> Self {
    NodeGroup { indexes: [a, b] }
  }

  pub fn size(&self) -> usize {
    Self::K_SIZE
  }

  pub fn get(&self, i: usize) -> OpIndex {
    self.indexes[i]
  }

  pub fn begin(&self) -> &[OpIndex; 2] {
    &self.indexes
  }

  pub fn end(&self) -> *const OpIndex {
    self.indexes.as_ptr_range().end
  }
}

// A PackNode consists of a fixed number of isomorphic simd128 nodes which can
// execute in parallel and convert to a 256-bit simd node later. The nodes in a
// PackNode must satisfy that they can be scheduled in the same basic block and
// are mutually independent.
#[derive(Debug)]
pub struct PackNode {
  nodes: NodeGroup,
  revectorized_node: Option<OpIndex>,
  operands: Vec<Option<Box<PackNode>>>,
  node_type: NodeType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeType {
  KDefault,        // Nodes are naturally packed without special attributes.
  KForcePackNode,  // Nodes do not satisfy some packing rule, but can be
                   // forcely coalesced with a Pack128To256 operation. E.g.
                   // inconsecutive loads. In x64, we can use the vinsertf128
                   // instruction to forcely coalescing two 128-bit values.
  KShufflePackNode,  // Nodes are Simd128Shuffle operations with specific
                       // info.
  KBundlePackNode,   // Nodes representing a i8x16/i16x8 to f32x4 conversion.
  KIntersectPackNode,  // One or more nodes already packed by an existing
                         // PackNode.
}

impl PackNode {
  pub fn new(node_group: NodeGroup, node_type: NodeType) -> Self {
    PackNode {
      nodes: node_group,
      revectorized_node: None,
      operands: Vec::new(),
      node_type,
    }
  }

  pub fn nodes(&self) -> &NodeGroup {
    &self.nodes
  }

  pub fn is_same(&self, node_group: &NodeGroup) -> bool {
    self.nodes == *node_group
  }

  pub fn is_same_packnode(&self, other: &PackNode) -> bool {
    self.nodes == other.nodes
  }

  pub fn revectorized_node(&self) -> Option<OpIndex> {
    self.revectorized_node
  }

  pub fn set_revectorized_node(&mut self, node: OpIndex) {
    self.revectorized_node = Some(node);
  }

  pub fn is_default_pack_node(&self) -> bool {
    self.node_type == NodeType::KDefault
  }

  pub fn is_force_pack_node(&self) -> bool {
    self.node_type == NodeType::KForcePackNode
  }

  pub fn is_shuffle_pack_node(&self) -> bool {
    self.node_type == NodeType::KShufflePackNode
  }

  pub fn is_bundle_pack_node(&self) -> bool {
    self.node_type == NodeType::KBundlePackNode
  }

  // We will force-pack nodes for both ForcePackNode and IntersectPackNode.
  pub fn is_force_packing(&self) -> bool {
    self.node_type == NodeType::KForcePackNode || self.node_type == NodeType::KIntersectPackNode
  }

  pub fn as_force_pack_node(&mut self) -> Option<&mut ForcePackNode> {
    if self.is_force_pack_node() {
      Some(unsafe { mem::transmute(self) })
    } else {
      None
    }
  }

  pub fn as_shuffle_pack_node(&mut self) -> Option<&mut ShufflePackNode> {
    if self.is_shuffle_pack_node() {
      Some(unsafe { mem::transmute(self) })
    } else {
      None
    }
  }

    pub fn as_bundle_pack_node(&mut self) -> Option<&mut BundlePackNode> {
        if self.is_bundle_pack_node() {
            Some(unsafe { mem::transmute(self) })
        } else {
            None
        }
    }

  pub fn get_operand(&self, index: usize) -> Option<&PackNode> {
    self.operands.get(index).and_then(|op| op.as_ref().map(|boxed| &**boxed))
  }

  pub fn set_operand(&mut self, index: usize, pnode: PackNode) {
    if self.operands.len() < index + 1 {
      self.operands.resize_with(index + 1, || None);
    }
    self.operands[index] = Some(Box::new(pnode));
  }

  pub fn get_operands_size(&self) -> usize {
    self.operands.len()
  }

  pub fn print(&self, _graph: &Graph) {
      println!("PackNode print is unimplemented!");
  }
}

#[derive(Debug)]
pub struct ForcePackNode {
  base: PackNode,
  force_pack_type: ForcePackType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ForcePackType {
  KSplat,    // force pack 2 identical nodes or 2 loads at the same address
  KGeneral,  // force pack 2 different nodes
}

impl ForcePackNode {
  pub fn new(node_group: NodeGroup, type_: ForcePackType) -> Self {
    ForcePackNode {
      base: PackNode::new(node_group, NodeType::KForcePackNode),
      force_pack_type: type_,
    }
  }

  pub fn force_pack_type(&self) -> ForcePackType {
    self.force_pack_type
  }
}

#[derive(Debug)]
pub struct ShufflePackNode {
  base: PackNode,
  info: SpecificInfo,
}

impl ShufflePackNode {
    pub fn new(node_group: NodeGroup, kind: SpecificInfo_Kind) -> Self {
        ShufflePackNode {
            base: PackNode::new(node_group, NodeType::KShufflePackNode),
            info: SpecificInfo { kind: kind, param: Param { splat_index: 0 } },
        }
    }

    pub fn info(&mut self) -> &mut SpecificInfo {
        &mut self.info
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SpecificInfo_Kind {
    kS256Load32Transform,
    kS256Load64Transform,
    kS256Load8x8U,
    kShufd,
    kShufps,
    kS32x8UnpackLow,
    kS32x8UnpackHigh,
}

#[derive(Debug, Copy, Clone)]
pub union Param {
    splat_index: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct SpecificInfo {
    kind: SpecificInfo_Kind,
    param: Param,
}

impl SpecificInfo {
    pub fn kind(&self) -> SpecificInfo_Kind {
        self.kind
    }

    pub fn set_kind(&mut self, kind: SpecificInfo_Kind) {
        self.kind = kind;
    }

    pub fn set_splat_index(&mut self, value: u8) {
        match self.kind {
            SpecificInfo_Kind::kS256Load32Transform |
            SpecificInfo_Kind::kS256Load64Transform => {
                unsafe { self.param.splat_index = value as i32; }
            }
            _ => panic!("Invalid kind for set_splat_index"),
        }
    }

    pub fn splat_index(&self) -> i32 {
        match self.kind {
            SpecificInfo_Kind::kS256Load32Transform |
            SpecificInfo_Kind::kS256Load64Transform => unsafe { self.param.splat_index },
            _ => panic!("Invalid kind for splat_index"),
        }
    }

    pub fn set_shufd_control(&mut self, _control: u8) {
        todo!()
    }

    pub fn set_shufps_control(&mut self, _control: u8) {
        todo!()
    }
}

// BundlePackNode is used to represent a i8x16/i16x8 to f32x4 conversion.
// The conversion extracts 4 lanes of i8x16/i16x8 input(base), start from lane
// index(offset), sign/zero(is_sign_extract) extends the extracted lanes to
// i32x4, then converts i32x4/u32x4(is_sign_convert) to f32x4.
#[derive(Debug)]
pub struct BundlePackNode {
    base: PackNode,
    base_: OpIndex,
    offset_: u8,
    lane_size_: u8,
    is_sign_extract_: bool,
    is_sign_convert_: bool,
}

impl BundlePackNode {
    pub fn new(node_group: NodeGroup, base: OpIndex, offset: i8, lane_size: u8, is_sign_extract: bool, is_sign_convert: bool) -> Self {
        BundlePackNode {
            base: PackNode::new(node_group, NodeType::KBundlePackNode),
            base_: base,
            offset_: offset as u8,
            lane_size_: lane_size,
            is_sign_extract_: is_sign_extract,
            is_sign_convert_: is_sign_convert,
        }
    }

    pub fn base(&self) -> OpIndex {
        self.base_
    }

    pub fn offset(&self) -> u8 {
        self.offset_
    }

    pub fn lane_size(&self) -> u8 {
        self.lane_size_
    }

    pub fn is_sign_extract(&self) -> bool {
        self.is_sign_extract_
    }

    pub fn is_sign_convert(&self) -> bool {
        self.is_sign_convert_
    }
}

// An auxillary tree structure with a set of PackNodes based on the Superword
// Level Parallelism (SLP) vectorization technique. The BuildTree method will
// start from a selected root, e.g. a group of consecutive stores, and extend
// through value inputs to create new PackNodes if the inputs are valid, or
// conclude that the current PackNode is a leaf and terminate the tree.
// Below is an example of SLPTree where loads and stores in each PackNode are
// all consecutive.
// [Load0, Load1]  [Load2, Load3]
//           \       /
//          [Add0, Add1]
//                |
//         [Store0, Store1]
#[derive(Debug)]
pub struct SLPTree<'a> {
    graph_: &'a Graph,
    analyzer_: &'a mut WasmRevecAnalyzer,
    phase_zone_: &'a Zone,
    root_: Option<Box<PackNode>>,
    node_to_packnode_: HashMap<OpIndex, Box<PackNode>>,
    node_to_intersect_packnodes_: HashMap<OpIndex, Vec<Box<PackNode>>>,
}

impl<'a> SLPTree<'a> {
    pub const RECURSION_MAX_DEPTH: usize = 1000;

    pub fn new(graph: &'a Graph, analyzer: &'a mut WasmRevecAnalyzer, zone: &'a Zone) -> Self {
        SLPTree {
            graph_: graph,
            analyzer_: analyzer,
            phase_zone_: zone,
            root_: None,
            node_to_packnode_: HashMap::new(),
            node_to_intersect_packnodes_: HashMap::new(),
        }
    }

    // Information for extending i8x16/i16x8 to f32x4
    #[derive(Debug)]
    pub struct ExtendIntToF32x4Info {
        extend_from: OpIndex,
        start_lane: u8,    // 0 or 8
        lane_size: u8,     // 1(i8) or 2(i16)
        is_sign_extract: bool,  // extract_lane_s or extract_lane_u
        is_sign_convert: bool,  // f32x4.convert_i32x4_s or f32x4.convert_i32x4_u
    }

    // Per-lane information for extending i8x16/i16x8 to f32x4
    #[derive(Debug)]
    pub struct LaneExtendInfo {
        extract_from: OpIndex,
        extract_kind: Simd128ExtractLaneOp_Kind,
        extract_lane_index: i32,
        change_kind: ChangeOp_Kind,
        replace_lane_index: i32,
    }

    pub fn build_tree(&mut self, roots: &NodeGroup) -> Option<Box<PackNode>> {
        self.root_ = self.build_tree_rec(roots, 0);
        self.root_.take()
    }

    pub fn delete_tree(&mut self) {
        self.root_ = None;
        self.node_to_packnode_.clear();
        self.node_to_intersect_packnodes_.clear();
    }

    pub fn get_pack_node(&self, node: OpIndex) -> Option<&PackNode> {
        self.node_to_packnode_.get(&node).map(|boxed| &**boxed)
    }

    pub fn get_intersect_packnodes(&self, node: OpIndex) -> Option<&Vec<Box<PackNode>>> {
        self.node_to_intersect_packnodes_.get(&node)
    }

    pub fn get_node_mapping(&mut self) -> &mut HashMap<OpIndex, Box<PackNode>> {
        &mut self.node_to_packnode_
    }

    pub fn get_intersect_node_mapping(&mut self) -> &mut HashMap<OpIndex, Vec<Box<PackNode>>> {
        &mut self.node_to_intersect_packnodes_
    }

    pub fn print(&self, _info: &str) {
        println!("SLPTree print is unimplemented");
    }

    // This is the recursive part of BuildTree.
    fn build_tree_rec(&mut self, node_group: &NodeGroup, depth: usize) -> Option<Box<PackNode>> {
        if depth == Self::RECURSION_MAX_DEPTH {
            return None;
        }

        if !self.can_be_packed(node_group) {
            return None;
        }

        // Check if this is a duplicate of another entry.
        if let Some(pnode) = self.get_pack_node(node_group.get(0)) {
            if pnode.is_same(node_group) {
                return self.node_to_packnode_.get(&node_group.get(0)).cloned();
            }
        }


