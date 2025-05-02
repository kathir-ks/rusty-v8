// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod output_stream_writer {
    use std::cmp;
    use std::fmt::Display;
    use std::num::TryFromIntError;
    use std::string::String;
    use std::vec::Vec;

    // Placeholder for v8::OutputStream, replace with a real Rust type if possible
    pub trait OutputStream {
        fn get_chunk_size(&self) -> usize;
        fn write_ascii_chunk(&mut self, data: &[u8]) -> OutputStreamWriteResult;
        fn end_of_stream(&mut self);
    }

    #[derive(PartialEq)]
    pub enum OutputStreamWriteResult {
        Continue,
        Abort,
    }

    pub struct OutputStreamWriter<'a> {
        stream_: &'a mut dyn OutputStream,
        chunk_size_: usize,
        chunk_: Vec<u8>,
        chunk_pos_: usize,
        aborted_: bool,
    }

    impl<'a> OutputStreamWriter<'a> {
        pub fn new(stream: &'a mut dyn OutputStream) -> Self {
            let chunk_size_ = stream.get_chunk_size();
            assert!(chunk_size_ > 0);
            OutputStreamWriter {
                stream_: stream,
                chunk_size_: chunk_size_,
                chunk_: vec![0; chunk_size_],
                chunk_pos_: 0,
                aborted_: false,
            }
        }

        pub fn aborted(&self) -> bool {
            self.aborted_
        }

        pub fn add_character(&mut self, c: char) -> Result<(), TryFromIntError> {
            assert_ne!(c, '\0');
            assert!(self.chunk_pos_ < self.chunk_size_);
            self.chunk_[self.chunk_pos_] = c as u8;
            self.chunk_pos_ += 1;
            self.maybe_write_chunk();
            Ok(())
        }

        pub fn add_string(&mut self, s: &str) {
            let mut s_ptr = s;
            while !s_ptr.is_empty() {
                let s_chunk_size = cmp::min(
                    self.chunk_size_ - self.chunk_pos_,
                    s_ptr.len(),
                );
                assert!(s_chunk_size > 0);

                let (to_write, remaining) = s_ptr.split_at(s_chunk_size);
                self.chunk_[self.chunk_pos_..self.chunk_pos_ + s_chunk_size].copy_from_slice(to_write.as_bytes());

                self.chunk_pos_ += s_chunk_size;
                s_ptr = remaining;
                self.maybe_write_chunk();
            }
        }

        pub fn add_number<T: Display>(&mut self, n: T) {
            let mut buffer = itoa::Buffer::new();
            let s = buffer.format(n);

            self.add_string(s);
        }

        pub fn finalize(&mut self) {
            if self.aborted_ {
                return;
            }
            assert!(self.chunk_pos_ <= self.chunk_size_);
            if self.chunk_pos_ != 0 {
                self.write_chunk();
            }
            self.stream_.end_of_stream();
        }

        fn maybe_write_chunk(&mut self) {
            assert!(self.chunk_pos_ <= self.chunk_size_);
            if self.chunk_pos_ == self.chunk_size_ {
                self.write_chunk();
            }
        }

        fn write_chunk(&mut self) {
            if self.aborted_ {
                return;
            }
            if self.stream_.write_ascii_chunk(&self.chunk_[..self.chunk_pos_]) == OutputStreamWriteResult::Abort {
                self.aborted_ = true;
            }
            self.chunk_pos_ = 0;
        }
    }
}