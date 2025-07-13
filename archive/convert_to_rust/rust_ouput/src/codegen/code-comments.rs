// Converted from V8 C++ source files:
// Header: code-comments.h
// Implementation: code-comments.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod macros {
        #[macro_export]
        macro_rules! DCHECK_NE {
            ($val1:expr, $val2:expr) => {
                if $val1 == $val2 {
                    panic!("DCHECK_NE failed: {} == {}", stringify!($val1), stringify!($val2));
                }
            };
        }
        #[macro_export]
        macro_rules! DCHECK_IMPLIES {
            ($condition:expr, $implication:expr) => {
                if $condition && !$implication {
                    panic!("DCHECK_IMPLIES failed: {} implies {}", stringify!($condition), stringify!($implication));
                }
            };
        }
    }
}

pub mod codegen {
    use std::{mem::size_of, string::String, vec::Vec};

    const K_UINT32_SIZE: u8 = 4;

    pub struct CodeCommentEntry {
        pub pc_offset: u32,
        pub comment: String,
    }

    impl CodeCommentEntry {
        pub fn comment_length(&self) -> u32 {
            (self.comment.len() + 1) as u32
        }

        pub fn size(&self) -> u32 {
            K_OFFSET_TO_COMMENT_STRING as u32 + self.comment_length()
        }
    }

    pub struct CodeCommentsWriter {
        byte_count_: u32,
        comments_: Vec<CodeCommentEntry>,
    }

    impl CodeCommentsWriter {
        pub fn new() -> Self {
            CodeCommentsWriter {
                byte_count_: 0,
                comments_: Vec::new(),
            }
        }

        pub fn add(&mut self, pc_offset: u32, comment: String) {
            let entry = CodeCommentEntry {
                pc_offset,
                comment,
            };
            self.byte_count_ += entry.size();
            self.comments_.push(entry);
        }

        pub fn emit(&self, assm: &mut Assembler) {
            assm.dd(self.section_size());
            for i in &self.comments_ {
                assm.dd(i.pc_offset);
                assm.dd(i.comment_length());
                for c in i.comment.chars() {
                    assm.db(c as u8);
                }
                assm.db(0);
            }
        }

        pub fn entry_count(&self) -> usize {
            self.comments_.len()
        }

        pub fn section_size(&self) -> u32 {
            K_OFFSET_TO_FIRST_COMMENT_ENTRY as u32 + self.byte_count_
        }
    }

    pub struct CodeCommentsIterator {
        code_comments_start_: usize,
        code_comments_size_: u32,
        current_entry_: usize,
    }

    impl CodeCommentsIterator {
        pub fn new(code_comments_start: usize, code_comments_size: u32) -> Self {
            assert_ne!(code_comments_start, 0);
            if code_comments_size != 0 {
                let size_from_start = unsafe { *(code_comments_start as *const u32) };
                assert_eq!(code_comments_size, size_from_start);
            }

            CodeCommentsIterator {
                code_comments_start_: code_comments_start,
                code_comments_size_: code_comments_size,
                current_entry_: code_comments_start + K_OFFSET_TO_FIRST_COMMENT_ENTRY as usize,
            }
        }

        pub fn size(&self) -> u32 {
            self.code_comments_size_
        }

        pub fn get_comment(&self) -> &str {
            let comment_string_ptr = self.current_entry_ + K_OFFSET_TO_COMMENT_STRING as usize;
            let comment_size = self.get_comment_size() as usize;
            let slice = unsafe { std::slice::from_raw_parts(comment_string_ptr as *const u8, comment_size - 1) };
            std::str::from_utf8(slice).unwrap()
        }

        pub fn get_comment_size(&self) -> u32 {
            unsafe { *(self.current_entry_ + K_OFFSET_TO_COMMENT_SIZE as usize as *const u32) }
        }

        pub fn get_pc_offset(&self) -> u32 {
            unsafe { *(self.current_entry_ + K_OFFSET_TO_PC_OFFSET as usize as *const u32) }
        }

        pub fn next(&mut self) {
            self.current_entry_ += self.get_comment_size() as usize + K_OFFSET_TO_COMMENT_STRING as usize;
        }

        pub fn has_current(&self) -> bool {
            self.current_entry_ < self.code_comments_start_ + self.size() as usize
        }
    }

    // Constants
    const K_OFFSET_TO_FIRST_COMMENT_ENTRY: u8 = K_UINT32_SIZE;
    const K_OFFSET_TO_PC_OFFSET: u8 = 0;
    const K_OFFSET_TO_COMMENT_SIZE: u8 = K_OFFSET_TO_PC_OFFSET + K_UINT32_SIZE;
    const K_OFFSET_TO_COMMENT_STRING: u8 = K_OFFSET_TO_COMMENT_SIZE + K_UINT32_SIZE;

    pub struct Assembler {
        buffer: Vec<u8>,
    }

    impl Assembler {
        pub fn new() -> Self {
            Assembler {
                buffer: Vec::new(),
            }
        }

        pub fn dd(&mut self, value: u32) {
            let bytes = value.to_le_bytes();
            self.buffer.extend_from_slice(&bytes);
        }

        pub fn db(&mut self, value: u8) {
            self.buffer.push(value);
        }

        pub fn get_buffer(&self) -> &Vec<u8> {
            &self.buffer
        }
    }
    pub struct EnsureSpace<'a> {
        assm: &'a mut Assembler,
    }

    impl<'a> EnsureSpace<'a> {
        pub fn new(assm: &'a mut Assembler) -> Self {
            EnsureSpace { assm }
        }
    }

    impl<'a> Drop for EnsureSpace<'a> {
        fn drop(&mut self) {}
    }

}
