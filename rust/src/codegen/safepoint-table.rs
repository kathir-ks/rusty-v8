// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// #![allow(dead_code)] // Suppress dead code warnings during initial conversion

use std::{
    fmt,
    mem::size_of,
    ops::{Deref, DerefMut},
    vec::Vec,
};

//use bitflags::bitflags; // Consider using bitflags crate
use smallvec::SmallVec;

const K_BITS_PER_BYTE: usize = 8;
const K_INT_SIZE: usize = size_of::<i32>(); // Assuming int is i32
// Placeholder for InstructionStream
#[derive(Debug)]
struct InstructionStream;

impl InstructionStream {
    const K_METADATA_ALIGNMENT: usize = 8; // Example alignment value
}

macro_rules! static_assert {
    ($cond:expr) => {
        #[cfg(debug_assertions)]
        const _: () = assert!($cond);
    };
}

#[allow(non_camel_case_types)]
type SafepointTableStackSlotsField_t = i32;

// Placeholder for Assembler (replace with your actual assembler abstraction)
struct Assembler {
    buffer: Vec<u8>,
    safepoint_table_offset: usize,
}

impl Assembler {
    fn new() -> Self {
        Assembler {
            buffer: Vec::new(),
            safepoint_table_offset: 0,
        }
    }

    fn pc_offset(&self) -> usize {
        self.buffer.len()
    }

    fn pc_offset_for_safepoint(&self) -> usize {
        self.pc_offset()
    }

    fn align(&mut self, alignment: usize) {
        while self.buffer.len() % alignment != 0 {
            self.nop(); // Assuming nop is the padding instruction
        }
    }

    fn nop(&mut self) {
        self.db(0x90); // Example nop instruction
    }

    fn record_comment(&mut self, comment: &str) {
        // Placeholder for comment recording
        println!("Comment: {}", comment);
    }

    fn dd(&mut self, value: i32) {
        self.buffer.extend_from_slice(&value.to_le_bytes());
    }

    fn db(&mut self, value: u8) {
        self.buffer.push(value);
    }

    fn safepoint_table_offset(&self) -> usize {
        self.safepoint_table_offset
    }

    fn block_const_pool_scope(&self) -> BlockConstPoolScope {
        BlockConstPoolScope {}
    }

    fn set_safepoint_table_offset(&mut self, offset: usize) {
        self.safepoint_table_offset = offset;
    }
}
struct BlockConstPoolScope;
impl BlockConstPoolScope {
    fn new(_assembler: &Assembler) -> Self {
        BlockConstPoolScope {}
    }
}

// Example implementation (replace with actual functionality)
#[allow(dead_code)]
struct Isolate {}
impl Isolate {
    fn instruction_start(&self, pc: Address) -> Address {
        pc // Placeholder
    }
}

// Placeholder for Tagged<Code> and Tagged<GcSafeCode>
#[derive(Clone, Copy, Debug)]
struct Address(usize);

// Placeholder for Tagged<Code> and Tagged<GcSafeCode>
#[derive(Clone, Copy, Debug)]
struct Tagged<T> {
    ptr: *mut T,
}

impl<T> Tagged<T> {
    fn instruction_start(&self, _isolate: &Isolate, pc: Address) -> Address {
        pc
    }

    fn safepoint_table_address(&self) -> Address {
        Address(0) // Placeholder
    }

    fn is_turbofanned(&self) -> bool {
        true // Placeholder
    }
}

// Placeholder for WasmCode
#[cfg(feature = "v8_enable_webassembly")]
mod wasm {
    #[derive(Debug)]
    pub struct WasmCode {
        instruction_start: super::Address,
        safepoint_table_offset: usize,
    }

    impl WasmCode {
        pub fn instruction_start(&self) -> super::Address {
            self.instruction_start
        }
        pub fn safepoint_table_offset(&self) -> usize {
            self.safepoint_table_offset
        }
    }
}

// -----------------------------------------------------------------------------
// SafepointTable
// -----------------------------------------------------------------------------

