// Converted from V8 C++ source files:
// Header: maglev-safepoint-table.h
// Implementation: maglev-safepoint-table.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod maglev_safepoint_table {
use std::fmt;
use std::io::Write;

use crate::base::bit_field::BitField;
use crate::codegen::safepoint_table_base::SafepointEntryBase;
use crate::common::assert_scope::AssertScope;
use crate::utils::allocation::AllocationType;
use crate::zone::zone_chunk_list::ZoneChunkList;
use crate::zone::zone::Zone;

// Placeholder for V8 types
pub struct Isolate {}
pub struct Code {}
pub struct GcSafeCode {}
pub struct Address(*const u8);
impl Address {
    pub fn advance(&mut self, offset: usize) {
        self.0 = unsafe { self.0.add(offset) };
    }
}

pub type Tagged<T> = *mut T;
pub type SafepointTableStackSlotsField_t = i32;
pub const kIntSize: usize = 4;
pub const kUInt32Size: usize = 4;
pub const kBitsPerByte: usize = 8;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MaglevSafepointEntry {
    pc: i32,
    deopt_index: i32,
    num_tagged_slots: u32,
    num_extra_spill_slots: u8,
    tagged_register_indexes: u32,
    trampoline_pc: i32,
}

impl MaglevSafepointEntry {
    pub const kNoDeoptIndex: i32 = -1;
    pub const kNoTrampolinePC: i32 = -1;

    pub fn new() -> Self {
        MaglevSafepointEntry {
            pc: 0,
            deopt_index: Self::kNoDeoptIndex,
            num_tagged_slots: 0,
            num_extra_spill_slots: 0,
            tagged_register_indexes: 0,
            trampoline_pc: Self::kNoTrampolinePC,
        }
    }

    pub fn with_values(
        pc: i32,
        deopt_index: i32,
        num_tagged_slots: u32,
        num_extra_spill_slots: u8,
        tagged_register_indexes: u32,
        trampoline_pc: i32,
    ) -> Self {
        MaglevSafepointEntry {
            pc,
            deopt_index,
            num_tagged_slots,
            num_extra_spill_slots,
            tagged_register_indexes,
            trampoline_pc,
        }
    }

    pub fn pc(&self) -> i32 {
        self.pc
    }

    pub fn deoptimization_index(&self) -> i32 {
        self.deopt_index
    }

    pub fn trampoline_pc(&self) -> i32 {
        self.trampoline_pc
    }

    pub fn num_tagged_slots(&self) -> u32 {
        self.num_tagged_slots
    }
    pub fn num_extra_spill_slots(&self) -> u8 {
        self.num_extra_spill_slots
    }
    pub fn tagged_register_indexes(&self) -> u32 {
        self.tagged_register_indexes
    }

    pub fn has_deoptimization_index(&self) -> bool {
        self.deopt_index != Self::kNoDeoptIndex
    }
}

// A wrapper class for accessing the safepoint table embedded into the
// InstructionStream object.
pub struct MaglevSafepointTable {
    instruction_start_: Address,
    safepoint_table_address_: Address,
    stack_slots_: SafepointTableStackSlotsField_t,
    length_: i32,
    entry_configuration_: u32,
    num_tagged_slots_: u32,
    no_gc_: AssertScope, // Placeholder: Assuming AssertScope can be default-constructed
}

impl MaglevSafepointTable {
    pub const kSafepointTableStackSlotsOffset: usize = 0;

    // The isolate and pc arguments are used for figuring out whether pc
    // belongs to the embedded or un-embedded code blob.
    pub fn new_from_code(isolate: *mut Isolate, pc: Address, code: Tagged<Code>) -> Self {
        // Dummy implementation
        MaglevSafepointTable::new(isolate, pc, code as *mut GcSafeCode)
    }

    pub fn new(isolate: *mut Isolate, pc: Address, code: Tagged<GcSafeCode>) -> Self {
        let instruction_start = unsafe {
            (code as *mut Code)
                .as_ref()
                .map(|c| Address(c as *const Code as *const u8))
                .unwrap_or(Address(std::ptr::null()))
        };
        let safepoint_table_address = unsafe {
            (code as *mut Code)
                .as_ref()
                .map(|c| Address(c as *const Code as *const u8))
                .unwrap_or(Address(std::ptr::null()))
        };
        MaglevSafepointTable::new_internal(instruction_start, safepoint_table_address)
    }

