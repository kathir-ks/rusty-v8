// src/codegen/maglev-safepoint-table.rs

//use std::any::Any;
//use std::collections::HashMap;
//use std::marker::PhantomData;
//use std::ops::{Deref, DerefMut};
//use std::sync::{Arc, Mutex, MutexGuard};

/// Represents the Maglev safepoint table.
pub struct MaglevSafepointTable {
    instruction_start_: usize, // Address
    safepoint_table_address_: usize, // Address
    stack_slots_: usize, // Address - points to SafepointTableStackSlotsField_t
    length_: usize, // Address - points to int
    entry_configuration_: usize, // Address - points to uint32_t
    num_tagged_slots_: usize, // Address - points to uint32_t
}

const K_STACK_SLOTS_OFFSET: usize = 0;
const K_LENGTH_OFFSET: usize = 4;
const K_ENTRY_CONFIGURATION_OFFSET: usize = 8;
const K_NUM_TAGGED_SLOTS_OFFSET: usize = 12;
const K_HEADER_SIZE: usize = 16;
const K_INT_SIZE: usize = 4;

impl MaglevSafepointTable {
    /// Creates a new `MaglevSafepointTable` from a `Code` object.
    // pub fn new_from_code(isolate: &mut Isolate, pc: usize, code: &Code) -> Self {
    //     assert!(code.is_maglevved());
    //     Self::new(isolate, code.instruction_start(isolate, pc), code.safepoint_table_address())
    // }

    /// Creates a new `MaglevSafepointTable` from a `GcSafeCode` object.
    // pub fn new_from_gc_safe_code(isolate: &mut Isolate, pc: usize, code: &GcSafeCode) -> Self {
    //     assert!(code.is_maglevved());
    //     Self::new(isolate, code.instruction_start(isolate, pc), code.safepoint_table_address())
    // }