struct MemoryAccessor<T> {
    address: Address,
    phantom: std::marker::PhantomData<T>,
}

impl<T> MemoryAccessor<T> {
    fn new(address: Address) -> Self {
        MemoryAccessor {
            address,
            phantom: std::marker::PhantomData,
        }
    }

    fn read(&self) -> T
    where
        T: Copy,
    {
        unsafe { (self.address.0 as *const T).read_unaligned() }
    }

    fn write(&self, value: T) {
        unsafe {
            (self.address.0 as *mut T).write_unaligned(value);
        }
    }
}

struct SafepointTable {
    instruction_start_: Address,
    safepoint_table_address_: Address,
    stack_slots_: MemoryAccessor<SafepointTableStackSlotsField_t>,
    length_: MemoryAccessor<i32>,
    entry_configuration_: MemoryAccessor<u32>,
}

impl SafepointTable {
    fn new_from_code(isolate: &Isolate, pc: Address, code: Tagged<Code>) -> Self {
        SafepointTable::new(
            code.instruction_start(isolate, pc),
            code.safepoint_table_address(),
        )
    }

    fn new_from_gc_safe_code(isolate: &Isolate, pc: Address, code: Tagged<GcSafeCode>) -> Self {
        SafepointTable::new(
            code.instruction_start(isolate, pc),
            code.safepoint_table_address(),
        )
    }

    #[cfg(feature = "v8_enable_webassembly")]
    fn new_from_wasm_code(code: &wasm::WasmCode) -> Self {
        SafepointTable::new(
            code.instruction_start(),
            Address(
                code.instruction_start().0 + code.safepoint_table_offset(), /*instruction_start() + code.safepoint_table_offset() as usize*/
            ),
        )
    }

    fn new(instruction_start: Address, safepoint_table_address: Address) -> Self {
        SafepointTable {
            instruction_start_: instruction_start,
            safepoint_table_address_: safepoint_table_address,
            stack_slots_: MemoryAccessor::new(Address(
                safepoint_table_address.0 + SafepointTable::k_stack_slots_offset(),
            )),
            length_: MemoryAccessor::new(Address(
                safepoint_table_address.0 + SafepointTable::k_length_offset(),
            )),
            entry_configuration_: MemoryAccessor::new(Address(
                safepoint_table_address.0 + SafepointTable::k_entry_configuration_offset(),
            )),
        }
    }

    const fn k_stack_slots_offset() -> usize {
        0 * K_INT_SIZE
    }
    const fn k_length_offset() -> usize {
        1 * K_INT_SIZE
    }
    const fn k_entry_configuration_offset() -> usize {
        2 * K_INT_SIZE
    }
    const fn k_header_size() -> usize {
        3 * K_INT_SIZE
    }

    fn find_return_pc(&self, pc_offset: i32) -> i32 {
        for i in 0..self.length() {
            let entry = self.get_entry(i as usize);
            if entry.trampoline_pc() == pc_offset || entry.pc() == pc_offset {
                return entry.pc();
            }
        }
        panic!("UNREACHABLE");
    }

    fn try_find_entry(&self, pc: Address) -> Option<SafepointEntry> {
        let pc_offset = (pc.0 as isize - self.instruction_start_.0 as isize) as i32;

        // Check if the PC is pointing at a trampoline.
        if self.has_deopt_data() {
            let mut candidate: Option<usize> = None;
            for i in 0..self.length_() as usize {
                let trampoline_pc = self.get_entry(i).trampoline_pc();
                if trampoline_pc != -1 && trampoline_pc <= pc_offset {
                    candidate = Some(i);
                }
                if trampoline_pc > pc_offset {
                    break;
                }
            }
            if let Some(candidate_index) = candidate {
                return Some(self.get_entry(candidate_index));
            }
        }

        for i in 0..self.length_() as usize {
            let entry = self.get_entry(i);
            if i == (self.length_() - 1) as usize || self.get_entry(i + 1).pc() > pc_offset {
                if entry.pc() > pc_offset {
                    return None;
                }
                return Some(entry);
            }
        }
        None
    }

