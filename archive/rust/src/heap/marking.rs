// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    fmt,
    marker::PhantomData,
    mem,
    ops::{BitAnd, BitOr, Not, Sub},
};

// Placeholder for src/heap/marking-inl.h content
// In a real conversion, this would define traits and structs used here.

pub struct MarkBitIndex(usize);
pub struct CellIndex(usize);
pub struct Address(usize);

pub struct MarkBit {
    index: usize,
}

impl MarkBit {
    pub fn from_for_testing(address: Address) -> Self {
        MarkingBitmap::mark_bit_from_address(address)
    }

    pub fn from_for_testing_heap_object(heap_object_address: Address) -> Self {
        MarkingBitmap::mark_bit_from_address(heap_object_address)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MarkingBitmap {
    cells: Vec<MarkBit::CellType>,
    k_cells_count: usize, // Storing this as a member since it is used often.
}

impl MarkingBitmap {
    pub const K_BITS_PER_CELL: usize = std::mem::size_of::<MarkBit::CellType>() * 8;
    // TODO: Initialize cells in the constructor.
    pub fn new(cells_count: usize) -> Self {
        MarkingBitmap {
            cells: vec![0; cells_count],
            k_cells_count: cells_count,
        }
    }

    pub fn cells(&self) -> &Vec<MarkBit::CellType> {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut Vec<MarkBit::CellType> {
        &mut self.cells
    }

    pub fn index_to_cell(&self, index: MarkBitIndex) -> CellIndex {
        CellIndex(index.0 / Self::K_BITS_PER_CELL)
    }

    pub fn index_in_cell_mask(&self, index: MarkBitIndex) -> MarkBit::CellType {
        1 << (index.0 % Self::K_BITS_PER_CELL)
    }

    pub fn all_bits_set_in_range(&self, start_index: MarkBitIndex, end_index: MarkBitIndex) -> bool {
        if start_index.0 >= end_index.0 {
            return false;
        }
        let mut end_index = MarkBitIndex(end_index.0 - 1);

        let start_cell_index = self.index_to_cell(start_index);
        let start_index_mask = self.index_in_cell_mask(start_index);
        let end_cell_index = self.index_to_cell(end_index);
        let end_index_mask = self.index_in_cell_mask(end_index);

        let matching_mask: MarkBit::CellType;
        if start_cell_index != end_cell_index {
            matching_mask = !(start_index_mask.wrapping_sub(1));
            if (self.cells[start_cell_index.0] & matching_mask) != matching_mask {
                return false;
            }
            for i in start_cell_index.0 + 1..end_cell_index.0 {
                if self.cells[i] != MarkBit::K_ALL_BITS_SET_IN_CELL_VALUE {
                    return false;
                }
            }
            matching_mask = end_index_mask | (end_index_mask.wrapping_sub(1));
            return (self.cells[end_cell_index.0] & matching_mask) == matching_mask;
        } else {
            matching_mask = end_index_mask | (end_index_mask.wrapping_sub(start_index_mask));
            return (self.cells[end_cell_index.0] & matching_mask) == matching_mask;
        }
    }

    pub fn all_bits_clear_in_range(&self, start_index: MarkBitIndex, end_index: MarkBitIndex) -> bool {
        if start_index.0 >= end_index.0 {
            return true;
        }
        let mut end_index = MarkBitIndex(end_index.0 - 1);

        let start_cell_index = self.index_to_cell(start_index);
        let start_index_mask = self.index_in_cell_mask(start_index);
        let end_cell_index = self.index_to_cell(end_index);
        let end_index_mask = self.index_in_cell_mask(end_index);

        let matching_mask: MarkBit::CellType;
        if start_cell_index != end_cell_index {
            matching_mask = !(start_index_mask.wrapping_sub(1));
            if (self.cells[start_cell_index.0] & matching_mask) != 0 {
                return false;
            }
            for i in start_cell_index.0 + 1..end_cell_index.0 {
                if self.cells[i] != 0 {
                    return false;
                }
            }
            matching_mask = end_index_mask | (end_index_mask.wrapping_sub(1));
            return (self.cells[end_cell_index.0] & matching_mask) == 0;
        } else {
            matching_mask = end_index_mask | (end_index_mask.wrapping_sub(start_index_mask));
            return (self.cells[end_cell_index.0] & matching_mask) == 0;
        }
    }

    pub fn print(&self) {
        let mut printer = CellPrinter::default();
        for i in 0..self.k_cells_count {
            printer.print(i, self.cells[i]);
        }
        printer.flush();
        println!("");
    }

    pub fn is_clean(&self) -> bool {
        for i in 0..self.k_cells_count {
            if self.cells[i] != 0 {
                return false;
            }
        }
        true
    }

    pub fn mark_bit_from_address(address: Address) -> MarkBit {
        MarkBit { index: address.0 }
    }
}

impl MarkBit {
    pub type CellType = usize;
    pub const K_ALL_BITS_SET_IN_CELL_VALUE: CellType = std::usize::MAX;
}

fn print_word(word: MarkBit::CellType, himask: MarkBit::CellType) {
    let mut mask: MarkBit::CellType = 1;
    while mask != 0 {
        if (mask & himask) != 0 {
            print!("[");
        }
        print!("{}", if (mask & word) != 0 { "1" } else { "0" });
        if (mask & himask) != 0 {
            print!("]");
        }
        mask <<= 1;
    }
}

#[derive(Default)]
struct CellPrinter {
    seq_start: usize,
    seq_type: MarkBit::CellType,
    seq_length: usize,
}

impl CellPrinter {
    fn print(&mut self, pos: usize, cell: MarkBit::CellType) {
        if cell == self.seq_type {
            self.seq_length += 1;
            return;
        }

        self.flush();

        if Self::is_seq(cell) {
            self.seq_start = pos;
            self.seq_length = 0;
            self.seq_type = cell;
            return;
        }

        print!("{}: ", pos);
        print_word(cell, 0);
        println!("");
    }

    fn flush(&mut self) {
        if self.seq_length > 0 {
            println!(
                "{}: {}x{}",
                self.seq_start,
                if self.seq_type == 0 { 0 } else { 1 },
                self.seq_length * MarkingBitmap::K_BITS_PER_CELL
            );
            self.seq_length = 0;
        }
    }

    fn is_seq(cell: MarkBit::CellType) -> bool {
        cell == 0 || cell == MarkBit::K_ALL_BITS_SET_IN_CELL_VALUE
    }
}

impl PartialEq for MarkBitIndex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for MarkBitIndex {}

impl PartialEq for CellIndex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for CellIndex {}