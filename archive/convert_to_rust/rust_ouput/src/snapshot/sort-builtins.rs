// Converted from V8 C++ source files:
// Header: sort-builtins.h
// Implementation: sort-builtins.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/snapshot/sort-builtins.h

use std::collections::HashMap;
use std::vec::Vec;

use crate::Builtin;

pub struct CallProbability {
    pub incoming_: i32,
    pub outgoing_: i32,
}

impl CallProbability {
    pub fn new(incoming: i32, outgoing: i32) -> Self {
        CallProbability {
            incoming_: incoming,
            outgoing_: outgoing,
        }
    }
}

pub type CallProbabilities = HashMap<Builtin, CallProbability>;
pub type CallGraph = HashMap<Builtin, CallProbabilities>;
pub type BuiltinDensityMap = HashMap<Builtin, u32>;
pub type BuiltinSize = Vec<u32>;
pub type BuiltinClusterMap = HashMap<Builtin, *mut Cluster>;

pub struct BuiltinsSorter {
    k_min_edge_probability_threshold: i32,
    k_max_cluster_size: u32,
    k_max_density_decrease_threshold: u32,

    k_builtin_call_block_density_marker: String,
    k_builtin_density_marker: String,

    clusters_: Vec<*mut Cluster>,
    builtin_density_order_: Vec<BuiltinDensitySlot>,
    call_graph_: CallGraph,
    builtin_density_map_: BuiltinDensityMap,
    builtin_size_: BuiltinSize,
    builtin_cluster_map_: BuiltinClusterMap,
}

struct BuiltinDensitySlot {
    density_: u32,
    builtin_: Builtin,
}

impl BuiltinDensitySlot {
    pub fn new(density: u32, builtin: Builtin) -> Self {
        BuiltinDensitySlot {
            density_: density,
            builtin_: builtin,
        }
    }
}

impl BuiltinsSorter {
    pub fn new() -> Self {
        BuiltinsSorter {
            k_min_edge_probability_threshold: 10,
            k_max_cluster_size: 1 * 1024 * 1024,
            k_max_density_decrease_threshold: 8,
            k_builtin_call_block_density_marker: "block_count".to_string(),
            k_builtin_density_marker: "builtin_count".to_string(),
            clusters_: Vec::new(),
            builtin_density_order_: Vec::new(),
            call_graph_: HashMap::new(),
            builtin_density_map_: HashMap::new(),
            builtin_size_: Vec::new(),
            builtin_cluster_map_: HashMap::new(),
        }
    }

    pub fn sort_builtins(
        &mut self,
        profiling_file: &str,
        builtin_size: &Vec<u32>,
    ) -> Vec<Builtin> {
        self.initialize_call_graph(profiling_file, builtin_size);
        self.initialize_clusters();
        self.merge_best_predecessors();
        self.sort_clusters();

        let mut processed_builtins: std::collections::HashSet<Builtin> =
            std::collections::HashSet::new();
        let mut builtin_order: Vec<Builtin> = Vec::new();

        unsafe {
            for i in 0..self.clusters_.len() {
                let cls = self.clusters_[i];
                for j in 0..(*cls).targets_.len() {
                    let builtin = (*cls).targets_[j];
                    add_builtin_if_not_processed(
                        builtin,
                        &mut builtin_order,
                        &mut processed_builtins,
                    );
                }
            }
        }

        for i in Builtin::kFirst..=Builtin::kLast {
            add_builtin_if_not_processed(i, &mut builtin_order, &mut processed_builtins);
        }

        builtin_order
    }

