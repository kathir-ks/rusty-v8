// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod wasm {
    use std::cmp;
    use std::mem::MaybeUninit;
    use std::slice;
    use std::str;
    use std::string::String;
    use std::string::ToString;
    use std::vec::Vec;

    /// Similar to std::ostringstream, but faster.
    /// This struct works best for small-ish strings (up to kChunkSize); for
    /// producing large amounts of text, you probably want a subclass like
    /// MultiLineStringBuilder.
    pub struct StringBuilder {
        chunks: Vec<Box<[u8]>>, // A very simple Zone, essentially.
        start: *mut u8,
        cursor: *mut u8,
        remaining_bytes: usize,
        on_growth: OnGrowth,
        stack_buffer: Box<[u8; Self::K_STACK_SIZE]>,
        use_stack: bool,
    }

    #[derive(Clone, Copy)]
    enum OnGrowth {
        KeepOldChunks,
        ReplacePreviousChunk,
    }

    impl StringBuilder {
        const K_STACK_SIZE: usize = 256;
        const K_CHUNK_SIZE: usize = 1024 * 1024;

        /// Creates a new StringBuilder with the default settings.
        pub fn new() -> Self {
            let mut stack_buffer = Box::new([0u8; Self::K_STACK_SIZE]);
            let start = stack_buffer.as_mut_ptr();
            let cursor = start;
            StringBuilder {
                chunks: Vec::new(),
                start,
                cursor,
                remaining_bytes: Self::K_STACK_SIZE,
                on_growth: OnGrowth::ReplacePreviousChunk,
                stack_buffer,
                use_stack: true,
            }
        }

        /// Creates a new StringBuilder with a specific growth strategy.
        fn with_growth(on_growth: OnGrowth) -> Self {
            let mut stack_buffer = Box::new([0u8; Self::K_STACK_SIZE]);
            let start = stack_buffer.as_mut_ptr();
            let cursor = start;

            StringBuilder {
                chunks: Vec::new(),
                start,
                cursor,
                remaining_bytes: Self::K_STACK_SIZE,
                on_growth,
                stack_buffer,
                use_stack: true,
            }
        }

        /// Reserves space for {n} characters and returns a pointer to its beginning.
        /// Clients *must* write all {n} characters after calling this!
        /// Don't call this directly, use operator<< overloads instead.
        #[allow(clippy::mut_from_ref)]
        #[inline]
        fn allocate(&mut self, n: usize) -> *mut u8 {
            if self.remaining_bytes < n {
                self.grow(n);
            }
            let result = self.cursor;
            self.cursor = unsafe { self.cursor.add(n) };
            self.remaining_bytes -= n;
            result
        }

        /// Convenience wrapper for writing a byte slice.
        #[inline]
        pub fn write_bytes(&mut self, data: &[u8]) {
            let n = data.len();
            let ptr = self.allocate(n);
            unsafe {
                std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, n);
            }
        }

        /// Convenience wrapper for writing a string slice.
        #[inline]
        pub fn write_str(&mut self, data: &str) {
            self.write_bytes(data.as_bytes());
        }

        /// Returns a pointer to the beginning of the string.
        #[inline]
        pub fn start(&self) -> *const u8 {
            self.start as *const u8
        }

        /// Returns a pointer to the current cursor position.
        #[inline]
        pub fn cursor(&self) -> *const u8 {
            self.cursor as *const u8
        }

        /// Returns the length of the string.
        #[inline]
        pub fn len(&self) -> usize {
            unsafe { self.cursor.offset_from(self.start) as usize }
        }

        /// Rewinds the cursor to the beginning of the string.
        #[inline]
        pub fn rewind_to_start(&mut self) {
            self.remaining_bytes += self.len();
            self.cursor = self.start;
        }

        /// Erases the last character that was written. Calling this repeatedly
        /// isn't safe due to internal chunking of the backing store.
        #[inline]
        pub fn backspace(&mut self) {
            if self.cursor > self.start {
                unsafe {
                    self.cursor = self.cursor.sub(1);
                }
                self.remaining_bytes += 1;
            }
        }

        /// Returns the approximate size in MB.
        #[inline]
        fn approximate_size_mb(&self) -> usize {
            self.chunks.len()
        }

        #[inline]
        fn start_here(&mut self) {
            self.start = self.cursor;
        }

        fn grow(&mut self, requested: usize) {
            let used = self.len();
            let required = used + requested;

            let mut chunk_size;
            if self.on_growth == OnGrowth::KeepOldChunks {
                // Usually grow by kChunkSize, unless super-long lines need even more.
                chunk_size = cmp::max(Self::K_CHUNK_SIZE, required * 2);
            } else {
                // When we only have one chunk, always (at least) double its size
                // when it grows, to minimize both wasted memory and growth effort.
                chunk_size = required * 2;
            }
            let mut new_chunk = vec![0u8; chunk_size].into_boxed_slice();
            unsafe {
                std::ptr::copy_nonoverlapping(self.start as *const u8, new_chunk.as_mut_ptr(), used);
            }

            if self.on_growth == OnGrowth::KeepOldChunks {
                self.chunks.push(new_chunk);
            } else {
                if self.use_stack {
                    self.use_stack = false;
                } else {
                    //Find the correct chunk and replace
                    let last_index = self.chunks.len() - 1;
                    self.chunks[last_index] = new_chunk;
                }
            }

            self.start = new_chunk.as_mut_ptr();
            self.cursor = unsafe { self.start.add(used) };
            self.remaining_bytes = chunk_size - used;
        }

        pub fn to_string(&self) -> String {
            unsafe {
                let slice = slice::from_raw_parts(self.start as *const u8, self.len());
                String::from_utf8_lossy(slice).into_owned()
            }
        }
    }

    impl Default for StringBuilder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl std::fmt::Write for StringBuilder {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.write_str(s);
            Ok(())
        }
    }
    
    // Overload << operator for &str
    impl StringBuilder {
        pub fn push_str(&mut self, str: &str) -> &mut Self {
            self.write_str(str);
            self
        }
        
        pub fn push_char(&mut self, c: char) -> &mut Self {
            let mut buffer = [0u8; 4];
            let len = c.encode_utf8(&mut buffer).len();
            self.write_bytes(&buffer[..len]);
            self
        }
    
        pub fn push_string(&mut self, s: &String) -> &mut Self {
            self.write_bytes(s.as_bytes());
            self
        }
    
        pub fn push_string_view(&mut self, s: &str) -> &mut Self {
            self.write_bytes(s.as_bytes());
            self
        }
    
        pub fn push_u32(&mut self, n: u32) -> &mut Self {
            if n == 0 {
                self.push_char('0');
                return self;
            }
    
            let mut buffer = [0u8; 10];
            let mut end = buffer.len();
            let mut num = n;
    
            while num != 0 {
                end -= 1;
                buffer[end] = b'0' + (num % 10) as u8;
                num /= 10;
            }
    
            self.write_bytes(&buffer[end..]);
            self
        }
    
        pub fn push_i32(&mut self, value: i32) -> &mut Self {
            if value >= 0 {
                self.push_u32(value as u32);
            } else {
                self.push_str("-");
                self.push_u32(((!value as u32) + 1));
            }
            self
        }
    }
}