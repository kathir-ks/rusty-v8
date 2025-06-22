// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod profile_generator {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use std::{fmt, mem, ptr};
    use std::sync::{Arc, Mutex, MutexGuard, RecursiveMutex, RwLock};

    //use crate::base::platform::time::TimeTicks; // Assuming a similar struct exists in Rust
    //use crate::builtins::builtins::Builtin;  // Assuming a similar enum exists in Rust
    //use crate::execution::vm_state::StateTag; // Assuming a similar enum exists in Rust
    //use crate::logging::code_events::LogEventListener; // Assuming a similar struct exists in Rust
    //use crate::profiler::output_stream_writer::OutputStreamWriter; // Assuming a similar struct exists in Rust
    //use crate::profiler::strings_storage::StringsStorage; // Assuming a similar struct exists in Rust
    //use crate::utils::allocation::Malloced; // Custom allocator (might not be needed in Rust)

    //use v8::{CpuProfileNode, OutputStream}; // Assuming these are v8-rs equivalents
    //use v8::internal::{Address, Isolate, Name, Tagged}; // Assuming these are v8-rs equivalents

    // Placeholder types, replace with actual v8-rs or custom equivalents
    pub type Address = usize; // Or a pointer type, depending on v8-rs
    pub type Isolate = usize; // Replace with actual Isolate type
    pub type Name = String; // Replace with actual Name type
    pub type Tagged<T> = T; // Replace with actual Tagged type
    pub type OutputStream = Vec<u8>; // Replace with actual OutputStream type

    pub const K_NO_LINE_NUMBER_INFO: i32 = 0; //v8::CpuProfileNode::kNoLineNumberInfo;
    pub const K_HEAP_OBJECT_TAG: Address = 1;
    pub const K_NULL_ADDRESS: Address = 0;

    #[derive(Debug, Default)]
    pub struct SourcePositionTable {
        pc_offsets_to_lines_: Vec<SourcePositionTuple>,
    }

    impl SourcePositionTable {
        pub fn new() -> Self {
            SourcePositionTable::default()
        }

        pub fn set_position(&mut self, pc_offset: i32, line: i32, inlining_id: i32) {
            let tuple = SourcePositionTuple {
                pc_offset,
                line_number: line,
                inlining_id,
            };
            self.pc_offsets_to_lines_.push(tuple);
            self.pc_offsets_to_lines_.sort();
        }

        pub fn get_source_line_number(&self, pc_offset: i32) -> i32 {
            if let Ok(index) = self.pc_offsets_to_lines_.binary_search_by(|probe| probe.pc_offset.cmp(&pc_offset)) {
                self.pc_offsets_to_lines_[index].line_number
            } else {
                K_NO_LINE_NUMBER_INFO //v8::CpuProfileNode::kNoLineNumberInfo
            }
        }

        pub fn get_inlining_id(&self, pc_offset: i32) -> i32 {
            if let Ok(index) = self.pc_offsets_to_lines_.binary_search_by(|probe| probe.pc_offset.cmp(&pc_offset)) {
                self.pc_offsets_to_lines_[index].inlining_id
            } else {
                0 // Or appropriate default
            }
        }

        pub fn size(&self) -> usize {
            self.pc_offsets_to_lines_.len()
        }

        pub fn print(&self) {
            println!("SourcePositionTable: {:?}", self.pc_offsets_to_lines_);
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    struct SourcePositionTuple {
        pc_offset: i32,
        line_number: i32,
        inlining_id: i32,
    }

    #[derive(Debug)]
    pub struct CodeEntry {
        bit_field_: u32,
        ref_count_: AtomicUsize,
        name_: String,
        resource_name_: String,
        line_number_: i32,
        column_number_: i32,
        script_id_: i32,
        position_: i32,
        line_info_: Option<Box<SourcePositionTable>>,
        rare_data_: Option<Box<RareData>>,
        instruction_start_: Address,
        heap_object_location_: Option<Address>, // Changed to Option<Address>
    }

    impl Drop for CodeEntry {
        fn drop(&mut self) {
            // No alive handles should be associated with the CodeEntry at time of
            // destruction.
            if self.heap_object_location_.is_some() {
                eprintln!("Heap object location is not none when dropping");
            }
            if self.ref_count_.load(Ordering::Relaxed) != 0 {
                eprintln!("Ref count is not zero when dropping.");
            }
            //DCHECK(!heap_object_location_);
            //DCHECK_EQ(ref_count_, 0UL);
        }
    }

    impl CodeEntry {
        pub const K_EMPTY_RESOURCE_NAME: &'static str = "";
        pub const K_EMPTY_BAILOUT_REASON: &'static str = "";
        pub const K_NO_DEOPT_REASON: &'static str = "";
        pub const K_PROGRAM_ENTRY_NAME: &'static str = "(program)";
        pub const K_IDLE_ENTRY_NAME: &'static str = "(idle)";
        pub const K_GARBAGE_COLLECTOR_ENTRY_NAME: &'static str = "(garbage collector)";
        pub const K_UNRESOLVED_FUNCTION_NAME: &'static str = "(unresolved)";
        pub const K_ROOT_ENTRY_NAME: &'static str = "(root)";

        pub fn new(
            tag: LogEventListenerCodeTag,
            name: &str,
            resource_name: &str,
            line_number: i32,
            column_number: i32,
            line_info: Option<Box<SourcePositionTable>>,
            is_shared_cross_origin: bool,
            code_type: CodeType,
        ) -> Self {
            let mut bit_field_: u32 = 0;

            bit_field_ = CodeTypeField::update(bit_field_, code_type);
            bit_field_ = SharedCrossOriginField::update(bit_field_, is_shared_cross_origin);
            bit_field_ = CodeTagField::update(bit_field_, tag);

            CodeEntry {
                bit_field_: bit_field_,
                ref_count_: AtomicUsize::new(0),
                name_: name.to_string(),
                resource_name_: resource_name.to_string(),
                line_number_: line_number,
                column_number_: column_number,
                script_id_: 0,
                position_: 0,
                line_info_: line_info,
                rare_data_: None,
                instruction_start_: K_NULL_ADDRESS,
                heap_object_location_: None,
            }
        }

        pub fn name(&self) -> &str {
            &self.name_
        }

        pub fn resource_name(&self) -> &str {
            &self.resource_name_
        }

        pub fn line_number(&self) -> i32 {
            self.line_number_
        }

        pub fn column_number(&self) -> i32 {
            self.column_number_
        }

        pub fn line_info(&self) -> Option<&SourcePositionTable> {
            self.line_info_.as_deref()
        }

        pub fn script_id(&self) -> i32 {
            self.script_id_
        }

        pub fn set_script_id(&mut self, script_id: i32) {
            self.script_id_ = script_id;
        }

        pub fn position(&self) -> i32 {
            self.position_
        }

        pub fn set_position(&mut self, position: i32) {
            self.position_ = position;
        }

        pub fn set_bailout_reason(&mut self, bailout_reason: &str) {
            self.ensure_rare_data().bailout_reason_ = bailout_reason.to_string();
        }

        pub fn bailout_reason(&self) -> &str {
            if let Some(rare_data) = &self.rare_data_ {
                &rare_data.bailout_reason_
            } else {
                CodeEntry::K_EMPTY_BAILOUT_REASON
            }
        }

        pub fn set_deopt_info(&mut self, deopt_reason: &str, deopt_id: i32, inlined_frames: Vec<CpuProfileDeoptFrame>) {
            let rare_data = self.ensure_rare_data();
            rare_data.deopt_reason_ = deopt_reason.to_string();
            rare_data.deopt_id_ = deopt_id;
            rare_data.deopt_inlined_frames_ = inlined_frames;
        }

        pub fn estimated_size(&self) -> usize {
            mem::size_of_val(self) + self.name_.len() + self.resource_name_.len()
        }

        pub fn get_deopt_info(&self) -> CpuProfileDeoptInfo {
            if let Some(rare_data) = &self.rare_data_ {
                CpuProfileDeoptInfo {
                    deopt_reason: rare_data.deopt_reason_.clone(),
                    deopt_id: rare_data.deopt_id_,
                    inlined_frames: rare_data.deopt_inlined_frames_.clone(),
                }
            } else {
                CpuProfileDeoptInfo {
                    deopt_reason: CodeEntry::K_NO_DEOPT_REASON.to_string(),
                    deopt_id: K_NO_DEOPTIMIZATION_ID,
                    inlined_frames: Vec::new(),
                }
            }
        }

        pub fn has_deopt_info(&self) -> bool {
            self.rare_data_.as_ref().map_or(false, |rd| rd.deopt_id_ != K_NO_DEOPTIMIZATION_ID)
        }

        pub fn clear_deopt_info(&mut self) {
            if let Some(rare_data) = &mut self.rare_data_ {
                rare_data.deopt_reason_ = CodeEntry::K_NO_DEOPT_REASON.to_string();
                rare_data.deopt_id_ = K_NO_DEOPTIMIZATION_ID;
            }
        }

        pub fn code_type_string(&self) -> &'static str {
            match CodeTypeField::decode(self.bit_field_) {
                CodeType::JS => "JS",
                CodeType::WASM => "wasm",
                CodeType::OTHER => "other",
            }
        }

        pub fn instruction_start(&self) -> Address {
            self.instruction_start_
        }

        pub fn set_instruction_start(&mut self, address: Address) {
            self.instruction_start_ = address;
        }

        pub fn heap_object_location_address(&mut self) -> &mut Option<Address> {
            &mut self.heap_object_location_
        }

        pub fn fill_function_info(&mut self, shared: Tagged<SharedFunctionInfo>) {
            // TODO: Implement FillFunctionInfo
            // This likely involves interacting with the v8-rs API to extract information
            // from the SharedFunctionInfo.
            // For now, leave it empty.
        }

        pub fn set_builtin_id(&mut self, id: Builtin) {
            self.bit_field_ = BuiltinField::update(self.bit_field_, id);
        }

        pub fn builtin(&self) -> Builtin {
            BuiltinField::decode(self.bit_field_)
        }

        pub fn is_shared_cross_origin(&self) -> bool {
            SharedCrossOriginField::decode(self.bit_field_)
        }

        pub fn is_ref_counted(&self) -> bool {
            RefCountedField::decode(self.bit_field_)
        }

        pub fn get_hash(&self) -> u32 {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            std::hash::Hash::hash(&self.name_, &mut hasher);
            std::hash::Hash::hash(&self.resource_name_, &mut hasher);
            std::hash::Hash::hash(&self.line_number_, &mut hasher);
            std::hash::Hash::hash(&self.column_number_, &mut hasher);
            hasher.finish() as u32
        }

        pub fn is_same_function_as(&self, entry: &CodeEntry) -> bool {
            self.name_ == entry.name_
                && self.resource_name_ == entry.resource_name_
                && self.line_number_ == entry.line_number_
                && self.column_number_ == entry.column_number_
        }

        pub fn get_source_line(&self, pc_offset: i32) -> i32 {
            if let Some(line_info) = &self.line_info_ {
                line_info.get_source_line_number(pc_offset)
            } else {
                self.line_number_
            }
        }

        pub fn set_inline_stacks(
            &mut self,
            inline_entries: HashSet<*mut CodeEntry>,
            inline_stacks: HashMap<i32, Vec<CodeEntryAndLineNumber>>,
        ) {
            let rare_data = self.ensure_rare_data();
            rare_data.inline_entries_ = inline_entries;
            rare_data.inline_stacks_ = inline_stacks;
        }

        pub fn get_inline_stack(&self, pc_offset: i32) -> Option<&Vec<CodeEntryAndLineNumber>> {
            self.rare_data_
                .as_ref()
                .and_then(|rare_data| rare_data.inline_stacks_.get(&pc_offset))
        }

        pub fn event(&self) -> LogEventListenerEvent {
            EventField::decode(self.bit_field_)
        }

        pub fn code_tag(&self) -> LogEventListenerCodeTag {
            CodeTagField::decode(self.bit_field_)
        }

        // Static CodeEntry instances. Since static mut is unsafe, and
        // Rust does not have static initialization like C++, lazy_static! macro
        // from the lazy_static crate is one safe way to do static initialization.
        // However, since Rust's ownership rules are different, this may not be
        // the most idiomatic way to handle these. Consider using a more Rust-like
        // approach with global immutable data or thread-local storage if possible.
        // For now, we will use a lazy_static with Mutex.

        //TODO:: Implement the proper static entry
        pub fn program_entry() -> &'static CodeEntry {
            todo!()
        }
        pub fn idle_entry() -> &'static CodeEntry {
            todo!()
        }
        pub fn gc_entry() -> &'static CodeEntry {
            todo!()
        }
        pub fn unresolved_entry() -> &'static CodeEntry {
            todo!()
        }
        pub fn root_entry() -> &'static CodeEntry {
            todo!()
        }

        pub fn release_strings(&mut self, strings: &mut StringsStorage) {
            strings.release_string(&self.name_);
            strings.release_string(&self.resource_name_);
            if let Some(rare_data) = &mut self.rare_data_ {
                rare_data.deopt_reason_ = "".to_string();
                rare_data.bailout_reason_ = "".to_string();
            }
        }

        pub fn print(&self) {
            println!("CodeEntry {{");
            println!("  name_: {}", self.name_);
            println!("  resource_name_: {}", self.resource_name_);
            println!("  line_number_: {}", self.line_number_);
            println!("  column_number_: {}", self.column_number_);
            println!("  script_id_: {}", self.script_id_);
            println!("  position_: {}", self.position_);
            println!("  instruction_start_: {}", self.instruction_start_);
            println!("  heap_object_location_: {:?}", self.heap_object_location_);
            println!("}}");
        }

        fn ensure_rare_data(&mut self) -> &mut RareData {
            if self.rare_data_.is_none() {
                self.rare_data_ = Some(Box::new(RareData::default()));
            }
            self.rare_data_.as_mut().unwrap()
        }

        fn mark_ref_counted(&mut self) {
            self.bit_field_ = RefCountedField::update(self.bit_field_, true);
            self.ref_count_.store(1, Ordering::Relaxed);
        }

        fn add_ref(&self) -> usize {
            if !self.is_ref_counted() {
                panic!("Attempted to increment ref count on non-ref-counted CodeEntry");
            }
            self.ref_count_.fetch_add(1, Ordering::Relaxed) + 1
        }

        fn dec_ref(&self) -> usize {
            if !self.is_ref_counted() {
                panic!("Attempted to decrement ref count on non-ref-counted CodeEntry");
            }

            let prev = self.ref_count_.fetch_sub(1, Ordering::Relaxed);
            if prev == 1 {
                // Last reference is gone; clean up if needed
                // For example, drop internal data
            }
            prev - 1
        }
    }

    #[derive(Debug, Default)]
    struct RareData {
        deopt_reason_: String,
        bailout_reason_: String,
        deopt_id_: i32,
        inline_stacks_: HashMap<i32, Vec<CodeEntryAndLineNumber>>,
        inline_entries_: HashSet<*mut CodeEntry>,
        deopt_inlined_frames_: Vec<CpuProfileDeoptFrame>,
    }

    #[derive(Debug, Clone)]
    pub struct CpuProfileDeoptInfo {
        deopt_reason: String,
        deopt_id: i32,
        inlined_frames: Vec<CpuProfileDeoptFrame>,
    }

    #[derive(Debug, Clone)]
    pub struct CpuProfileDeoptFrame {
        // TODO: Define fields as needed, based on the C++ CpuProfileDeoptFrame structure
        // Example:
        // function_name: String,
        // line_number: i32,
    }

    pub type LogEventListenerEvent = u32;
    pub type LogEventListenerCodeTag = u32;
    pub type Builtin = u32;

    mod base {
        pub mod BitField {
            pub trait BitField<T, const OFFSET: usize, const LENGTH: usize> {
                fn update(value: u32, new_value: T) -> u32;
                fn decode(value: u32) -> T;
            }

            macro_rules! implement_bitfield {
                ($type:ty, $offset:expr, $length:expr) => {
                    impl BitField<$type, {$offset}, {$length}> for $type {
                        fn update(value: u32, new_value: $type) -> u32 {
                            let mask: u32 = ((1 << $length) - 1) << $offset;
                            let shifted_new_value: u32 = (new_value as u32) << $offset;
                            (value & !mask) | (shifted_new_value & mask)
                        }

                        fn decode(value: u32) -> $type {
                            let mask: u32 = ((1 << $length) - 1) << $offset;
                            ((value & mask) >> $offset) as $type
                        }
                    }
                };
            }

            implement_bitfield!(u32, 0, 4);
            implement_bitfield!(bool, 28, 1);
        }
    }

    use base::BitField::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CodeType {
        JS,
        WASM,
        OTHER,
    }

    impl From<u32> for CodeType {
        fn from(value: u32) -> Self {
            match value {
                0 => CodeType::JS,
                1 => CodeType::WASM,
                _ => CodeType::OTHER,
            }
        }
    }

    implement_bitfield!(CodeType, 29, 2);
    implement_bitfield!(LogEventListenerCodeTag, 0, 4);
    implement_bitfield!(Builtin, 8, 20);
    implement_bitfield!(bool, 31, 1);
    implement_bitfield!(LogEventListenerEvent, 0, 4);

    pub const K_NO_DEOPTIMIZATION_ID: i32 = -1;

    #[derive(Debug, Clone)]
    pub struct CodeEntryAndLineNumber {
        pub code_entry: *mut CodeEntry, // Raw pointer
        pub line_number: i32,
    }

    #[derive(Debug)]
    pub type ProfileStackTrace = Vec<CodeEntryAndLineNumber>;

    #[derive(Debug)]
    pub struct ContextFilter {
        native_context_address_: Address,
    }

    impl ContextFilter {
        pub fn new(native_context_address: Address) -> Self {
            ContextFilter {
                native_context_address_: native_context_address,
            }
        }

        pub fn on_move_event(&mut self, from_address: Address, to_address: Address) {
            if self.native_context_address_ == from_address {
                self.native_context_address_ = to_address;
            }
        }

        pub fn accept(&self, native_context_address: Address) -> bool {
            if self.native_context_address_ == K_NULL_ADDRESS {
                return true;
            }
            (native_context_address & !K_HEAP_OBJECT_TAG) == native_context_address
        }

        pub fn set_native_context_address(&mut self, address: Address) {
            self.native_context_address_ = address;
        }

        pub fn native_context_address(&self) -> Address {
            self.native_context_address_
        }
    }

    #[derive(Debug)]
    pub struct ProfileNode {
        tree_: *mut ProfileTree, // Raw pointer to avoid ownership issues
        entry_: *mut CodeEntry,   // Raw pointer to CodeEntry
        self_ticks_: u32,
        children_: HashMap<CodeEntryAndLineNumber, *mut ProfileNode>,
        line_number_: i32,
        children_list_: Vec<*mut ProfileNode>,
        parent_: *mut ProfileNode,   // Raw pointer
        id_: u32,
        line_ticks_: HashMap<i32, i32>,
        deopt_infos_: Vec<CpuProfileDeoptInfo>,
    }

    impl ProfileNode {
        pub fn new(tree: *mut ProfileTree, entry: *mut CodeEntry, parent: *mut ProfileNode, line_number: i32) -> Self {
            unsafe {
                let tree_ref = &mut *tree;
                ProfileNode {
                    tree_: tree,
                    entry_: entry,
                    self_ticks_: 0,
                    children_: HashMap::new(),
                    line_number_: if line_number != 0 { line_number } else { (*entry).line_number() },
                    children_list_: Vec::new(),
                    parent_: parent,
                    id_: tree_ref.next_node_id(),
                    line_ticks_: HashMap::new(),
                    deopt_infos_: Vec::new(),
                }
            }
        }
    }

    impl Drop for ProfileNode {
        fn drop(&mut self) {
            // Drop all children nodes
            for child in self.children_list_.iter() {
                unsafe {
                    drop(Box::from_raw(*child));
                }
            }
        }
    }

    impl ProfileNode {
        pub fn find_child(&self, entry: *mut CodeEntry, line_number: i32) -> Option<*mut ProfileNode> {
            let key = CodeEntryAndLineNumber {
                code_entry: entry,
                line_number: if line_number != K_NO_LINE_NUMBER_INFO { line_number } else { unsafe { (*entry).line_number() } },
            };
            self.children_.get(&key).copied()
        }

        pub fn find_or_add_child(&mut self, entry: *mut CodeEntry, line_number: i32) -> *mut ProfileNode {
            let key = CodeEntryAndLineNumber {
                code_entry: entry,
                line_number: line_number,
            };
            if let Some(child) = self.children_.get(&key).copied() {
                return child;
            }

            unsafe {
                let tree_ref = &mut *(self.tree_);
                let new_child = Box::new(ProfileNode::new(self.tree_, entry, self as *mut ProfileNode, line_number));
                let new_child_ptr = Box::into_raw(new_child);

                self.children_.insert(key, new_child_ptr);
                self.children_list_.push(new_child_ptr);
                new_child_ptr
            }
        }

        pub fn increment_self_ticks(&mut self) {
            self.self_ticks_ += 1;
        }

        pub fn increase_self_ticks(&mut self, amount: u32) {
            self.self_ticks_ += amount;
        }

        pub fn increment_line_ticks(&mut self, src_line: i32) {
            *self.line_ticks_.entry(src_line).or_insert(0) += 1;
        }

        pub fn entry(&self) -> *mut CodeEntry {
            self.entry_
        }

        pub fn self_ticks(&self) -> u32 {
            self.self_ticks_
        }

        pub fn children(&self) -> &Vec<*mut ProfileNode> {
            &self.children_list_
        }

        pub fn id(&self) -> u32 {
            self.id_
        }

        pub fn parent(&self) -> *mut ProfileNode {
            self.parent_
        }

        pub fn line_number(&self) -> i32 {
            if self.line_number_ != 0 {
                self.line_number_
            } else {
                unsafe { (*self.entry_).line_number() }
            }
        }

        pub fn source_type(&self) -> CpuProfileNodeSourceType {
            CpuProfileNodeSourceType::JavaScript // Placeholder
        }

        pub fn get_hit_line_count(&self) -> u32 {
            self.line_ticks_.len() as u32
        }

        pub fn get_line_ticks(&self, entries: &mut [CpuProfileNodeLineTick]) -> bool {
            if entries.len() < self.line_ticks_.len() {
                return false;
            }

            let mut i = 0;
            for (&line_number, &ticks) in &self.line_ticks_ {
                entries[i].line = line_number;
                entries[i].ticks = ticks as u32;
                i += 1;
            }

            true
        }

        pub fn collect_deopt_info(&mut self, entry: *mut CodeEntry) {
            unsafe {
                if (*entry).has_deopt_info() {
                    self.deopt_infos_.push((*entry).get_deopt_info());
                }
            }
        }

        pub fn deopt_infos(&self) -> &Vec<CpuProfileDeoptInfo> {
            &self.deopt_infos_
        }

        pub fn isolate(&self) -> Isolate {
            unsafe {
                (&*self.tree_).isolate()
            }
        }

        pub fn print(&self, indent: i32) const {
            let indent_str = " ".repeat(indent as usize);
            unsafe {
                println!("{}ProfileNode {{", indent_str);
                println!("{}  id: {}", indent_str, self.id_);
                println!("{}  name: {}", indent_str, (&*self.entry_).name());
                println!("{}  self_ticks: {}", indent_str, self.self_ticks_);
                println!("{}  line_number: {}", indent_str, self.line_number());
                println!("{}  children: [", indent_str);
                for child in &self.children_list_ {
                    (&**child).print(indent + 2);
                }
                println!("{}  ]", indent_str);
                println!("{}}}", indent_str);
            }
        }
    }

    #[derive(Debug)]
    pub struct ProfileTree {
        pending_nodes_: Vec<*const ProfileNode>,
        next_node_id_: u32,
        isolate_: Isolate,
        code_entries_: *mut CodeEntryStorage,
        root_: Box<ProfileNode>,
    }

    impl ProfileTree {
        pub fn new(isolate: Isolate, storage: *mut CodeEntryStorage) -> Self {
            unsafe {
                let root_entry = (*storage).create_program_entry();
                let root_node = Box::new(ProfileNode::new(ptr::null_mut(), root_entry, ptr::null_mut(), 0));
                let root_node_ptr = Box::into_raw(root_node);

                ProfileTree {
                    pending_nodes_: Vec::new(),
                    next_node_id_: 1,
                    isolate_: isolate,
                    code_entries_: storage,
                    root_: Box::from_raw(root_node_ptr),
                }
            }
        }
    }

    impl Drop for ProfileTree {
        fn drop(&mut self) {
            unsafe {
                drop(Box::from_raw(self.root_.as_mut() as *mut ProfileNode));
            }
        }
    }

    impl ProfileTree {
        pub fn add_path_from_end(
            &mut self,
            path: &Vec<*mut CodeEntry>,
            src_line: i32,
            update_stats: bool,
        ) -> *mut ProfileNode {
            let mut current = self.root_.as_mut() as *mut ProfileNode;
            for &entry in path.iter().rev() {
                unsafe {
                    current = (&mut *current).find_or_add_child(entry, src_line);
                    if update_stats {
                        (&mut *current).increment_self_ticks();
                    }
                }
            }
            current
        }

        pub fn add_path_from_end_stack(
            &mut self,
            path: &ProfileStackTrace,
            src_line: i32,
            update_stats: bool,
            mode: ProfilingMode,
        ) -> *mut ProfileNode {
            let mut current = self.root_.as_mut() as *mut ProfileNode;
            for frame in path.iter().rev() {
                unsafe {
                    let line_number = match mode {
                        ProfilingMode::kLeafNodeLineNumbers => src_line,
                        _ => frame.line_number,
                    };
                    current = (&mut *current).find_or_add_child(frame.code_entry, line_number);
                    if update_stats {
                        (&mut *current).increment_self_ticks();
                    }
                    if mode == ProfilingMode::kAllNodesLineNumbers && frame.line_number != 0 {
                        (&mut *current).increment_line_ticks(frame.line_number);
                    }
                }
            }
            current
        }

        pub fn root(&self) -> &ProfileNode {
            self.root_.as_ref()
        }

        pub fn next_node_id(&mut self) -> u32 {
            self.next_node_id_ += 1;
            self.next_node_id_ - 1
        }

        pub fn print(&self) const {
            self.root_.print(0);
        }

        pub fn isolate(&self) -> Isolate {
            self.isolate_
        }

        pub fn enqueue_node(&mut self, node: *const ProfileNode) {
            self.pending_nodes_.push(node);
        }

        pub fn pending_nodes_count(&self) -> usize {
            self.pending_nodes_.len()
        }

        pub fn take_pending_nodes(&mut self) -> Vec<*const ProfileNode> {
            std::mem::take(&mut self.pending_nodes_)
        }

        pub fn code_entries(&self) -> *mut CodeEntryStorage {
            self.code_entries_
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ProfilingMode {
        kNoLineNumbers,
        kLeafNodeLineNumbers,
        kAllNodesLineNumbers,
    }

    pub struct CpuProfiler;

    #[derive(Debug)]
    pub struct CpuProfile {
        title_: String,
        options_: CpuProfilingOptions,
        delegate_: Option<Box<DiscardedSamplesDelegate>>,
        context_filter_: ContextFilter,
        start_time_: TimeTicks,
        end_time_: TimeTicks,
        samples_: Vec<SampleInfo>,
        top_down_: ProfileTree,
        profiler_: *mut CpuProfiler, // Raw pointer
        streaming_next_sample_: usize,
        id_: ProfilerId,
        next_sample_delta_: TimeDelta,
    }