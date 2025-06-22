// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::mem;
use std::slice;
use std::str;

// These constants should be defined in a common place
const K_UINT32_SIZE: usize = mem::size_of::<u32>();
const K_NULL_ADDRESS: usize = 0;

const K_OFFSET_TO_FIRST_COMMENT_ENTRY: u8 = K_UINT32_SIZE as u8;
const K_OFFSET_TO_PC_OFFSET: u8 = 0;
const K_OFFSET_TO_COMMENT_SIZE: u8 = K_OFFSET_TO_PC_OFFSET + K_UINT32_SIZE as u8;
const K_OFFSET_TO_COMMENT_STRING: u8 = K_OFFSET_TO_COMMENT_SIZE + K_UINT32_SIZE as u8;

/// Represents a code comment entry.
#[derive(Debug, Clone)]
pub struct CodeCommentEntry {
    pub pc_offset: u32,
    pub comment: String,
}

impl CodeCommentEntry {
    /// Calculates the length of the comment, including the null terminator.
    pub fn comment_length(&self) -> u32 {
        (self.comment.len() + 1) as u32
    }

    /// Calculates the total size of the comment entry in bytes.
    pub fn size(&self) -> u32 {
        K_OFFSET_TO_COMMENT_STRING as u32 + self.comment_length()
    }
}

/// Iterator for traversing code comments.
#[derive(Debug)]
pub struct CodeCommentsIterator<'a> {
    code_comments_start_: usize,
    code_comments_size_: u32,
    current_entry_: usize,
    _phantom: std::marker::PhantomData<&'a [u8]>,
}

impl<'a> CodeCommentsIterator<'a> {
    /// Creates a new `CodeCommentsIterator`.
    pub fn new(code_comments_start: usize, code_comments_size: u32) -> Self {
        assert_ne!(K_NULL_ADDRESS, code_comments_start);
        if code_comments_size != 0 {
            let size_at_start = unsafe { *(code_comments_start as *const u32) };
            assert_eq!(code_comments_size, size_at_start);
        }

        CodeCommentsIterator {
            code_comments_start_: code_comments_start,
            code_comments_size_: code_comments_size,
            current_entry_: code_comments_start + K_OFFSET_TO_FIRST_COMMENT_ENTRY as usize,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Returns the total size of the code comments section.
    pub fn size(&self) -> u32 {
        self.code_comments_size_
    }

    /// Retrieves the current comment as a string slice.
    pub fn get_comment(&self) -> &'a str {
        let comment_string_ptr = self.current_entry_ + K_OFFSET_TO_COMMENT_STRING as usize;
        let comment_size = self.get_comment_size() as usize;

        let slice = unsafe { slice::from_raw_parts(comment_string_ptr as *const u8, comment_size - 1) };
        str::from_utf8(slice).expect("Invalid UTF-8 sequence")
    }

    /// Retrieves the size of the current comment.
    pub fn get_comment_size(&self) -> u32 {
        unsafe { *(self.current_entry_ + K_OFFSET_TO_COMMENT_SIZE as usize as *const usize as *const u32) }
    }

    /// Retrieves the PC offset of the current comment.
    pub fn get_pc_offset(&self) -> u32 {
        unsafe { *(self.current_entry_ + K_OFFSET_TO_PC_OFFSET as usize as *const usize as *const u32) }
    }

    /// Advances the iterator to the next comment entry.
    pub fn next(&mut self) {
        self.current_entry_ += K_OFFSET_TO_COMMENT_STRING as usize + self.get_comment_size() as usize;
    }

    /// Checks if there is a current comment entry.
    pub fn has_current(&self) -> bool {
        self.current_entry_ < self.code_comments_start_ + self.size() as usize
    }
}

/// Writer for emitting code comments.
#[derive(Debug, Default)]
pub struct CodeCommentsWriter {
    comments_: Vec<CodeCommentEntry>,
    byte_count_: usize,
}

impl CodeCommentsWriter {
    /// Creates a new `CodeCommentsWriter`.
    pub fn new() -> Self {
        CodeCommentsWriter {
            comments_: Vec::new(),
            byte_count_: 0,
        }
    }

    /// Emits the code comments using the given Assembler.
    // TODO: Implement Assembler in Rust.  This is a placeholder.
    pub fn emit(&self/*, assm: &mut Assembler*/) {
        // assm.dd(self.section_size());
        // for i in &self.comments_ {
        //   assm.dd(i.pc_offset);
        //   assm.dd(i.comment_length());
        //   for c in i.comment.chars() {
        //     //EnsureSpace ensure_space(assm); // what is this?
        //     assm.db(c as u8);
        //   }
        //   assm.db('\0' as u8);
        // }

        //This is a dummy printout, as the Assembler is missing
        println!("Section size: {}", self.section_size());
        for i in &self.comments_ {
          println!("PC Offset: {}", i.pc_offset);
          println!("Comment length: {}", i.comment_length());
          println!("Comment: {}", i.comment);
        }
    }

    /// Adds a new code comment entry.
    pub fn add(&mut self, pc_offset: u32, comment: String) {
        let entry = CodeCommentEntry {
            pc_offset,
            comment
        };
        self.byte_count_ += entry.size() as usize;
        self.comments_.push(entry);
    }

    /// Returns the number of comment entries.
    pub fn entry_count(&self) -> usize {
        self.comments_.len()
    }

    /// Returns the total size of the code comments section.
    pub fn section_size(&self) -> u32 {
        K_OFFSET_TO_FIRST_COMMENT_ENTRY as u32 + self.byte_count_ as u32
    }
}