    fn initialize_call_graph(&mut self, profiling_file: &str, size: &Vec<u32>) {
        use std::fs::File;
        use std::io::{self, BufRead};
        use std::path::Path;
        use std::str::FromStr;

        let mut name2id: HashMap<String, Builtin> = HashMap::new();
        for i in Builtin::kFirst..=Builtin::kLast {
            let name = Builtins::name(i);
            name2id.insert(name.clone(), i);
            self.builtin_size_.push(size[i as usize]);
        }

        if let Ok(lines) = BuiltinsSorter::read_lines(profiling_file) {
            for line in lines {
                if let Ok(line) = line {
                    let mut tokens = line.split(',').map(|s| s.to_string()).collect::<Vec<_>>();
                    if tokens.is_empty() {
                        continue;
                    }

                    let token = tokens.remove(0);

                    if token == self.k_builtin_call_block_density_marker {
                        self.process_block_count_line_info(tokens, &mut name2id);
                    } else if token == self.k_builtin_density_marker {
                        self.process_builtin_density_line_info(tokens, &mut name2id);
                    }
                }
            }
        }
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    fn initialize_clusters(&mut self) {
        for i in 0..self.builtin_size_.len() as u32 {
            let id = Builtins::from_int(i as i32);
            let kind = Builtins::kind_of(id);

            if kind == Builtins::Kind::ASM || kind == Builtins::Kind::CPP {
                if let Some(&density) = self.builtin_density_map_.get(&id) {
                    assert_eq!(density, 0);
                }
                continue;
            }

            unsafe {
                let cls = Box::into_raw(Box::new(Cluster::new(
                    self.builtin_density_map_[&id],
                    self.builtin_size_[i as usize],
                    id,
                    self,
                )));
                self.clusters_.push(cls);
            }
            self.builtin_density_order_.push(BuiltinDensitySlot::new(
                self.builtin_density_map_[&id],
                id,
            ));
        }

        self.builtin_density_order_.sort_by(|x, y| y.density_.cmp(&x.density_));
    }

    fn find_best_predecessor_of(&mut self, callee: Builtin) -> Builtin {
        let mut best_pred = Builtin::kNoBuiltinId;
        let mut best_prob = 0;

        for (caller, callees_prob) in &self.call_graph_ {
            if callees_prob.contains_key(&callee) {
                let incoming_prob = callees_prob[&callee].incoming_;
                if incoming_prob == -1 {
                    continue;
                }
                if best_pred == Builtin::kNoBuiltinId || incoming_prob > best_prob {
                    best_pred = *caller;
                    best_prob = incoming_prob;
                }
            }
        }

        if best_prob < self.k_min_edge_probability_threshold
            || best_pred == Builtin::kNoBuiltinId
        {
            return best_pred;
        }

        unsafe {
            let pred_cls = self.builtin_cluster_map_[&best_pred];
            let succ_cls = self.builtin_cluster_map_[&callee];

            if pred_cls == succ_cls {
                return best_pred;
            }

            if (*pred_cls).size_ + (*succ_cls).size_ > self.k_max_cluster_size {
                return best_pred;
            }

            if (*pred_cls).density_ == 0 {
                return best_pred;
            }

            assert!((*pred_cls).size_ > 0);

            let new_density =
                ((*pred_cls).time_approximation() + (*succ_cls).time_approximation())
                    / ((*pred_cls).size_ + (*succ_cls).size_);

            if (*pred_cls).density_ / self.k_max_density_decrease_threshold > new_density as u32 {
                return best_pred;
            }
        }
        best_pred
    }

    fn merge_best_predecessors(&mut self) {
        for i in 0..self.builtin_density_order_.len() {
            let id = self.builtin_density_order_[i].builtin_;
            unsafe {
                let succ_cluster = self.builtin_cluster_map_[&id];
                let best_pred = self.find_best_predecessor_of(id);

                if best_pred != Builtin::kNoBuiltinId {
                    let pred_cluster = self.builtin_cluster_map_[&best_pred];
                    (*pred_cluster).merge(*(&mut *succ_cluster));
                }
            }
        }
    }

    fn sort_clusters(&mut self) {
        self.clusters_.sort_by(|x, y| unsafe {
            (*y).density_.cmp(&(*x).density_)
        });

        self.clusters_.retain(|x| unsafe { !(*x).targets_.is_empty() });
    }

    fn process_block_count_line_info(
        &mut self,
        tokens: Vec<String>,
        name2id: &mut HashMap<String, Builtin>,
    ) {
        if tokens.len() != 3 {
            eprintln!("Unexpected number of tokens in block count line: {:?}", tokens);
            return;
        }

        let caller_name = &tokens[0];
        let caller_id = match name2id.get(caller_name) {
            Some(&id) => id,
            None => {
                eprintln!("Caller name not found: {}", caller_name);
                return;
            }
        };

        let block_id = match tokens[1].parse::<i32>() {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Failed to parse block id: {}", e);
                return;
            }
        };

        let normalized_count = match tokens[2].parse::<i32>() {
            Ok(count) => count,
            Err(e) => {
                eprintln!("Failed to parse normalized count: {}", e);
                return;
            }
        };

        let profiler = BuiltinsCallGraph::get();
        let block_callees = profiler.get_builtin_callees(caller_id);

        if let Some(callees) = block_callees {
            let caller_density = self.builtin_density_map_.get(&caller_id).cloned().unwrap_or(0);

            if let Some(callee_ids) = callees.get(&block_id) {
                for &callee_id in callee_ids {
                    let outgoing_prob = if caller_density != 0 {
                        normalized_count * 100 / caller_density
                    } else {
                        if normalized_count > 0 {
                            100
                        } else {
                            0
                        }
                    };

                    let callee_density = self.builtin_density_map_.get(&callee_id).cloned().unwrap_or(0);

                    let incoming_prob = if callee_density != 0 {
                        normalized_count * 100 / callee_density
                    } else {
                        if normalized_count > 0 {
                            100
                        } else {
                            0
                        }
                    };
                    let probs = CallProbability::new(incoming_prob, outgoing_prob);

                    if !self.call_graph_.contains_key(&caller_id) {
                        self.call_graph_.insert(caller_id, HashMap::new());
                    }

                    if let Some(call_probs) = self.call_graph_.get_mut(&caller_id) {
                        call_probs.insert(callee_id, probs);
                    }
                }
            }
        }
    }