    fn new_internal(
        instruction_start: Address,
        safepoint_table_address: Address,
    ) -> Self {
        // Dummy values, replace with actual logic when available
        MaglevSafepointTable {
            instruction_start_: instruction_start,
            safepoint_table_address_: safepoint_table_address,
            stack_slots_: 0,
            length_: 0,
            entry_configuration_: 0,
            num_tagged_slots_: 0,
            no_gc_: AssertScope::new(), // Assuming AssertScope can be default-constructed
        }
    }

    pub fn length(&self) -> i32 {
        self.length_
    }

    pub fn byte_size(&self) -> i32 {
        Self::kHeaderSize as i32 + self.length_ * self.entry_size()
    }

    pub fn find_return_pc(&self, pc_offset: i32) -> i32 {
        for i in 0..self.length() {
            let entry = self.get_entry(i);
            if entry.trampoline_pc() == pc_offset || entry.pc() == pc_offset {
                return entry.pc();
            }
        }
        panic!("UNREACHABLE");
    }

    pub fn stack_slots(&self) -> SafepointTableStackSlotsField_t {
        self.stack_slots_
    }

    pub fn get_entry(&self, index: i32) -> MaglevSafepointEntry {
        assert!(self.length_ > index);
        let mut entry_ptr = self.safepoint_table_address_;
        unsafe { entry_ptr.advance(Self::kHeaderSize) };
        unsafe { entry_ptr.advance((index * self.entry_size()) as usize) };

        let pc = self.read_bytes(&mut entry_ptr, self.pc_size());
        let mut deopt_index = MaglevSafepointEntry::kNoDeoptIndex;
        let mut trampoline_pc = MaglevSafepointEntry::kNoTrampolinePC;

        if self.has_deopt_data() {
            assert_eq!(MaglevSafepointEntry::kNoDeoptIndex, -1);
            assert_eq!(MaglevSafepointEntry::kNoTrampolinePC, -1);

            deopt_index = self.read_bytes(&mut entry_ptr, self.deopt_index_size()) - 1;
            trampoline_pc = self.read_bytes(&mut entry_ptr, self.pc_size()) - 1;
            assert!(
                deopt_index >= 0 || deopt_index == MaglevSafepointEntry::kNoDeoptIndex
            );
            assert!(
                trampoline_pc >= 0
                    || trampoline_pc == MaglevSafepointEntry::kNoTrampolinePC
            );
        }

        let num_extra_spill_slots = self.read_byte(&mut entry_ptr);
        let tagged_register_indexes =
            self.read_bytes(&mut entry_ptr, self.register_indexes_size());

        MaglevSafepointEntry::with_values(
            pc,
            deopt_index,
            self.num_tagged_slots_,
            num_extra_spill_slots,
            tagged_register_indexes as u32,
            trampoline_pc,
        )
    }

    // Returns the entry for the given pc.
    pub fn find_entry(&self, pc: Address) -> MaglevSafepointEntry {
        let pc_offset = unsafe {
            pc.0.offset_from(self.instruction_start_.0) as i32
        };
        // Check if the PC is pointing at a trampoline.
        if self.has_deopt_data() {
            for i in 0..self.length_ {
                let entry = self.get_entry(i);
                let trampoline_pc = self.get_entry(i).trampoline_pc();
                if trampoline_pc != -1 && trampoline_pc == pc_offset {
                    return entry;
                }
                if trampoline_pc > pc_offset {
                    break;
                }
            }
        }

        // Try to find an exact pc match.
        for i in 0..self.length_ {
            let entry = self.get_entry(i);
            if entry.pc() == pc_offset {
                return entry;
            }
        }

        // Return a default entry which has no deopt data and no pushed registers.
        // This allows us to elide emitting entries for trivial calls.
        let deopt_index = MaglevSafepointEntry::kNoDeoptIndex;
        let trampoline_pc = MaglevSafepointEntry::kNoTrampolinePC;
        let num_extra_spill_slots = 0;
        let tagged_register_indexes = 0;

        MaglevSafepointEntry::with_values(
            pc_offset,
            deopt_index,
            self.num_tagged_slots_,
            num_extra_spill_slots,
            tagged_register_indexes,
            trampoline_pc,
        )
    }