    fn find_entry(&self, pc: Address) -> SafepointEntry {
        self.try_find_entry(pc).expect("CHECK failed")
    }

    fn find_entry_static(isolate: &Isolate, code: Tagged<GcSafeCode>, pc: Address) -> SafepointEntry {
        let table = SafepointTable::new_from_gc_safe_code(isolate, pc, code);
        table.find_entry(pc)
    }

    fn print(&self, os: &mut dyn std::fmt::Write) -> fmt::Result {
        writeln!(
            os,
            "Safepoints (stack slots = {}, entries = {}, byte size = {})",
            self.stack_slots(),
            self.length(),
            self.byte_size()
        )?;

        for index in 0..self.length() {
            let entry = self.get_entry(index as usize);
            write!(os, "{:p} ", (self.instruction_start_.0 + entry.pc() as usize) as *const ())?;
            write!(os, "{:6x} ", entry.pc())?;

            if !entry.tagged_slots().is_empty() {
                write!(os, "  slots (sp->fp): ")?;
                let mut i = 0;
                for &bits in entry.tagged_slots() {
                    for bit in 0..K_BITS_PER_BYTE {
                        if i < self.stack_slots() as u32 {
                            write!(os, "{}", (bits >> bit) & 1)?;
                            i += 1;
                        } else {
                            break;
                        }
                    }
                }
                for _ in i..self.stack_slots() as u32 {
                    write!(os, "0")?;
                }
            }

            if entry.tagged_register_indexes() != 0 {
                write!(os, "  registers: ")?;
                let register_bits = entry.tagged_register_indexes();
                let bits = 32 - register_bits.leading_zeros();
                for j in (0..bits).rev() {
                    write!(os, "{}", (register_bits >> j) & 1)?;
                }
            }

            if entry.has_deoptimization_index() {
                write!(os, "  deopt {:6} trampoline: {:6x}", entry.deoptimization_index(), entry.trampoline_pc())?;
            }
            writeln!(os)?;
        }
        Ok(())
    }

    fn stack_slots(&self) -> SafepointTableStackSlotsField_t {
        self.stack_slots_.read()
    }

    fn length(&self) -> i32 {
        self.length_.read()
    }

    fn entry_configuration(&self) -> u32 {
        self.entry_configuration_.read()
    }

    fn byte_size(&self) -> usize {
        let length = self.length();
        let entry_config = self.entry_configuration();
        let has_deopt_data = SafepointTable::HasDeoptDataField::decode(entry_config);
        let register_indexes_size = SafepointTable::RegisterIndexesSizeField::decode(entry_config) as usize;
        let pc_size = SafepointTable::PcSizeField::decode(entry_config) as usize;
        let deopt_index_size = SafepointTable::DeoptIndexSizeField::decode(entry_config) as usize;
        let tagged_slots_bytes = SafepointTable::TaggedSlotsBytesField::decode(entry_config) as usize;

        let mut entry_size = pc_size;
        if has_deopt_data {
            entry_size += deopt_index_size + pc_size;
        }
        entry_size += register_indexes_size;
        let entries_size = length as usize * entry_size;
        let bitmap_size = length as usize * tagged_slots_bytes;
        SafepointTable::k_header_size() + entries_size + bitmap_size
    }

