// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod constant_pool {
    use std::collections::HashMap;
    use std::fmt;
    use std::mem;

    pub mod base {
        pub mod numbers {
            #[derive(Clone, Copy)]
            pub struct Double {
                value: f64,
            }

            impl Double {
                pub fn new(value: f64) -> Self {
                    Double { value }
                }

                pub fn as_f64(&self) -> f64 {
                    self.value
                }

                pub fn as_u64(&self) -> u64 {
                    self.value.to_bits()
                }

                // Implement From<f64> for Double
                impl From<f64> for Double {
                    fn from(value: f64) -> Self {
                        Double::new(value)
                    }
                }
            }
        }
    }

    pub mod codegen {
        pub struct Label {
            // Placeholder, implement as needed
            used: bool,
            near_used: bool,
        }

        impl Label {
            pub fn new() -> Self {
                Label { used: false, near_used: false }
            }

            pub fn unuse(&mut self) {
                self.used = false;
            }

            pub fn unuse_near(&mut self) {
                self.near_used = false;
            }
        }

        impl Default for Label {
            fn default() -> Self {
                Self::new()
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum RelocInfoMode {
            NO_INFO,
            CODE_TARGET,
            CONST_POOL,
            VENEER_POOL,
            DEOPT_SCRIPT_OFFSET,
            DEOPT_INLINING_ID,
            DEOPT_REASON,
            DEOPT_ID,
            DEOPT_NODE_ID,
            EMBEDDED_OBJECT,
            EXTERNAL_REFERENCE,
            // Add other modes as needed
        }

        impl RelocInfoMode {
            pub fn is_embedded_object_mode(mode: RelocInfoMode) -> bool {
                match mode {
                    RelocInfoMode::EMBEDDED_OBJECT => true,
                    _ => false,
                }
            }

            pub fn is_shareable_reloc_mode(mode: RelocInfoMode) -> bool {
                match mode {
                    RelocInfoMode::EXTERNAL_REFERENCE => true,
                    _ => false,
                }
            }
        }
    }

    use self::base::numbers::Double;
    use self::codegen::Label;
    use self::codegen::RelocInfoMode;

    // Placeholder for Instruction
    pub struct Instruction {}

    const K_SYSTEM_POINTER_SIZE: usize = mem::size_of::<usize>();
    const K_DOUBLE_SIZE: usize = mem::size_of::<f64>();

    #[derive(Clone, Copy)]
    pub struct ConstantPoolEntry {
        position_: i32,
        merged_index_: i32,
        value_: i64,  // Assuming intptr_t is i64
        value64_: u64,
        rmode_: RelocInfoMode,
    }

    impl ConstantPoolEntry {
        pub fn new_default() -> Self {
            ConstantPoolEntry {
                position_: 0,
                merged_index_: Self::SHARING_PROHIBITED,
                value_: 0,
                value64_: 0,
                rmode_: RelocInfoMode::NO_INFO,
            }
        }

        pub fn new(position: i32, value: i64, sharing_ok: bool, rmode: RelocInfoMode) -> Self {
            ConstantPoolEntry {
                position_: position,
                merged_index_: if sharing_ok { Self::SHARING_ALLOWED } else { Self::SHARING_PROHIBITED },
                value_: value,
                value64_: 0,
                rmode_: rmode,
            }
        }

        pub fn new_double(position: i32, value: Double, rmode: RelocInfoMode) -> Self {
            ConstantPoolEntry {
                position_: position,
                merged_index_: Self::SHARING_ALLOWED,
                value_: 0,
                value64_: value.as_u64(),
                rmode_: rmode,
            }
        }

        pub fn position(&self) -> i32 {
            self.position_
        }

        pub fn sharing_ok(&self) -> bool {
            self.merged_index_ != Self::SHARING_PROHIBITED
        }

        pub fn is_merged(&self) -> bool {
            self.merged_index_ >= 0
        }

        pub fn merged_index(&self) -> i32 {
            if !self.is_merged() {
                panic!("Check failed: is_merged()");
            }
            self.merged_index_
        }

        pub fn set_merged_index(&mut self, index: i32) {
            if !self.sharing_ok() {
                panic!("Check failed: sharing_ok()");
            }
            self.merged_index_ = index;
            if !self.is_merged() {
                panic!("Check failed: is_merged()");
            }
        }

        pub fn offset(&self) -> i32 {
            if self.merged_index_ < 0 {
                panic!("Check failed: merged_index_ >= 0");
            }
            self.merged_index_
        }

        pub fn set_offset(&mut self, offset: i32) {
            if offset < 0 {
                panic!("Check failed: offset >= 0");
            }
            self.merged_index_ = offset;
        }

        pub fn value(&self) -> i64 {
            self.value_
        }

        pub fn value64(&self) -> u64 {
            self.value64_
        }

        pub fn rmode(&self) -> RelocInfoMode {
            self.rmode_
        }

        pub fn size(type_: Type) -> usize {
            match type_ {
                Type::INTPTR => K_SYSTEM_POINTER_SIZE,
                Type::DOUBLE => K_DOUBLE_SIZE,
            }
        }

        const SHARING_PROHIBITED: i32 = -2;
        const SHARING_ALLOWED: i32 = -1;
    }

    impl Default for ConstantPoolEntry {
        fn default() -> Self {
            Self::new_default()
        }
    }

    #[derive(PartialEq, Eq)]
    pub enum Type {
        INTPTR,
        DOUBLE,
        NUMBER_OF_TYPES,
    }

    pub enum Access {
        REGULAR,
        OVERFLOWED,
    }

    #[cfg(target_arch = "powerpc64")]
    pub mod ppc64 {
        use super::*;

        pub struct ConstantPoolBuilder {
            info_: [PerTypeEntryInfo; 2], // Assuming NUMBER_OF_TYPES is 2
            emitted_label_: Label,
            ptr_reach_bits: i32,
            double_reach_bits: i32,
        }

        impl ConstantPoolBuilder {
            pub fn new(ptr_reach_bits: i32, double_reach_bits: i32) -> Self {
                ConstantPoolBuilder {
                    info_: [PerTypeEntryInfo::new(), PerTypeEntryInfo::new()],
                    emitted_label_: Label::new(),
                    ptr_reach_bits,
                    double_reach_bits,
                }
            }

            #[allow(dead_code)]
            pub fn add_entry(&mut self, position: i32, value: i64, sharing_ok: bool) -> Access {
                let entry = ConstantPoolEntry::new(position, value, sharing_ok, RelocInfoMode::NO_INFO);
                self.add_entry_internal(&entry, Type::INTPTR)
            }

            #[allow(dead_code)]
            pub fn add_entry_double(&mut self, position: i32, value: Double) -> Access {
                let entry = ConstantPoolEntry::new_double(position, value, RelocInfoMode::NO_INFO);
                self.add_entry_internal(&entry, Type::DOUBLE)
            }

            #[allow(dead_code)]
            pub fn add_entry_f64(&mut self, position: i32, value: f64) -> Access {
                self.add_entry_double(position, Double::new(value))
            }

            #[allow(dead_code)]
            pub fn next_access(&self, type_: Type) -> Access {
                if self.info_[type_ as usize].overflow() {
                    Access::OVERFLOWED
                } else {
                    Access::REGULAR
                }
            }

            #[allow(dead_code)]
            pub fn is_empty(&self) -> bool {
                self.info_[Type::INTPTR as usize].entries.is_empty() &&
                    self.info_[Type::INTPTR as usize].shared_entries.is_empty() &&
                    self.info_[Type::DOUBLE as usize].entries.is_empty() &&
                    self.info_[Type::DOUBLE as usize].shared_entries.is_empty()
            }

            #[allow(dead_code)]
            pub fn emit(&mut self, assm: &mut Assembler) -> i32 {
                if self.is_empty() {
                    return 0;
                }

                assm.bind(&mut self.emitted_label_);

                self.emit_shared_entries(assm, Type::INTPTR);
                self.emit_shared_entries(assm, Type::DOUBLE);

                self.emit_group(assm, Access::REGULAR, Type::INTPTR);
                self.emit_group(assm, Access::OVERFLOWED, Type::INTPTR);
                self.emit_group(assm, Access::REGULAR, Type::DOUBLE);
                self.emit_group(assm, Access::OVERFLOWED, Type::DOUBLE);

                self.emitted_label_.used = true;

                assm.pc_offset
            }

            #[allow(dead_code)]
            pub fn emitted_position(&mut self) -> &mut Label {
                &mut self.emitted_label_
            }

            fn add_entry_internal(&mut self, entry: &ConstantPoolEntry, type_: Type) -> Access {
                let info = &mut self.info_[type_ as usize];

                if entry.sharing_ok() {
                    info.shared_entries.push(*entry);
                } else {
                    info.entries.push(*entry);
                }

                let access = self.next_access(type_);

                if access == Access::REGULAR {
                    info.regular_count += 1;
                } else if info.overflow_start < 0 {
                    info.overflow_start = (info.entries.len() + info.shared_entries.len()) as i32 - 1;
                }

                access
            }

            fn emit_shared_entries(&mut self, assm: &mut Assembler, type_: Type) {
                let info = &mut self.info_[type_ as usize];
                for entry in &info.shared_entries {
                    assm.emit_int64(entry.value64() as i64); // Assuming emit_int64 handles both i64 and f64
                }
            }

            fn emit_group(&mut self, assm: &mut Assembler, access: Access, type_: Type) {
                let info = &mut self.info_[type_ as usize];
                let entries = if access == Access::REGULAR {
                    &info.entries[0..info.regular_count as usize]
                } else {
                    if info.overflow_start < 0 {
                        &info.entries[0..0]
                    } else {
                        &info.entries[info.overflow_start as usize..]
                    }
                };

                for entry in entries {
                    match type_ {
                        Type::INTPTR => {
                            assm.emit_int64(entry.value());
                        }
                        Type::DOUBLE => {
                            assm.emit_int64(entry.value64() as i64); // Assuming emit_int64 handles both i64 and f64
                        }
                        _ => panic!("Unexpected type"),
                    }
                }
            }
        }

        struct PerTypeEntryInfo {
            regular_count: i32,
            overflow_start: i32,
            regular_reach_bits: i32,
            entries: Vec<ConstantPoolEntry>,
            shared_entries: Vec<ConstantPoolEntry>,
        }

        impl PerTypeEntryInfo {
            fn new() -> Self {
                PerTypeEntryInfo {
                    regular_count: 0,
                    overflow_start: -1,
                    regular_reach_bits: 0,
                    entries: Vec::new(),
                    shared_entries: Vec::new(),
                }
            }

            fn overflow(&self) -> bool {
                self.overflow_start >= 0 &&
                    self.overflow_start < self.entries.len() as i32
            }
        }

        // Placeholder for Assembler
        pub struct Assembler {
            pc_offset: i32,
        }

        impl Assembler {
            pub fn new() -> Self {
                Assembler { pc_offset: 0 }
            }

            pub fn bind(&mut self, label: &mut Label) {
                label.used = true;
                self.pc_offset += 4; // Placeholder
            }

            pub fn emit_int64(&mut self, _value: i64) {
                self.pc_offset += 8; // Placeholder
            }
        }
    }

    #[cfg(any(target_arch = "arm64", target_arch = "riscv64", target_arch = "riscv32"))]
    pub mod other_arch {
        use std::cmp::Ordering;
        use std::collections::HashMap;

        use super::codegen::RelocInfoMode;
        use super::*;

        #[derive(Clone, Copy, Eq, PartialEq, Hash)]
        pub struct ConstantPoolKey {
            is_value32_: bool,
            value64_: u64,
            value32_: u32,
            rmode_: RelocInfoMode,
        }

        impl ConstantPoolKey {
            pub fn new_u64(value: u64, rmode: RelocInfoMode) -> Self {
                ConstantPoolKey {
                    is_value32_: false,
                    value64_: value,
                    value32_: 0,
                    rmode_: rmode,
                }
            }

            pub fn new_u32(value: u32, rmode: RelocInfoMode) -> Self {
                ConstantPoolKey {
                    is_value32_: true,
                    value64_: 0,
                    value32_: value,
                    rmode_: rmode,
                }
            }

            pub fn value64(&self) -> u64 {
                if self.is_value32_ {
                    panic!("Check failed: !is_value32_");
                }
                self.value64_
            }

            pub fn value32(&self) -> u32 {
                if !self.is_value32_ {
                    panic!("Check failed: is_value32_");
                }
                self.value32_
            }

            pub fn is_value32(&self) -> bool {
                self.is_value32_
            }

            pub fn rmode(&self) -> RelocInfoMode {
                self.rmode_
            }

            pub fn allows_deduplication(&self) -> bool {
                if self.rmode_ == RelocInfoMode::CONST_POOL ||
                    self.rmode_ == RelocInfoMode::VENEER_POOL ||
                    self.rmode_ == RelocInfoMode::DEOPT_SCRIPT_OFFSET ||
                    self.rmode_ == RelocInfoMode::DEOPT_INLINING_ID ||
                    self.rmode_ == RelocInfoMode::DEOPT_REASON ||
                    self.rmode_ == RelocInfoMode::DEOPT_ID ||
                    self.rmode_ == RelocInfoMode::DEOPT_NODE_ID {
                    return false;
                }

                let is_sharable_code_target =
                    self.rmode_ == RelocInfoMode::CODE_TARGET &&
                        (if self.is_value32() {
                            self.value32() != 0
                        } else {
                            self.value64() != 0
                        });
                let is_sharable_embedded_object = RelocInfoMode::is_embedded_object_mode(self.rmode_);
                RelocInfoMode::is_shareable_reloc_mode(self.rmode_) || is_sharable_code_target ||
                    is_sharable_embedded_object
            }
        }

        impl PartialOrd for ConstantPoolKey {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for ConstantPoolKey {
            fn cmp(&self, other: &Self) -> Ordering {
                if self.is_value32() < other.is_value32() {
                    return Ordering::Less;
                }
                if self.is_value32() > other.is_value32() {
                    return Ordering::Greater;
                }
                if self.rmode() < other.rmode() {
                    return Ordering::Less;
                }
                if self.rmode() > other.rmode() {
                    return Ordering::Greater;
                }
                if self.is_value32() {
                    return self.value32().cmp(&other.value32());
                }
                self.value64().cmp(&other.value64())
            }
        }

        pub struct ConstantPool {
            assm_: *mut Assembler, // raw pointer to Assembler
            first_use_32_: i32,
            first_use_64_: i32,
            entries_: HashMap<ConstantPoolKey, Vec<i32>>, // Map key to list of load pc offsets
            entry32_count_: usize,
            entry64_count_: usize,
            next_check_: i32,
            old_next_check_: i32,
            blocked_nesting_: i32,
        }

        impl ConstantPool {
            pub fn new(assm: *mut Assembler) -> Self {
                ConstantPool {
                    assm_: assm,
                    first_use_32_: -1,
                    first_use_64_: -1,
                    entries_: HashMap::new(),
                    entry32_count_: 0,
                    entry64_count_: 0,
                    next_check_: 0,
                    old_next_check_: 0,
                    blocked_nesting_: 0,
                }
            }

            pub fn record_entry_u32(&mut self, data: u32, rmode: RelocInfoMode) -> RelocInfoStatus {
                self.record_entry(data as u64, rmode, true)
            }

            pub fn record_entry_u64(&mut self, data: u64, rmode: RelocInfoMode) -> RelocInfoStatus {
                self.record_entry(data, rmode, false)
            }

            fn record_entry(&mut self, data: u64, rmode: RelocInfoMode, is_32bit: bool) -> RelocInfoStatus {
                let key = if is_32bit {
                    ConstantPoolKey::new_u32(data as u32, rmode)
                } else {
                    ConstantPoolKey::new_u64(data, rmode)
                };

                if self.first_use_32_ == -1 && is_32bit {
                    self.first_use_32_ = unsafe { (*self.assm_).pc_offset };
                }

                if self.first_use_64_ == -1 && !is_32bit {
                    self.first_use_64_ = unsafe { (*self.assm_).pc_offset };
                }

                let offset = unsafe { (*self.assm_).pc_offset };
                let reloc_info_status = self.record_key(key, offset);

                if is_32bit {
                    self.entry32_count_ += 1;
                } else {
                    self.entry64_count_ += 1;
                }

                reloc_info_status
            }

            pub fn entry32_count(&self) -> usize {
                self.entry32_count_
            }

            pub fn entry64_count(&self) -> usize {
                self.entry64_count_
            }

            pub fn is_empty(&self) -> bool {
                self.entries_.is_empty()
            }

            pub fn is_in_imm_range_if_emitted_at(&self, pc_offset: i32) -> bool {
                // Placeholder implementation
                true
            }

            pub fn compute_size(&self, require_jump: Jump, require_alignment: Alignment) -> i32 {
                let mut size = 0;

                if require_jump == Jump::kRequired {
                    size += self.prologue_size(require_jump); // Size of unconditional jump instruction.
                }

                size += self.entry32_count() as i32 * 4; // Size of each 32-bit entry.
                size += self.entry64_count() as i32 * 8; // Size of each 64-bit entry.

                if require_alignment == Alignment::kRequired {
                    size += self.prologue_size(require_jump);
                }

                size
            }

            pub fn emit_and_clear(&mut self, require: Jump) {
                unsafe {
                    let alignment = self.is_alignment_required_if_emitted_at(require, (*self.assm_).pc_offset);
                    self.emit_prologue(alignment);
                    self.emit_entries();
                    (*self.assm_).pc_offset += self.prologue_size(require);
                }
                self.clear();
            }

            pub fn should_emit_now(&self, require_jump: Jump, margin: usize) -> bool {
                let current_pc_offset = unsafe { (*self.assm_).pc_offset };
                if self.blocked_nesting_ > 0 {
                    return false;
                }
                if self.is_empty() {
                    return false;
                }

                let alignment = self.is_alignment_required_if_emitted_at(require_jump, current_pc_offset);
                let total_size = self.compute_size(require_jump, alignment);

                if require_jump == Jump::kRequired {
                    if current_pc_offset as usize + margin + total_size as usize > Self::kMaxDistToPool64 {
                        return true;
                    }
                } else {
                    if current_pc_offset as usize + margin + total_size as usize > Self::kOpportunityDistToPool64 {
                        return true;
                    }
                }

                self.next_check_ <= current_pc_offset
            }

            #[allow(dead_code)]
            pub fn check(&mut self, force_emission: Emission, require_jump: Jump, margin: usize) {
                if self.is_blocked() {
                    return;
                }

                let current_pc_offset = unsafe { (*self.assm_).pc_offset };
                let should_emit = match force_emission {
                    Emission::kForced => true,
                    Emission::kIfNeeded => self.should_emit_now(require_jump, margin),
                };

                if should_emit {
                    self.emit_and_clear(require_jump);
                } else {
                    self.maybe_set_next_check();
                }
            }

            #[allow(dead_code)]
            pub fn maybe_check(&mut self) {
                self.check(Emission::kIfNeeded, Jump::kOmitted, 0);
            }

            pub fn clear(&mut self) {
                self.entries_.clear();
                self.entry32_count_ = 0;
                self.entry64_count_ = 0;
                self.first_use_32_ = -1;
                self.first_use_64_ = -1;
            }

            pub fn is_blocked(&self) -> bool {
                self.blocked_nesting_ > 0
            }

            pub fn set_next_check_in(&mut self, instructions: usize) {
                self.next_check_ = unsafe { (*self.assm_).pc_offset } + (instructions as i32) * 4;
                self.old_next_check_ = self.next_check_;
            }

            fn start_block(&mut self) {
                self.blocked_nesting_ += 1;
            }

            fn end_block(&mut self) {
                self.blocked_nesting_ -= 1;
            }

            fn emit_entries(&mut self) {
                let mut sorted_entries: Vec<_> = self.entries_.iter().collect();
                sorted_entries.sort_by_key(|entry| entry.0); // Sort by ConstantPoolKey

                for (key, load_offsets) in sorted_entries {
                    self.emit(*key);
                    unsafe {
                        let last_offset = (*self.assm_).pc_offset;
                        for load_offset in load_offsets {
                            let instruction_ptr = load_offset as *mut Instruction;
                            self.set_load_offset_to_const_pool_entry(last_offset, instruction_ptr, key);
                        }
                    }
                }
            }

            fn emit_prologue(&mut self, require_alignment: Alignment) {
                unsafe {
                    // Placeholder implementation - emit alignment if required
                    if require_alignment == Alignment::kRequired {
                        (*self.assm_).pc_offset += 4; // Placeholder for alignment padding
                    }
                }
            }

            fn prologue_size(&self, require_jump: Jump) -> i32 {
                if require_jump == Jump::kRequired {
                    4 // Placeholder for jump instruction size
                } else {
                    0
                }
            }

            fn record_key(&mut self, key: ConstantPoolKey, offset: i32) -> RelocInfoStatus {
                if let Some(offsets) = self.entries_.get_mut(&key) {
                    offsets.push(offset);
                    RelocInfoStatus::kMustOmitForDuplicate
                } else {
                    self.entries_.insert(key, vec![offset]);
                    RelocInfoStatus::kMustRecord
                }
            }

            fn get_reloc_info_status_for(&self, key: &ConstantPoolKey) -> RelocInfoStatus {
                if self.entries_.contains_key(key) {
                    RelocInfoStatus::kMustOmitForDuplicate
                } else {
                    RelocInfoStatus::kMustRecord
                }
            }

            fn emit(&mut self, key: ConstantPoolKey) {
                unsafe {
                    if key.is_value32() {
                        (*self.assm_).pc_offset += 4; // Placeholder for emitting 32-bit value
                    } else {
                        (*self.assm_).pc_offset += 8; // Placeholder for emitting 64-bit value
                    }
                }
            }

            fn set_load_offset_to_const_pool_entry(&mut self, _load_offset: i32, _entry_offset: *mut Instruction, _key: &ConstantPoolKey) {
                // Placeholder implementation. Backpatching is usually platform-specific.
            }

            fn is_alignment_required_if_emitted_at(&self, require_jump: Jump, pc_offset: i32) -> Alignment {
                // Placeholder implementation
                Alignment::kOmitted
            }

            fn maybe_set_next_check(&mut self) {
                if self.next_check_ == self.old_next_check_ {
                    self.set_next_check_in(Self::kCheckInterval as usize);
                }
            }

            pub const kMaxDistToPool32: usize = 256 * 1024;
            pub const kMaxDistToPool64: usize = 256 * 1024;
            pub const kApproxDistToPool32: usize = 128 * 1024;
            pub const kOpportunityDistToPool32: usize = 64 * 1024;
            pub const kOpportunityDistToPool64: usize = 64 * 1024;
            pub const kCheckInterval: usize = 256;
            pub const kApproxMaxEntryCount: usize = 32;
            pub const kApproxDistToPool64: usize = 128 * 1024;
        }

        impl Drop for ConstantPool {
            fn drop(&mut self) {
                // No explicit memory management needed here in this simplified version
            }
        }

        #[derive(PartialEq)]
        pub enum Jump {
            kOmitted,
            kRequired,
        }

        pub enum Emission {
            kIfNeeded,
            kForced,
        }

        #[derive(PartialEq)]
        pub enum Alignment {
            kOmitted,
            kRequired,
        }

        pub enum RelocInfoStatus {
            kMustRecord,
            kMustOmitForDuplicate,
        }

        pub enum PoolEmissionCheck {
            kSkip,
        }

        pub struct BlockScope<'a> {
            pool_: &'a mut ConstantPool,
        }

        impl<'a> BlockScope<'a> {
            pub fn new(pool: &'a mut Assembler, margin: usize) -> Self {
                let mut scope_pool = unsafe { &mut *(&mut pool).constant_pool };
                if scope_pool.should_emit_now(Jump::kOmitted, margin) {
                    scope_pool.emit_and_clear(Jump::kOmitted);
                }
                scope_pool.start_block();
                BlockScope { pool_: scope_pool }
            }

            pub fn new_with_check(pool: &'a mut Assembler, _check: PoolEmissionCheck) -> Self {
                let mut scope_pool = unsafe { &mut *(&mut pool).constant_pool };
                scope_pool.start_block();
                BlockScope { pool_: scope_pool }
            }
        }

        impl<'a> Drop for BlockScope<'a> {
            fn drop(&mut self) {
                self.pool_.end_block();
            }
        }

        // Placeholder for Assembler
        pub struct Assembler {
            pc_offset: i32,
            constant_pool: *mut ConstantPool,
        }

        impl Assembler {
            pub fn new() -> Self {
                Assembler { pc_offset: 0, constant_pool: std::ptr::null_mut() }
            }

            pub fn set_constant_pool(&mut self, pool: *mut ConstantPool) {
                self.constant_pool = pool;
            }
        }

        impl fmt::Debug for ConstantPoolKey {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("ConstantPoolKey")
                    .field("is_value32_", &self.is_value32_)
                    .field("value64_", &self.value64_)
                    .field("value32_", &self.value32_)
                    .field("rmode_", &self.rmode_)
                    .finish()
            }
        }
    }
}