    pub fn find_entry_static(
        isolate: *mut Isolate,
        code: Tagged<GcSafeCode>,
        pc: Address,
    ) -> MaglevSafepointEntry {
        let table = MaglevSafepointTable::new(isolate, pc, code);
        table.find_entry(pc)
    }

    const kStackSlotsOffset: usize = 0 * kIntSize;
    const kLengthOffset: usize = 1 * kIntSize;
    const kEntryConfigurationOffset: usize = 2 * kIntSize;
    const kNumTaggedSlotsOffset: usize = 3 * kIntSize;
    const kHeaderSize: usize = 4 * kIntSize;

    fn entry_size(&self) -> i32 {
        let deopt_data_size = if self.has_deopt_data() {
            self.pc_size() + self.deopt_index_size()
        } else {
            0
        };
        let num_pushed_registers_size = 1;
        self.pc_size() + deopt_data_size + num_pushed_registers_size + self.register_indexes_size()
    }

    fn has_deopt_data(&self) -> bool {
        HasDeoptDataField::decode(self.entry_configuration_)
    }
    fn pc_size(&self) -> i32 {
        PcSizeField::decode(self.entry_configuration_) as i32
    }
    fn deopt_index_size(&self) -> i32 {
        DeoptIndexSizeField::decode(self.entry_configuration_) as i32
    }
    fn register_indexes_size(&self) -> i32 {
        RegisterIndexesSizeField::decode(self.entry_configuration_) as i32
    }

    fn read_bytes(&self, ptr: &mut Address, bytes: i32) -> i32 {
        let mut result: u32 = 0;
        for b in 0..bytes {
            result |= (self.read_byte(ptr) as u32) << (8 * b);
        }
        result as i32
    }

    fn read_byte(&self, ptr: &mut Address) -> u8 {
        let result = unsafe { *(ptr.0 as *const u8) };
        unsafe { ptr.advance(1) };
        result
    }
}