    fn get_entry(&self, index: usize) -> SafepointEntry {
        let entry_config = self.entry_configuration();
        let has_deopt_data = SafepointTable::HasDeoptDataField::decode(entry_config);
        let register_indexes_size = SafepointTable::RegisterIndexesSizeField::decode(entry_config) as usize;
        let pc_size = SafepointTable::PcSizeField::decode(entry_config) as usize;
        let deopt_index_size = SafepointTable::DeoptIndexSizeField::decode(entry_config) as usize;
        let tagged_slots_bytes = SafepointTable::TaggedSlotsBytesField::decode(entry_config) as usize;

        let mut offset = SafepointTable::k_header_size();
        offset += index * (pc_size + if has_deopt_data { deopt_index_size + pc_size } else { 0 } + register_indexes_size);
        let pc_offset = self.read_bytes(self.safepoint_table_address_.0 + offset, pc_size) as i32;
        offset += pc_size;

        let (deopt_index, trampoline) = if has_deopt_data {
            let deopt_index_value = self.read_bytes(self.safepoint_table_address_.0 + offset, deopt_index_size) as i32 - 1;
            offset += deopt_index_size;
            let trampoline_value = self.read_bytes(self.safepoint_table_address_.0 + offset, pc_size) as i32 - 1;
            offset += pc_size;
            (deopt_index_value, trampoline_value)
        } else {
            (SafepointEntry::K_NO_DEOPT_INDEX, SafepointEntry::K_NO_TRAMPOLINE_PC)
        };
        let register_indexes = self.read_bytes(self.safepoint_table_address_.0 + offset, register_indexes_size);

        let bitmap_offset = SafepointTable::k_header_size();
        let entry_size = pc_size + if has_deopt_data { deopt_index_size + pc_size } else { 0 } + register_indexes_size;
        let bitmap_offset = bitmap_offset + self.length() as usize * entry_size + index * tagged_slots_bytes;
        let tagged_slots = (0..tagged_slots_bytes)
            .map(|i| unsafe {
                *((self.safepoint_table_address_.0 + bitmap_offset + i) as *const u8)
            })
            .collect();

        SafepointEntry::new(pc_offset, trampoline, deopt_index, register_indexes, tagged_slots)
    }

    fn has_deopt_data(&self) -> bool {
        SafepointTable::HasDeoptDataField::decode(self.entry_configuration())
    }

    fn read_bytes(&self, address: usize, bytes: usize) -> u32 {
        let mut value: u32 = 0;
        for i in 0..bytes {
            let byte = unsafe { *((address + i) as *const u8) };
            value |= (byte as u32) << (i * 8);
        }
        value
    }

    // Define nested field accessors with bit manipulation for entry_configuration
    mod HasDeoptDataField {
        pub fn decode(entry_configuration: u32) -> bool {
            (entry_configuration & 0x1) != 0
        }
        pub fn encode(has_deopt_data: bool) -> u32 {
            if has_deopt_data {
                1 << 0
            } else {
                0
            }
        }
    }

    mod RegisterIndexesSizeField {
        pub fn decode(entry_configuration: u32) -> u32 {
            (entry_configuration >> 1) & 0x7
        }
        pub fn encode(register_indexes_size: usize) -> u32 {
            (register_indexes_size as u32) << 1
        }

        pub fn is_valid(_value: usize) -> bool {
            true
        }
    }

    mod PcSizeField {
        pub fn decode(entry_configuration: u32) -> u32 {
            (entry_configuration >> 4) & 0x7
        }

        pub fn encode(pc_size: usize) -> u32 {
            (pc_size as u32) << 4
        }

        pub fn is_valid(_value: usize) -> bool {
            true
        }
    }

    mod DeoptIndexSizeField {
        pub fn decode(entry_configuration: u32) -> u32 {
            (entry_configuration >> 7) & 0x7
        }

        pub fn encode(deopt_index_size: usize) -> u32 {
            (deopt_index_size as u32) << 7
        }

        pub fn is_valid(_value: usize) -> bool {
            true
        }
    }

    mod TaggedSlotsBytesField {
        pub fn decode(entry_configuration: u32) -> u32 {
            (entry_configuration >> 10) & 0x7
        }

        pub fn encode(tagged_slots_bytes: usize) -> u32 {
            (tagged_slots_bytes as u32) << 10
        }

        pub fn is_valid(_value: usize) -> bool {
            true
        }
    }
}