    /// Creates a new `MaglevSafepointTable`.
    pub fn new(instruction_start: usize, safepoint_table_address: usize) -> Self {
        let stack_slots_ = safepoint_table_address + K_STACK_SLOTS_OFFSET;
        let length_ = safepoint_table_address + K_LENGTH_OFFSET;
        let entry_configuration_ = safepoint_table_address + K_ENTRY_CONFIGURATION_OFFSET;
        let num_tagged_slots_ = safepoint_table_address + K_NUM_TAGGED_SLOTS_OFFSET;

        MaglevSafepointTable {
            instruction_start_: instruction_start,
            safepoint_table_address_: safepoint_table_address,
            stack_slots_: stack_slots_,
            length_: length_,
            entry_configuration_: entry_configuration_,
            num_tagged_slots_: num_tagged_slots_,
        }
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

    /// Finds the safepoint entry for a given program counter.
    pub fn find_entry(&self, pc: usize) -> MaglevSafepointEntry {
        let pc_offset = (pc as isize - self.instruction_start_ as isize) as i32;

        // Check if the PC is pointing at a trampoline.
        if self.has_deopt_data() {
            for i in 0..self.length_() {
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
        for i in 0..self.length_() {
            let entry = self.get_entry(i);
            if entry.pc() == pc_offset {
                return entry;
            }
        }

        // Return a default entry which has no deopt data and no pushed registers.
        // This allows us to elide emitting entries for trivial calls.
        let deopt_index = MaglevSafepointEntry::K_NO_DEOPT_INDEX;
        let trampoline_pc = MaglevSafepointEntry::K_NO_TRAMPOLINE_PC;
        let num_extra_spill_slots = 0;
        let tagged_register_indexes = 0;

        MaglevSafepointEntry::new(
            pc_offset,
            deopt_index,
            self.num_tagged_slots_(),
            num_extra_spill_slots,
            tagged_register_indexes,
            trampoline_pc,
        )
    }

    /// Finds the safepoint entry for a given program counter from GcSafeCode.
    // pub fn find_entry_from_gc_safe_code(isolate: &mut Isolate, code: &GcSafeCode, pc: usize) -> MaglevSafepointEntry {
    //     let table = MaglevSafepointTable::new_from_gc_safe_code(isolate, pc, code);
    //     table.find_entry(pc)
    // }

    /// Prints the safepoint table to the given output stream.
    pub fn print(&self, os: &mut dyn std::io::Write) -> std::io::Result<()> {
        writeln!(
            os,
            "Safepoints (stack slots = {}, entries = {}, byte size = {}, tagged slots = {})",
            self.stack_slots_(),
            self.length_(),
            self.byte_size(),
            self.num_tagged_slots_()
        )?;

        for index in 0..self.length_() {
            let entry = self.get_entry(index);
            writeln!(
                os,
                "{:p} {:6x}  num extra spill slots: {}",
                (self.instruction_start_ as usize + entry.pc() as usize) as *const u8,
                entry.pc(),
                entry.num_extra_spill_slots()
            )?;

            if entry.tagged_register_indexes() != 0 {
                write!(os, "  registers: ")?;
                let register_bits = entry.tagged_register_indexes();
                let bits = 32 - register_bits.leading_zeros();
                for j in (0..bits).rev() {
                    write!(os, "{}", (register_bits >> j) & 1)?;
                }
                writeln!(os)?;
            }

            if entry.has_deoptimization_index() {
                writeln!(
                    os,
                    "  deopt {:6} trampoline: {:6x}",
                    entry.deoptimization_index(),
                    entry.trampoline_pc()
                )?;
            }
        }

        Ok(())
    }

    /// Returns the number of stack slots.
    pub fn stack_slots_(&self) -> i32 {
        //unsafe { *(self.stack_slots_ as *const i32) }
        0 // Placeholder
    }

    /// Returns the length of the safepoint table.
    pub fn length_(&self) -> i32 {
        //unsafe { *(self.length_ as *const i32) }
        0 // Placeholder
    }

    /// Returns the entry configuration.
    pub fn entry_configuration_(&self) -> u32 {
        //unsafe { *(self.entry_configuration_ as *const u32) }
        0 // Placeholder
    }

    /// Returns the number of tagged slots.
    pub fn num_tagged_slots_(&self) -> u32 {
        //unsafe { *(self.num_tagged_slots_ as *const u32) }
        0 // Placeholder
    }

    /// Returns the size of the safepoint table in bytes.
    pub fn byte_size(&self) -> usize {
        K_HEADER_SIZE + self.length_() as usize * self.entry_size()
    }

    /// Returns the size of each entry in the safepoint table.
    fn entry_size(&self) -> usize {
        let mut size = 0;
        size += self.pc_size() as usize;
        if self.has_deopt_data() {
            size += self.deopt_index_size() as usize;
            size += self.pc_size() as usize;
        }
        size += 1; // num_extra_spill_slots
        size += self.register_indexes_size() as usize;
        size
    }

    /// Returns the safepoint entry at the given index.
    pub fn get_entry(&self, index: i32) -> MaglevSafepointEntry {
        let mut offset = K_HEADER_SIZE as i32;
        for i in 0..index {
            let entry_size = self.get_entry_size(i);
            offset += entry_size as i32;
        }
        let offset = offset as usize;

        let pc = self.read_bytes(offset, self.pc_size() as usize) as i32;

        let mut current_offset = offset + self.pc_size() as usize;
        let (deopt_index, trampoline_pc) = if self.has_deopt_data() {
            let deopt_index = self.read_bytes(current_offset, self.deopt_index_size() as usize) as i32 - 1;
            current_offset += self.deopt_index_size() as usize;
            let trampoline_pc = self.read_bytes(current_offset, self.pc_size() as usize) as i32 - 1;
            current_offset += self.pc_size() as usize;
            (deopt_index, trampoline_pc)
        } else {
            (MaglevSafepointEntry::K_NO_DEOPT_INDEX, MaglevSafepointEntry::K_NO_TRAMPOLINE_PC)
        };

        let num_extra_spill_slots = self.read_byte(current_offset);
        current_offset += 1;

        let tagged_register_indexes = self.read_bytes(current_offset, self.register_indexes_size() as usize);

        MaglevSafepointEntry::new(
            pc,
            deopt_index,
            self.num_tagged_slots_(),
            num_extra_spill_slots,
            tagged_register_indexes,
            trampoline_pc,
        )
    }

    fn get_entry_size(&self, index: i32) -> usize {
        let mut size = 0;
        size += self.pc_size() as usize;
        if self.has_deopt_data() {
            size += self.deopt_index_size() as usize;
            size += self.pc_size() as usize;
        }
        size += 1; // num_extra_spill_slots
        size += self.register_indexes_size() as usize;
        size
    }

    /// Returns whether the safepoint table has deoptimization data.
    pub fn has_deopt_data(&self) -> bool {
        (self.entry_configuration_() & HasDeoptDataField::MASK) != 0
    }

    /// Returns the size of the register indexes field.
    pub fn register_indexes_size(&self) -> u32 {
        (self.entry_configuration_() & RegisterIndexesSizeField::MASK) >> RegisterIndexesSizeField::SHIFT
    }

    /// Returns the size of the program counter field.
    pub fn pc_size(&self) -> u32 {
        (self.entry_configuration_() & PcSizeField::MASK) >> PcSizeField::SHIFT
    }

    /// Returns the size of the deoptimization index field.
    pub fn deopt_index_size(&self) -> u32 {
        (self.entry_configuration_() & DeoptIndexSizeField::MASK) >> DeoptIndexSizeField::SHIFT
    }

    fn read_byte(&self, offset: usize) -> u8 {
        //unsafe { *(self.safepoint_table_address_ as *const u8).add(offset) }
        0 // Placeholder
    }

    fn read_bytes(&self, offset: usize, bytes: usize) -> u32 {
        let mut value: u32 = 0;
        for i in 0..bytes {
            let byte = self.read_byte(offset + i) as u32;
            value |= byte << (i * 8);
        }
        value
    }
}

/// Represents a safepoint entry.
#[derive(Debug, Copy, Clone)]
pub struct MaglevSafepointEntry {
    pc_: i32,
    deoptimization_index_: i32,
    num_tagged_slots_: u32,
    num_extra_spill_slots_: u8,
    tagged_register_indexes_: u32,
    trampoline_pc_: i32,
}

impl MaglevSafepointEntry {
    /// Represents that no deoptimization index is present.
    pub const K_NO_DEOPT_INDEX: i32 = -1;
    /// Represents that no trampoline PC is present.
    pub const K_NO_TRAMPOLINE_PC: i32 = -1;

    /// Creates a new `MaglevSafepointEntry`.
    pub fn new(
        pc: i32,
        deoptimization_index: i32,
        num_tagged_slots: u32,
        num_extra_spill_slots: u8,
        tagged_register_indexes: u32,
        trampoline_pc: i32,
    ) -> Self {
        MaglevSafepointEntry {
            pc_: pc,
            deoptimization_index_: deoptimization_index,
            num_tagged_slots_: num_tagged_slots,
            num_extra_spill_slots_: num_extra_spill_slots,
            tagged_register_indexes_: tagged_register_indexes,
            trampoline_pc_: trampoline_pc,
        }
    }

    /// Returns the program counter.
    pub fn pc(&self) -> i32 {
        self.pc_
    }

    /// Returns the deoptimization index.
    pub fn deoptimization_index(&self) -> i32 {
        self.deoptimization_index_
    }

    /// Returns the number of tagged slots.
    pub fn num_tagged_slots(&self) -> u32 {
        self.num_tagged_slots_
    }

    /// Returns the number of extra spill slots.
    pub fn num_extra_spill_slots(&self) -> u8 {
        self.num_extra_spill_slots_
    }

    /// Returns the tagged register indexes.
    pub fn tagged_register_indexes(&self) -> u32 {
        self.tagged_register_indexes_
    }

    /// Returns the trampoline program counter.
    pub fn trampoline_pc(&self) -> i32 {
        self.trampoline_pc_
    }

    /// Returns whether the safepoint entry has a deoptimization index.
    pub fn has_deoptimization_index(&self) -> bool {
        self.deoptimization_index_ != Self::K_NO_DEOPT_INDEX
    }
}

macro_rules! define_field {
    ($name:ident, $offset:expr, $width:expr) => {
        pub struct $name {}
        impl $name {
            pub const MASK: u32 = ((1 << $width) - 1) << $offset;
            pub const SHIFT: u32 = $offset;

            pub fn encode(value: bool) -> u32 {
                (value as u32) << $offset
            }

            pub fn is_valid(value: i32) -> bool {
                value >= 0 && value < (1 << $width)
            }
        }
    };
}

define_field!(HasDeoptDataField, 0, 1);
define_field!(RegisterIndexesSizeField, 1, 2);
define_field!(PcSizeField, 3, 2);
define_field!(DeoptIndexSizeField, 5, 2);

pub struct MaglevSafepointTableBuilder {
    entries_: Vec<EntryBuilder>,
    safepoint_table_offset_: i32,
    num_tagged_slots_: i32,
}

impl MaglevSafepointTableBuilder {
    pub fn new() -> Self {
        MaglevSafepointTableBuilder {
            entries_: Vec::new(),
            safepoint_table_offset_: 0,
            num_tagged_slots_: 0,
        }
    }

    pub fn define_safepoint(&mut self, assembler: &mut Assembler) -> Safepoint {
        self.entries_.push(EntryBuilder::new(assembler.pc_offset_for_safepoint()));
        Safepoint {
            entry: self.entries_.last_mut().unwrap()
        }
    }

    pub fn update_deoptimization_info(&mut self, pc: i32, trampoline: i32, start: usize, deopt_index: i32) -> usize {
        assert_ne!(MaglevSafepointEntry::K_NO_TRAMPOLINE_PC, trampoline);
        assert_ne!(MaglevSafepointEntry::K_NO_DEOPT_INDEX, deopt_index);

        let mut index = start;
        while self.entries_[index].pc != pc {
            index += 1;
        }

        self.entries_[index].trampoline = trampoline;
        self.entries_[index].deopt_index = deopt_index;

        index
    }

    pub fn emit(&mut self, assembler: &mut Assembler, stack_slots: i32) {
        #[cfg(debug_assertions)]
        {
            let mut last_pc = -1;
            let mut last_trampoline = -1;
            for entry in &self.entries_ {
                // Entries are ordered by PC.
                assert!(last_pc < entry.pc);
                last_pc = entry.pc;

                // Trampoline PCs are increasing, and larger than regular PCs.
                if entry.trampoline != MaglevSafepointEntry::K_NO_TRAMPOLINE_PC {
                    assert!(last_trampoline < entry.trampoline);
                    assert!(self.entries_.last().unwrap().pc < entry.trampoline);
                    last_trampoline = entry.trampoline;
                }

                // An entry either has trampoline and deopt index, or none of the two.
                assert_eq!(entry.trampoline == MaglevSafepointEntry::K_NO_TRAMPOLINE_PC, entry.deopt_index == MaglevSafepointEntry::K_NO_DEOPT_INDEX);
            }
        }

        // We cannot emit a const pool within the safepoint table.
        // Assembler::BlockConstPoolScope block_const_pool(assembler);

        // Make sure the safepoint table is properly aligned. Pad with nops.
        //assembler.align(InstructionStream::kMetadataAlignment);
        assembler.record_comment(";;; Maglev safepoint table.");
        self.set_safepoint_table_offset(assembler.pc_offset());

        // Compute the required sizes of the fields.
        let mut used_register_indexes = 0;
        assert_eq!(MaglevSafepointEntry::K_NO_TRAMPOLINE_PC, -1);
        let mut max_pc = MaglevSafepointEntry::K_NO_TRAMPOLINE_PC;
        assert_eq!(MaglevSafepointEntry::K_NO_DEOPT_INDEX, -1);
        let mut max_deopt_index = MaglevSafepointEntry::K_NO_DEOPT_INDEX;

        for entry in &self.entries_ {
            used_register_indexes |= entry.tagged_register_indexes;
            max_pc = std::cmp::max(max_pc, std::cmp::max(entry.pc, entry.trampoline));
            max_deopt_index = std::cmp::max(max_deopt_index, entry.deopt_index);
        }

        // Derive the bytes and bools for the entry configuration from the values.
        let value_to_bytes = |value: i32| -> i32 {
            assert!(value >= 0);
            if value == 0 { return 0; }
            if value <= 0xff { return 1; }
            if value <= 0xffff { return 2; }
            if value <= 0xffffff { return 3; }
            return 4;
        };

        let has_deopt_data = max_deopt_index != -1;
        let register_indexes_size = value_to_bytes(used_register_indexes as i32);
        // Add 1 so all values (including kNoDeoptIndex and kNoTrampolinePC) are
        // non-negative.
        assert_eq!(MaglevSafepointEntry::K_NO_DEOPT_INDEX, -1);
        assert_eq!(MaglevSafepointEntry::K_NO_TRAMPOLINE_PC, -1);
        let pc_size = value_to_bytes(max_pc + 1);
        let deopt_index_size = value_to_bytes(max_deopt_index + 1);

        // Add a CHECK to ensure we never overflow the space in the bitfield, even for
        // huge functions which might not be covered by tests.
        assert!(RegisterIndexesSizeField::is_valid(register_indexes_size));
        assert!(PcSizeField::is_valid(pc_size));
        assert!(DeoptIndexSizeField::is_valid(deopt_index_size));

        let entry_configuration =
            HasDeoptDataField::encode(has_deopt_data) |
            RegisterIndexesSizeField::encode(register_indexes_size > 0) |
            PcSizeField::encode(pc_size > 0) |
            DeoptIndexSizeField::encode(deopt_index_size > 0);

        // Emit the table header.
        assert_eq!(K_STACK_SLOTS_OFFSET, 0 * K_INT_SIZE);
        assert_eq!(K_LENGTH_OFFSET, 1 * K_INT_SIZE);
        assert_eq!(K_ENTRY_CONFIGURATION_OFFSET, 2 * K_INT_SIZE);
        assert_eq!(K_NUM_TAGGED_SLOTS_OFFSET, 3 * K_INT_SIZE);
        assert_eq!(K_HEADER_SIZE, 4 * K_INT_SIZE);

        let length = self.entries_.len() as i32;

        assembler.dd(stack_slots);
        assembler.dd(length);
        assembler.dd(entry_configuration as i32);
        assembler.dd(self.num_tagged_slots_);

        let emit_bytes = |assembler: &mut Assembler, value: i32, bytes: i32| {
            assert!(value >= 0);
            let mut value = value as u32;
            for _ in 0..bytes {
                assembler.db((value & 0xff) as u8);
                value >>= 8;
            }
            assert_eq!(0, value);
        };

        // Emit entries, sorted by pc offsets.
        for entry in &self.entries_ {
            emit_bytes(assembler, entry.pc, pc_size);

            if has_deopt_data {
                // Add 1 so all values (including kNoDeoptIndex and kNoTrampolinePC) are
                // non-negative.
                assert_eq!(MaglevSafepointEntry::K_NO_DEOPT_INDEX, -1);
                assert_eq!(MaglevSafepointEntry::K_NO_TRAMPOLINE_PC, -1);

                emit_bytes(assembler, entry.deopt_index + 1, deopt_index_size);
                emit_bytes(assembler, entry.trampoline + 1, pc_size);
            }

            assembler.db(entry.num_extra_spill_slots);
            emit_bytes(assembler, entry.tagged_register_indexes as i32, register_indexes_size);
        }
    }

    pub fn set_safepoint_table_offset(&mut self, offset: i32) {
        self.safepoint_table_offset_ = offset;
    }

}

pub struct Safepoint<'a> {
    entry: &'a mut EntryBuilder,
}

impl<'a> Safepoint<'a> {
    pub fn tagged_registers(&mut self, mask: u32) {
        self.entry.tagged_register_indexes |= mask;
    }

    pub fn num_extra_spill_slots(&mut self, slots: u8) {
        self.entry.num_extra_spill_slots = slots;
    }
}

pub struct EntryBuilder {
    pc: i32,
    trampoline: i32,
    deopt_index: i32,
    num_extra_spill_slots: u8,
    tagged_register_indexes: u32,
}

impl EntryBuilder {
    pub fn new(pc: i32) -> Self {
        EntryBuilder {
            pc,
            trampoline: MaglevSafepointEntry::K_NO_TRAMPOLINE_PC,
            deopt_index: MaglevSafepointEntry::K_NO_DEOPT_INDEX,
            num_extra_spill_slots: 0,
            tagged_register_indexes: 0,
        }
    }
}

pub struct Assembler {
    pc_offset: i32,
    comments: Vec<String>,
    emitted_code: Vec<u8>
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            pc_offset: 0,
            comments: Vec::new(),
            emitted_code: Vec::new()
        }
    }

    pub fn pc_offset(&self) -> i32 {
        self.pc_offset
    }

    pub fn pc_offset_for_safepoint(&mut self) -> i32 {
        self.pc_offset
    }

    pub fn record_comment(&mut self, comment: &str) {
        self.comments.push(comment.to_string());
    }

    pub fn align(&mut self, alignment: usize) {
        while (self.emitted_code.len() % alignment) != 0 {
            self.db(0x90); // NOP
        }
    }

    pub fn dd(&mut self, value: i32) {
        self.db((value & 0xFF) as u8);
        self.db(((value >> 8) & 0xFF) as u8);
        self.db(((value >> 16) & 0xFF) as u8);
        self.db(((value >> 24) & 0xFF) as u8);
        self.pc_offset += 4;
    }

    pub fn db(&mut self, value: u8) {
        self.emitted_code.push(value);
        self.pc_offset += 1;
    }
}