impl fmt::Display for MaglevSafepointTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Safepoints (stack slots = {}, entries = {}, byte size = {}, tagged slots = {})\n",
            self.stack_slots_,
            self.length_,
            self.byte_size(),
            self.num_tagged_slots_
        )?;

        for index in 0..self.length_ {
            let entry = self.get_entry(index);
            let address = unsafe {
                self.instruction_start_.0.add(entry.pc() as usize) as *const std::ffi::c_void
            };
            write!(f, "{:p} {:6x}", address, entry.pc())?;

            write!(
                f,
                "  num extra spill slots: {}",
                entry.num_extra_spill_slots()
            )?;

            if entry.tagged_register_indexes() != 0 {
                write!(f, "  registers: ")?;
                let register_bits = entry.tagged_register_indexes();
                let bits = 32 - register_bits.leading_zeros();
                for j in (0..bits).rev() {
                    write!(f, "{}", (register_bits >> j) & 1)?;
                }
            }

            if entry.has_deoptimization_index() {
                write!(
                    f,
                    "  deopt {:6} trampoline: {:6x}",
                    entry.deoptimization_index(),
                    entry.trampoline_pc()
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

type HasDeoptDataField = BitField<bool, 0, 1>;
type RegisterIndexesSizeField = BitField<u32, { HasDeoptDataField::kNextOffset }, 3>;
type PcSizeField = BitField<u32, { RegisterIndexesSizeField::kNextOffset }, 3>;
type DeoptIndexSizeField = BitField<u32, { PcSizeField::kNextOffset }, 3>;

pub struct MaglevSafepointTableBuilder<'a> {
    num_tagged_slots_: u32,
    entries_: ZoneChunkList<'a, EntryBuilder>,
    safepoint_table_offset: i32,
}

impl<'a> MaglevSafepointTableBuilder<'a> {
    pub fn new(zone: &'a Zone, num_tagged_slots: u32) -> Self {
        MaglevSafepointTableBuilder {
            num_tagged_slots_: num_tagged_slots,
            entries_: ZoneChunkList::new(zone),
            safepoint_table_offset: 0,
        }
    }

    pub fn define_safepoint(&mut self, assembler: &mut Assembler) -> Safepoint<'a> {
        self.entries_
            .push_back(EntryBuilder::new(assembler.pc_offset_for_safepoint()));
        Safepoint::new(&mut self.entries_.back())
    }

    pub fn update_deoptimization_info(
        &mut self,
        pc: i32,
        trampoline: i32,
        start: usize,
        deopt_index: i32,
    ) -> usize {
        assert_ne!(MaglevSafepointEntry::kNoTrampolinePC, trampoline);
        assert_ne!(MaglevSafepointEntry::kNoDeoptIndex, deopt_index);

        let mut it = self.entries_.find(start);

        if it.is_none() {
            panic!("Entry not found!");
        }
        let mut it = it.unwrap();

        let mut index = start;
        while it.pc != pc {
            it = self.entries_.next(index).unwrap();
            index += 1;
        }

        it.trampoline = trampoline;
        it.deopt_index = deopt_index;
        index
    }

    pub fn emit(&mut self, assembler: &mut Assembler, stack_slots: i32) {
        #[cfg(debug_assertions)]
        {
            let mut last_pc = -1;
            let mut last_trampoline = -1;
            for entry in self.entries_.iter() {
                // Entries are ordered by PC.
                assert!(last_pc < entry.pc);
                last_pc = entry.pc;

                // Trampoline PCs are increasing, and larger than regular PCs.
                if entry.trampoline != MaglevSafepointEntry::kNoTrampolinePC {
                    assert!(last_trampoline < entry.trampoline);
                    assert!(self.entries_.back().pc < entry.trampoline);
                    last_trampoline = entry.trampoline;
                }

                // An entry either has trampoline and deopt index, or none of the two.
                assert_eq!(
                    (entry.trampoline == MaglevSafepointEntry::kNoTrampolinePC),
                    (entry.deopt_index == MaglevSafepointEntry::kNoDeoptIndex)
                );
            }
        }

        // We cannot emit a const pool within the safepoint table.
        // Assembler::BlockConstPoolScope block_const_pool(assembler);

        // Make sure the safepoint table is properly aligned. Pad with nops.
        assembler.align(InstructionStream::kMetadataAlignment as i32);
        assembler.record_comment(";;; Maglev safepoint table.");
        self.set_safepoint_table_offset(assembler.pc_offset());

        // Compute the required sizes of the fields.
        let mut used_register_indexes = 0;
        assert_eq!(MaglevSafepointEntry::kNoTrampolinePC, -1);
        let mut max_pc = MaglevSafepointEntry::kNoTrampolinePC;
        assert_eq!(MaglevSafepointEntry::kNoDeoptIndex, -1);
        let mut max_deopt_index = MaglevSafepointEntry::kNoDeoptIndex;
        for entry in self.entries_.iter() {
            used_register_indexes |= entry.tagged_register_indexes;
            max_pc = std::cmp::max(max_pc, std::cmp::max(entry.pc, entry.trampoline));
            max_deopt_index = std::cmp::max(max_deopt_index, entry.deopt_index);
        }

        // Derive the bytes and bools for the entry configuration from the values.
        let value_to_bytes = |value: i32| -> i32 {
            assert!(value >= 0);
            if value == 0 {
                return 0;
            }
            if value <= 0xff {
                return 1;
            }
            if value <= 0xffff {
                return 2;
            }
            if value <= 0xffffff {
                return 3;
            }
            4
        };

        let has_deopt_data = max_deopt_index != -1;
        let register_indexes_size = value_to_bytes(used_register_indexes as i32);

        // Add 1 so all values (including kNoDeoptIndex and kNoTrampolinePC) are
        // non-negative.
        assert_eq!(MaglevSafepointEntry::kNoDeoptIndex, -1);
        assert_eq!(MaglevSafepointEntry::kNoTrampolinePC, -1);
        let pc_size = value_to_bytes(max_pc + 1);
        let deopt_index_size = value_to_bytes(max_deopt_index + 1);

        // Add a CHECK to ensure we never overflow the space in the bitfield, even for
        // huge functions which might not be covered by tests.
        assert!(RegisterIndexesSizeField::is_valid(register_indexes_size as u32));
        assert!(PcSizeField::is_valid(pc_size as u32));
        assert!(DeoptIndexSizeField::is_valid(deopt_index_size as u32));

        let entry_configuration =
            HasDeoptDataField::encode(has_deopt_data)
            | RegisterIndexesSizeField::encode(register_indexes_size as u32)
            | PcSizeField::encode(pc_size as u32)
            | DeoptIndexSizeField::encode(deopt_index_size as u32);

        // Emit the table header.
        assert_eq!(MaglevSafepointTable::kStackSlotsOffset, 0 * kIntSize);
        assert_eq!(MaglevSafepointTable::kLengthOffset, 1 * kIntSize);
        assert_eq!(
            MaglevSafepointTable::kEntryConfigurationOffset,
            2 * kIntSize
        );
        assert_eq!(MaglevSafepointTable::kNumTaggedSlotsOffset, 3 * kIntSize);
        assert_eq!(MaglevSafepointTable::kHeaderSize, 4 * kIntSize);

        let length = self.entries_.size() as i32;
        assembler.dd(stack_slots);
        assembler.dd(length);
        assembler.dd(entry_configuration as i32);
        assembler.dd(self.num_tagged_slots_ as i32);

        let emit_bytes = |assembler: &mut Assembler, value: i32, bytes: i32| {
            assert!(value >= 0);
            let mut value = value as u32;
            for _ in 0..bytes {
                assembler.db((value & 0xff) as i32);
                value >>= 8;
            }
            assert_eq!(0, value);
        };

        // Emit entries, sorted by pc offsets.
        for entry in self.entries_.iter() {
            emit_bytes(assembler, entry.pc, pc_size);
            if has_deopt_data {
                // Add 1 so all values (including kNoDeoptIndex and kNoTrampolinePC) are
                // non-negative.
                assert_eq!(MaglevSafepointEntry::kNoDeoptIndex, -1);
                assert_eq!(MaglevSafepointEntry::kNoTrampolinePC, -1);
                emit_bytes(assembler, entry.deopt_index + 1, deopt_index_size);
                emit_bytes(assembler, entry.trampoline + 1, pc_size);
            }
            assembler.db(entry.num_extra_spill_slots as i32);
            emit_bytes(assembler, entry.tagged_register_indexes as i32, register_indexes_size);
        }
    }

    fn set_safepoint_table_offset(&mut self, offset: i32) {
        self.safepoint_table_offset = offset;
    }
}

pub struct Safepoint<'a> {
    entry_: &'a mut EntryBuilder,
}