// -----------------------------------------------------------------------------
// SafepointEntry
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
struct SafepointEntry {
    pc: i32,
    trampoline: i32,
    deopt_index: i32,
    register_indexes: u32,
    tagged_slots: SmallVec<[u8; 4]>,
}

impl SafepointEntry {
    const K_NO_TRAMPOLINE_PC: i32 = -1;
    const K_NO_DEOPT_INDEX: i32 = -1;

    fn new(pc: i32, trampoline: i32, deopt_index: i32, register_indexes: u32, tagged_slots: SmallVec<[u8; 4]>) -> Self {
        SafepointEntry {
            pc,
            trampoline,
            deopt_index,
            register_indexes,
            tagged_slots,
        }
    }

    fn pc(&self) -> i32 {
        self.pc
    }

    fn trampoline_pc(&self) -> i32 {
        self.trampoline
    }

    fn deoptimization_index(&self) -> i32 {
        self.deopt_index
    }

    fn tagged_register_indexes(&self) -> u32 {
        self.register_indexes
    }

    fn tagged_slots(&self) -> &SmallVec<[u8; 4]> {
        &self.tagged_slots
    }

    fn has_deoptimization_index(&self) -> bool {
        self.deopt_index != SafepointEntry::K_NO_DEOPT_INDEX
    }

    fn is_initialized(&self) -> bool {
        true
    }
}

impl Default for SafepointEntry {
    fn default() -> Self {
        SafepointEntry {
            pc: 0,
            trampoline: SafepointEntry::K_NO_TRAMPOLINE_PC,
            deopt_index: SafepointEntry::K_NO_DEOPT_INDEX,
            register_indexes: 0,
            tagged_slots: SmallVec::new(),
        }
    }
}

// -----------------------------------------------------------------------------
// SafepointTableBuilder
// -----------------------------------------------------------------------------

struct SafepointTableBuilder<'a> {
    entries_: Vec<EntryBuilder<'a>>,
    zone_: &'a Zone,
    max_stack_index_: i32,
}

impl<'a> SafepointTableBuilder<'a> {
    fn new(zone: &'a Zone) -> Self {
        SafepointTableBuilder {
            entries_: Vec::new(),
            zone_: zone,
            max_stack_index_: 0,
        }
    }