    fn process_builtin_density_line_info(
        &mut self,
        tokens: Vec<String>,
        name2id: &mut HashMap<String, Builtin>,
    ) {
        if tokens.len() != 2 {
            eprintln!("Unexpected number of tokens in builtin density line: {:?}", tokens);
            return;
        }

        let builtin_name = &tokens[0];
        let density = match tokens[1].parse::<u32>() {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Failed to parse density: {}", e);
                return;
            }
        };

        let builtin_id = match name2id.get(builtin_name) {
            Some(&id) => id,
            None => {
                eprintln!("Builtin name not found: {}", builtin_name);
                return;
            }
        };

        self.builtin_density_map_.insert(builtin_id, density);
    }
}

fn add_builtin_if_not_processed(
    builtin: Builtin,
    order: &mut Vec<Builtin>,
    processed_builtins: &mut std::collections::HashSet<Builtin>,
) -> bool {
    if !processed_builtins.contains(&builtin) {
        order.push(builtin);
        processed_builtins.insert(builtin);
        return true;
    }
    false
}

pub struct Cluster {
    density_: u32,
    size_: u32,
    targets_: Vec<Builtin>,
    sorter_: *mut BuiltinsSorter,
}

impl Cluster {
    pub fn new(density: u32, size: u32, target: Builtin, sorter: *mut BuiltinsSorter) -> Self {
        let mut cluster = Cluster {
            density_: density,
            size_: size,
            targets_: Vec::new(),
            sorter_: sorter,
        };
        assert!(size > 0);
        cluster.targets_.push(target);
        unsafe {
            (*sorter).builtin_cluster_map_.insert(target, &mut cluster);
        }
        cluster
    }

    pub fn merge(&mut self, other: &mut Cluster) {
        for builtin in &other.targets_ {
            self.targets_.push(*builtin);
            unsafe {
                (*self.sorter_).builtin_cluster_map_.insert(*builtin, self);
            }
        }
        self.density_ = ((self.time_approximation() + other.time_approximation())
            / (self.size_ + other.size_)) as u32;
        self.size_ += other.size_;
        other.density_ = 0;
        other.size_ = 0;
        other.targets_.clear();
    }

    pub fn time_approximation(&self) -> u64 {
        self.size_ as u64 * self.density_ as u64
    }
}

// Dummy implementations
struct BuiltinsCallGraph {}

impl BuiltinsCallGraph {
    pub fn get() -> BuiltinsCallGraph {
        BuiltinsCallGraph {}
    }
    pub fn get_builtin_callees(&self, _builtin: Builtin) -> Option<HashMap<i32, Vec<Builtin>>> {
        Some(HashMap::new())
    }
}

struct Builtins {}

impl Builtins {
    pub const kFirst: Builtin = Builtin(0);
    pub const kLast: Builtin = Builtin(10);
    pub const kNoBuiltinId: Builtin = Builtin(-1);

    pub fn name(builtin: Builtin) -> String {
        format!("Builtin_{}", builtin.0)
    }

    pub fn kind_of(_builtin: Builtin) -> BuiltinsKind {
        BuiltinsKind::CPP
    }

    pub fn from_int(i: i32) -> Builtin {
        Builtin(i)
    }
}

#[derive(PartialEq, Eq)]
enum BuiltinsKind {
    ASM,
    CPP,
}