impl<'a> Safepoint<'a> {
    fn new(entry: &'a mut EntryBuilder) -> Self {
        Safepoint { entry_: entry }
    }

    pub fn define_tagged_register(&mut self, reg_code: i32) {
        assert!(reg_code < (kBitsPerByte as i32 * std::mem::size_of::<u32>() as i32));
        self.entry_.tagged_register_indexes |= 1 << reg_code;
    }

    pub fn set_num_extra_spill_slots(&mut self, num_slots: u8) {
        self.entry_.num_extra_spill_slots = num_slots;
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct EntryBuilder {
    pc: i32,
    deopt_index: i32,
    trampoline: i32,
    num_extra_spill_slots: u8,
    tagged_register_indexes: u32,
}

impl EntryBuilder {
    fn new(pc: i32) -> Self {
        EntryBuilder {
            pc,
            deopt_index: MaglevSafepointEntry::kNoDeoptIndex,
            trampoline: MaglevSafepointEntry::kNoTrampolinePC,
            num_extra_spill_slots: 0,
            tagged_register_indexes: 0,
        }
    }
}

// Dummy Assembler
pub struct Assembler {
    pc_offset: i32,
    buffer: Vec<u8>,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            pc_offset: 0,
            buffer: Vec::new(),
        }
    }

    pub fn pc_offset(&self) -> i32 {
        self.pc_offset
    }

    pub fn pc_offset_for_safepoint(&self) -> i32 {
        self.pc_offset
    }

    pub fn align(&mut self, alignment: i32) {
        let alignment = alignment as usize;
        let remainder = self.buffer.len() % alignment;
        if remainder != 0 {
            let padding = alignment - remainder;
            self.buffer.extend(vec![0; padding]);
            self.pc_offset += padding as i32;
        }
    }

    pub fn record_comment(&mut self, _comment: &str) {}

    pub fn dd(&mut self, value: i32) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
        self.pc_offset += 4;
    }

    pub fn db(&mut self, value: i32) {
        self.buffer.push(value as u8);
        self.pc_offset += 1;
    }

    pub fn get_buffer(&self) -> &Vec<u8> {
        &self.buffer
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct InstructionStream {}

impl InstructionStream {
    pub const kMetadataAlignment: usize = 8;
}
}
