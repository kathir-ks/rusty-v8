// Converted from V8 C++ source files:
// Header: constant-pool.h
// Implementation: constant-pool.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod constant_pool {
    use std::collections::HashMap;
    use std::collections::BTreeMap;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    use crate::codegen::assembler::*;
    use crate::codegen::reloc_info::*;
    use crate::common::globals::*;

    pub struct ConstantPoolEntry {
        position_: i32,
        merged_index_: i32,
        value_: i64,
        value64_: u64,
        rmode_: RelocInfo::Mode,
    }

    impl ConstantPoolEntry {
        pub fn new(position: i32, value: i64, sharing_ok: bool, rmode: RelocInfo::Mode) -> Self {
            ConstantPoolEntry {
                position_: position,
                merged_index_: if sharing_ok {
                    ConstantPoolEntry::SHARING_ALLOWED
                } else {
                    ConstantPoolEntry::SHARING_PROHIBITED
                },
                value_: value,
                value64_: 0,
                rmode_: rmode,
            }
        }

        pub fn new_double(position: i32, value: f64, rmode: RelocInfo::Mode) -> Self {
            ConstantPoolEntry {
                position_: position,
                merged_index_: ConstantPoolEntry::SHARING_ALLOWED,
                value_: 0,
                value64_: value.to_bits(),
                rmode_: rmode,
            }
        }

        pub fn position(&self) -> i32 {
            self.position_
        }

        pub fn sharing_ok(&self) -> bool {
            self.merged_index_ != ConstantPoolEntry::SHARING_PROHIBITED
        }

        pub fn is_merged(&self) -> bool {
            self.merged_index_ >= 0
        }

        pub fn merged_index(&self) -> i32 {
            assert!(self.is_merged());
            self.merged_index_
        }

        pub fn set_merged_index(&mut self, index: i32) {
            assert!(self.sharing_ok());
            self.merged_index_ = index;
            assert!(self.is_merged());
        }

        pub fn offset(&self) -> i32 {
            assert!(self.merged_index_ >= 0);
            self.merged_index_
        }

        pub fn set_offset(&mut self, offset: i32) {
            assert!(offset >= 0);
            self.merged_index_ = offset;
        }

        pub fn value(&self) -> i64 {
            self.value_
        }

        pub fn value64(&self) -> u64 {
            self.value64_
        }

        pub fn rmode(&self) -> RelocInfo::Mode {
            self.rmode_
        }

        pub fn size(type_: Type) -> i32 {
            match type_ {
                Type::INTPTR => kSystemPointerSize as i32,
                Type::DOUBLE => kDoubleSize as i32,
            }
        }

        const SHARING_PROHIBITED: i32 = -2;
        const SHARING_ALLOWED: i32 = -1;
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

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ConstantPoolKey {
        is_value32_: bool,
        value64_: u64,
        value32_: u32,
        rmode_: RelocInfo::Mode,
    }

    impl ConstantPoolKey {
        pub fn new_u64(value: u64, rmode: RelocInfo::Mode) -> Self {
            ConstantPoolKey {
                is_value32_: false,
                value64_: value,
                value32_: 0,
                rmode_: rmode,
            }
        }

        pub fn new_u32(value: u32, rmode: RelocInfo::Mode) -> Self {
            ConstantPoolKey {
                is_value32_: true,
                value64_: 0,
                value32_: value,
                rmode_: rmode,
            }
        }

        pub fn value64(&self) -> u64 {
            assert!(!self.is_value32_);
            self.value64_
        }

        pub fn value32(&self) -> u32 {
            assert!(self.is_value32_);
            self.value32_
        }

        pub fn is_value32(&self) -> bool {
            self.is_value32_
        }

        pub fn rmode(&self) -> RelocInfo::Mode {
            self.rmode_
        }

        pub fn allows_deduplication(&self) -> bool {
            assert!(
                self.rmode_ != RelocInfo::Mode::CONST_POOL
                    && self.rmode_ != RelocInfo::Mode::VENEER_POOL
                    && self.rmode_ != RelocInfo::Mode::DEOPT_SCRIPT_OFFSET
                    && self.rmode_ != RelocInfo::Mode::DEOPT_INLINING_ID
                    && self.rmode_ != RelocInfo::Mode::DEOPT_REASON
                    && self.rmode_ != RelocInfo::Mode::DEOPT_ID
                    && self.rmode_ != RelocInfo::Mode::DEOPT_NODE_ID
            );

            let is_sharable_code_target =
                self.rmode_ == RelocInfo::Mode::CODE_TARGET
                    && (self.is_value32() && (self.value32() != 0)
                        || (!self.is_value32() && (self.value64() != 0)));
            let is_sharable_embedded_object = RelocInfo::is_embedded_object_mode(self.rmode_);
            RelocInfo::is_shareable_reloc_mode(self.rmode_) || is_sharable_code_target || is_sharable_embedded_object
        }
    }

    #[derive(PartialEq, Eq)]
    pub enum Jump {
        kOmitted,
        kRequired,
    }

    #[derive(PartialEq, Eq)]
    pub enum Emission {
        kIfNeeded,
        kForced,
    }

    #[derive(PartialEq, Eq)]
    pub enum Alignment {
        kOmitted,
        kRequired,
    }

    #[derive(PartialEq, Eq, Debug)]
    pub enum RelocInfoStatus {
        kMustRecord,
        kMustOmitForDuplicate,
    }

    #[derive(PartialEq, Eq)]
    pub enum PoolEmissionCheck {
        kSkip,
    }

    pub struct ConstantPool {
        assm_: *mut Assembler,
        entries_: BTreeMap<ConstantPoolKey, i32>,
        entry32_count_: usize,
        entry64_count_: usize,
        first_use_32_: i32,
        first_use_64_: i32,
        next_check_: i32,
        old_next_check_: i32,
        blocked_nesting_: i32,
    }

    impl ConstantPool {
        pub const kMaxDistToPool32: usize = 2048;
        pub const kMaxDistToPool64: usize = 4096;
        pub const kApproxDistToPool32: usize = 1024;
        pub const kApproxDistToPool64: usize = 2048;
        pub const kOpportunityDistToPool32: usize = 512;
        pub const kOpportunityDistToPool64: usize = 1024;
        pub const kCheckInterval: usize = 256;
        pub const kApproxMaxEntryCount: usize = 32;

        pub fn new(assm: *mut Assembler) -> Self {
            ConstantPool {
                assm_: assm,
                entries_: BTreeMap::new(),
                entry32_count_: 0,
                entry64_count_: 0,
                first_use_32_: -1,
                first_use_64_: -1,
                next_check_: 0,
                old_next_check_: 0,
                blocked_nesting_: 0,
            }
        }

        pub fn record_entry_u32(&mut self, data: u32, rmode: RelocInfo::Mode) -> RelocInfoStatus {
            let key = ConstantPoolKey::new_u32(data, rmode);
            self.record_key(key, unsafe { (*self.assm_).pc_offset() })
        }

        pub fn record_entry_u64(&mut self, data: u64, rmode: RelocInfo::Mode) -> RelocInfoStatus {
            let key = ConstantPoolKey::new_u64(data, rmode);
            self.record_key(key, unsafe { (*self.assm_).pc_offset() })
        }

        fn record_key(&mut self, key: ConstantPoolKey, offset: i32) -> RelocInfoStatus {
            let write_reloc_info = self.get_reloc_info_status_for(&key);

            if write_reloc_info == RelocInfoStatus::kMustRecord {
                if key.is_value32() {
                    if self.entry32_count_ == 0 {
                        self.first_use_32_ = offset;
                    }
                    self.entry32_count_ += 1;
                } else {
                    if self.entry64_count_ == 0 {
                        self.first_use_64_ = offset;
                    }
                    self.entry64_count_ += 1;
                }
            }
            self.entries_.insert(key, offset);

            if self.entry32_count_ + self.entry64_count_ > ConstantPool::kApproxMaxEntryCount {
                self.set_next_check_in(1);
            }

            write_reloc_info
        }

        fn get_reloc_info_status_for(&self, key: &ConstantPoolKey) -> RelocInfoStatus {
            if key.allows_deduplication() {
                if self.entries_.contains_key(key) {
                    return RelocInfoStatus::kMustOmitForDuplicate;
                }
            }
            RelocInfoStatus::kMustRecord
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
            let require_alignment = self.is_alignment_required_if_emitted_at(Jump::kRequired, pc_offset);
            let pool_end_32 = pc_offset as usize + self.compute_size(Jump::kRequired, require_alignment);
            let pool_end_64 = pool_end_32 - self.entry32_count_ * kInt32Size;
            let entries_in_range_32 = self.entry32_count_ == 0 || pool_end_32 < self.first_use_32_ as usize + ConstantPool::kMaxDistToPool32;
            let entries_in_range_64 = self.entry64_count_ == 0 || pool_end_64 < self.first_use_64_ as usize + ConstantPool::kMaxDistToPool64;
            entries_in_range_32 && entries_in_range_64
        }

        pub fn compute_size(&self, require_jump: Jump, require_alignment: Alignment) -> usize {
            let size_up_to_marker = self.prologue_size(require_jump);
            let alignment = if require_alignment == Alignment::kRequired { kInstrSize } else { 0 };
            let size_after_marker = self.entry32_count_ * kInt32Size + alignment + self.entry64_count_ * kInt64Size;
            size_up_to_marker + size_after_marker
        }

        pub fn emit_and_clear(&mut self, require: Jump) {
            assert!(!self.is_blocked());

            let block_pools = BlockPoolsScope::new(self, PoolEmissionCheck::kSkip);

            let require_alignment = self.is_alignment_required_if_emitted_at(require, unsafe { (*self.assm_).pc_offset() });
            let size = self.compute_size(require, require_alignment);

            let mut size_check = Label::new();
            unsafe { (*self.assm_).bind(&mut size_check) };
             unsafe { (*self.assm_).RecordConstPool(size) };

            let mut after_pool = Label::new();
            if require == Jump::kRequired {
                unsafe { (*self.assm_).b(&mut after_pool) };
            }

             unsafe { (*self.assm_).RecordComment("[ Constant Pool".to_string()) };
            self.emit_prologue(require_alignment);

            if require_alignment == Alignment::kRequired {
                unsafe { (*self.assm_).DataAlign(kInt64Size as i32) };
            }

            self.emit_entries();
             unsafe { (*self.assm_).RecordComment("]".to_string()) };

            if after_pool.is_linked() {
                unsafe { (*self.assm_).bind(&mut after_pool) };
            }
            assert_eq!(unsafe { (*self.assm_).SizeOfCodeGeneratedSince(&size_check) }, size);
            self.clear();
            drop(block_pools);
        }

        fn clear(&mut self) {
            self.entries_.clear();
            self.first_use_32_ = -1;
            self.first_use_64_ = -1;
            self.entry32_count_ = 0;
            self.entry64_count_ = 0;
            self.next_check_ = 0;
            self.old_next_check_ = 0;
        }

        fn start_block(&mut self) {
            if self.blocked_nesting_ == 0 {
                self.old_next_check_ = self.next_check_;
                self.next_check_ = i32::MAX;
            }
            self.blocked_nesting_ += 1;
        }

        fn end_block(&mut self) {
            self.blocked_nesting_ -= 1;
            if self.blocked_nesting_ == 0 {
                assert!(self.is_in_imm_range_if_emitted_at(unsafe { (*self.assm_).pc_offset() }));
                self.next_check_ = std::cmp::min(self.next_check_, self.old_next_check_);
            }
        }

        fn is_blocked(&self) -> bool {
            self.blocked_nesting_ > 0
        }

        fn set_next_check_in(&mut self, instructions: usize) {
            self.next_check_ = unsafe { (*self.assm_).pc_offset() } + (instructions * kInstrSize) as i32;
        }

        fn emit_entries(&mut self) {
            let mut iter = self.entries_.iter();
            while let Some((key, offset)) = iter.next() {
                assert!(key.is_value32() || unsafe { (*self.assm_).IsAligned( (*self.assm_).pc_offset() as usize, 8) });
                let range = self.entries_.range(key..);
                let shared = key.allows_deduplication();

                for (it_key, it_offset) in range {
                    self.set_load_offset_to_const_pool_entry(*it_offset, unsafe { (*self.assm_).pc() }, *it_key);
                    if !shared {
                        self.emit(*it_key);
                    }
                }

                if shared {
                    self.emit(*key);
                }
                self.entries_.remove(key);
                iter = self.entries_.iter();
            }
        }

        fn emit(&mut self, key: ConstantPoolKey) {
            if key.is_value32() {
                 unsafe { (*self.assm_).dd(key.value32()) };
            } else {
                 unsafe { (*self.assm_).dq(key.value64()) };
            }
        }

        pub fn should_emit_now(&self, require_jump: Jump, margin: usize) -> bool {
            if self.is_empty() {
                return false;
            }

            if self.entry32_count_ + self.entry64_count_ > ConstantPool::kApproxMaxEntryCount {
                return true;
            }
            let worst_case_size = self.compute_size(Jump::kRequired, Alignment::kRequired);
            let pool_end_32 = unsafe { (*self.assm_).pc_offset() } as usize + margin + worst_case_size;
            let pool_end_64 = pool_end_32 - self.entry32_count_ * kInt32Size;

            if self.entry64_count_ != 0 {
                let dist64 = pool_end_64 - self.first_use_64_ as usize;
                let next_check_too_late = dist64 + 2 * ConstantPool::kCheckInterval >= ConstantPool::kMaxDistToPool64;
                let opportune_emission_without_jump = require_jump == Jump::kOmitted && (dist64 >= ConstantPool::kOpportunityDistToPool64);
                let approximate_distance_exceeded = dist64 >= ConstantPool::kApproxDistToPool64;
                if next_check_too_late || opportune_emission_without_jump || approximate_distance_exceeded {
                    return true;
                }
            }

            if self.entry32_count_ != 0 {
                let dist32 = pool_end_32 - self.first_use_32_ as usize;
                let next_check_too_late = dist32 + 2 * ConstantPool::kCheckInterval >= ConstantPool::kMaxDistToPool32;
                let opportune_emission_without_jump = require_jump == Jump::kOmitted && (dist32 >= ConstantPool::kOpportunityDistToPool32);
                let approximate_distance_exceeded = dist32 >= ConstantPool::kApproxDistToPool32;
                if next_check_too_late || opportune_emission_without_jump || approximate_distance_exceeded {
                    return true;
                }
            }

            false
        }

        fn is_alignment_required_if_emitted_at(&self, require_jump: Jump, pc_offset: i32) -> Alignment {
            let size_up_to_marker = self.prologue_size(require_jump);
            if self.entry64_count_ != 0 && unsafe { !(*self.assm_).IsAligned(pc_offset as usize + size_up_to_marker, kInt64Size) } {
                return Alignment::kRequired;
            }
            Alignment::kOmitted
        }

        fn prologue_size(&self, require_jump: Jump) -> usize {
            let mut size = 0;
            if require_jump == Jump::kRequired {
                size += kInstrSize;
            }
            size
        }

        fn emit_prologue(&mut self, require_alignment: Alignment) {

        }

        fn set_load_offset_to_const_pool_entry(&mut self, load_offset: i32, entry_offset: *mut Instruction, key: ConstantPoolKey) {

        }

        pub fn maybe_check(&mut self) {
            if unsafe { (*self.assm_).pc_offset() } >= self.next_check_ {
                self.check(Emission::kIfNeeded, Jump::kRequired);
            }
        }

        pub fn check(&mut self, force_emission: Emission, require_jump: Jump, margin: usize){

        }
    }

    pub struct BlockPoolsScope<'a> {
        pool_: &'a mut ConstantPool,
    }

    impl<'a> BlockPoolsScope<'a> {
        pub fn new(pool: &'a mut ConstantPool, check: PoolEmissionCheck) -> Self {
            assert_eq!(check, PoolEmissionCheck::kSkip);
            pool.start_block();
            BlockPoolsScope { pool_: pool }
        }
    }

    impl<'a> Drop for BlockPoolsScope<'a> {
        fn drop(&mut self) {
            self.pool_.end_block();
        }
    }
}