    fn define_safepoint(&mut self, assembler: &Assembler, pc_offset: Option<usize>) -> Safepoint<'a> {
        let pc_offset_val = pc_offset.unwrap_or_else(|| assembler.pc_offset_for_safepoint());
        self.entries_.push(EntryBuilder::new(self.zone_, pc_offset_val as i32));
        Safepoint {
            entry: self.entries_.last_mut().unwrap(),
            builder: self,
        }
    }

    fn update_deoptimization_info(&mut self, pc: i32, trampoline: i32, start: usize, deopt_index: i32) -> usize {
        assert_ne!(SafepointEntry::K_NO_TRAMPOLINE_PC, trampoline);
        assert_ne!(SafepointEntry::K_NO_DEOPT_INDEX, deopt_index);

        let mut index = start;
        let mut it = self.entries_.iter_mut().skip(start);

        let found = it.find(|entry| entry.pc == pc).is_some();
        assert!(found);

        let mut it = self.entries_.iter_mut().skip(start);
        let entry = it.find(|entry| entry.pc == pc).unwrap();

        entry.trampoline = trampoline;
        entry.deopt_index = deopt_index;
        index
    }

    fn emit(&mut self, assembler: &mut Assembler, stack_slot_count: i32) {
        assert!(self.max_stack_index_ < stack_slot_count);

        #[cfg(debug_assertions)]
        {
            let mut last_pc = -1;
            let mut last_trampoline = -1;
            for entry in &self.entries_ {
                // Entries are ordered by PC.
                assert!(last_pc < entry.pc);
                last_pc = entry.pc;
                // Trampoline PCs are increasing, and larger than regular PCs.
                if entry.trampoline != SafepointEntry::K_NO_TRAMPOLINE_PC {
                    assert!(last_trampoline < entry.trampoline);
                    assert!(self.entries_.last().unwrap().pc < entry.trampoline);
                    last_trampoline = entry.trampoline;
                }
                // An entry either has trampoline and deopt index, or none of the two.
                assert_eq!(
                    entry.trampoline == SafepointEntry::K_NO_TRAMPOLINE_PC,
                    entry.deopt_index == SafepointEntry::K_NO_DEOPT_INDEX
                );
            }
        }

        self.remove_duplicates();

        // The encoding is compacted by translating stack slot indices s.t. they
        // start at 0. See also below.
        let tagged_slots_size = stack_slot_count - self.min_stack_index();

        // We cannot emit a const pool within the safepoint table.
        let _block_const_pool = assembler.block_const_pool_scope();

        // Make sure the safepoint table is properly aligned. Pad with nops.
        assembler.align(InstructionStream::K_METADATA_ALIGNMENT);
        assembler.record_comment(";;; Safepoint table.");
        assembler.set_safepoint_table_offset(assembler.pc_offset());

        // Compute the required sizes of the fields.
        let mut used_register_indexes = 0;
        static_assert!(SafepointEntry::K_NO_TRAMPOLINE_PC == -1);
        let mut max_pc = SafepointEntry::K_NO_TRAMPOLINE_PC;
        static_assert!(SafepointEntry::K_NO_DEOPT_INDEX == -1);
        let mut max_deopt_index = SafepointEntry::K_NO_DEOPT_INDEX;

        for entry in &self.entries_ {
            used_register_indexes |= entry.register_indexes;
            max_pc = max_pc.max(entry.pc.max(entry.trampoline));
            max_deopt_index = max_deopt_index.max(entry.deopt_index);
        }

        // Derive the bytes and bools for the entry configuration from the values.
        let value_to_bytes = |value: i32| -> usize {
            assert!(value >= 0);
            if value == 0 {
                0
            } else if value <= 0xff {
                1
            } else if value <= 0xffff {
                2
            } else if value <= 0xffffff {
                3
            } else {
                4
            }
        };

        let has_deopt_data = max_deopt_index != -1;
        let register_indexes_size = value_to_bytes(used_register_indexes as i32);

        // Add 1 so all values (including kNoDeoptIndex and kNoTrampolinePC) are
        // non-negative.
        static_assert!(SafepointEntry::K_NO_DEOPT_INDEX == -1);
        static_assert!(SafepointEntry::K_NO_TRAMPOLINE_PC == -1);
        let pc_size = value_to_bytes(max_pc + 1);
        let deopt_index_size = value_to_bytes(max_deopt_index + 1);
        let tagged_slots_bytes = (tagged_slots_size as usize + K_BITS_PER_BYTE - 1) / K_BITS_PER_BYTE;

        // Add a CHECK to ensure we never overflow the space in the bitfield, even for
        // huge functions which might not be covered by tests.
        assert!(SafepointTable::RegisterIndexesSizeField::is_valid(
            register_indexes_size
        ));
        assert!(SafepointTable::PcSizeField::is_valid(pc_size));
        assert!(SafepointTable::DeoptIndexSizeField::is_valid(deopt_index_size));
        assert!(SafepointTable::TaggedSlotsBytesField::is_valid(
            tagged_slots_bytes
        ));

        let entry_configuration =
            SafepointTable::HasDeoptDataField::encode(has_deopt_data)
                | SafepointTable::RegisterIndexesSizeField::encode(register_indexes_size)
                | SafepointTable::PcSizeField::encode(pc_size)
                | SafepointTable::DeoptIndexSizeField::encode(deopt_index_size)
                | SafepointTable::TaggedSlotsBytesField::encode(tagged_slots_bytes);

        // Emit the table header.
        static_assert!(SafepointTable::k_stack_slots_offset() == 0 * K_INT_SIZE);
        static_assert!(SafepointTable::k_length_offset() == 1 * K_INT_SIZE);
        static_assert!(SafepointTable::k_entry_configuration_offset() == 2 * K_INT_SIZE);
        static_assert!(SafepointTable::k_header_size() == 3 * K_INT_SIZE);

        let length = self.entries_.len() as i32;
        assembler.dd(stack_slot_count);
        assembler.dd(length);
        assembler.dd(entry_configuration as i32);

        let emit_bytes = |assembler: &mut Assembler, value: i32, bytes: usize| {
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
                static_assert!(SafepointEntry::K_NO_DEOPT_INDEX == -1);
                static_assert!(SafepointEntry::K_NO_TRAMPOLINE_PC == -1);
                emit_bytes(assembler, entry.deopt_index + 1, deopt_index_size);
                emit_bytes(assembler, entry.trampoline + 1, pc_size);
            }
            emit_bytes(assembler, entry.register_indexes as i32, register_indexes_size);
        }

        // Emit bitmaps of tagged stack slots. Note the slot list is reversed in the
        // encoding.
        // TODO(jgruber): Avoid building a reversed copy of the bit vector.
        let mut bits: Vec<u8> = vec![0; tagged_slots_bytes];

        for entry in &self.entries_ {
            bits.iter_mut().for_each(|b| *b = 0);

            // Run through the indexes and build a bitmap.
            for &idx in entry.stack_indexes.as_ref().unwrap() {
                // The encoding is compacted by translating stack slot indices s.t. they
                // start at 0. See also above.
                let adjusted_idx = idx - self.min_stack_index();
                assert!(tagged_slots_size > adjusted_idx);

                let index = tagged_slots_size as usize - 1 - adjusted_idx as usize;
                let byte_index = index >> K_BITS_PER_BYTE.trailing_zeros() as usize; // Equivalent to index / kBitsPerByte
                let bit_index = index & (K_BITS_PER_BYTE - 1); // Equivalent to index % kBitsPerByte

                bits[byte_index] |= 1u8 << bit_index;
            }

            // Emit the bitmap for the current entry.
            for &byte in &bits {
                assembler.db(byte);
            }
        }
    }

    fn remove_duplicates(&mut self) {
        if self.entries_.len() < 2 {
            return;
        }

        let is_identical_except_for_pc = |entry1: &EntryBuilder, entry2: &EntryBuilder| -> bool {
            if entry1.deopt_index != entry2.deopt_index {
                return false;
            }
            assert_eq!(entry1.trampoline, entry2.trampoline);
            return entry1.register_indexes == entry2.register_indexes
                && entry1.stack_indexes.as_ref().unwrap().equals(entry2.stack_indexes.as_ref().unwrap());
        };

        let mut remaining_it = self.entries_.iter_mut();
        let mut it = self.entries_.iter_mut();

        if let Some(mut remaining) = remaining_it.next() {
            while let Some(current) = it.next() {
                if !is_identical_except_for_pc(current, remaining) {
                    *remaining = std::mem::replace(current, EntryBuilder::new(self.zone_,0));
                    remaining = remaining_it.next().unwrap_or(current);
                }
            }
        }
        self.entries_.truncate(remaining_it.count());
    }

    fn min_stack_index(&self) -> i32 {
        0 //Placeholder, must search entries and find real min
    }
}

impl<'a> SafepointTableBuilder<'a> {
    struct Safepoint<'b> {
        entry: &'b mut EntryBuilder<'a>,
        builder: &'b mut SafepointTableBuilder<'a>,
    }
}

impl<'a> SafepointTableBuilder<'a>::Safepoint<'a> {
    fn with_registers(&mut self, indexes: u32) -> &mut Self {
        self.entry.register_indexes = indexes;
        self
    }

    fn with_tagged_slots<I>(&mut self, indexes: I) -> &mut Self
    where
        I: IntoIterator<Item = i32>,
    {
        let mut vec = TaggedSlots::new(self.builder.zone_);
        for idx in indexes {
            vec.push(idx);
            self.builder.max_stack_index_ = std::cmp::max(self.builder.max_stack_index_, idx);
        