// Converted from V8 C++ source files:
// Header: snapshot-source-sink.h
// Implementation: snapshot-source-sink.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod snapshot_source_sink {
    use std::mem;
    use std::slice;

    pub struct SnapshotByteSource<'a> {
        data_: &'a [u8],
        length_: usize,
        position_: usize,
    }

    impl<'a> SnapshotByteSource<'a> {
        pub fn new(data: &'a [u8]) -> Self {
            SnapshotByteSource {
                data_: data,
                length_: data.len(),
                position_: 0,
            }
        }

        pub fn from_raw_parts(data: *const u8, length: usize) -> Self {
            unsafe {
                SnapshotByteSource {
                    data_: slice::from_raw_parts(data, length),
                    length_: length,
                    position_: 0,
                }
            }
        }

        pub fn has_more(&self) -> bool {
            self.position_ < self.length_
        }

        pub fn get(&mut self) -> Result<u8, String> {
            if self.position_ < self.length_ {
                let value = self.data_[self.position_];
                self.position_ += 1;
                Ok(value)
            } else {
                Err("Position out of bounds".to_string())
            }
        }

        pub fn peek(&self) -> Result<u8, String> {
            if self.position_ < self.length_ {
                Ok(self.data_[self.position_])
            } else {
                Err("Position out of bounds".to_string())
            }
        }

        pub fn advance(&mut self, by: usize) -> Result<(), String> {
            if self.position_ + by <= self.length_ {
                self.position_ += by;
                Ok(())
            } else {
                Err("Advance out of bounds".to_string())
            }
        }

        pub fn copy_raw(&mut self, to: &mut [u8]) -> Result<(), String> {
            let number_of_bytes = to.len();
            if self.position_ + number_of_bytes <= self.length_ {
                to.copy_from_slice(&self.data_[self.position_..self.position_ + number_of_bytes]);
                self.position_ += number_of_bytes;
                Ok(())
            } else {
                Err("CopyRaw out of bounds".to_string())
            }
        }

        pub fn copy_slots(&mut self, dest: &mut [usize]) -> Result<(), String> {
            let number_of_slots = dest.len();
            let number_of_bytes = number_of_slots * mem::size_of::<usize>();

            if self.position_ + number_of_bytes <= self.length_ {
                unsafe {
                    let src_ptr = self.data_[self.position_..].as_ptr() as *const usize;
                    let dest_ptr = dest.as_mut_ptr();
                    std::ptr::copy_nonoverlapping(src_ptr, dest_ptr, number_of_slots);
                }
                self.position_ += number_of_bytes;
                Ok(())
            } else {
                Err("CopySlots out of bounds".to_string())
            }
        }

        pub fn get_uint30(&mut self) -> Result<u32, String> {
            if self.position_ + 3 < self.length_ {
                let mut answer: u32 = self.data_[self.position_] as u32;
                answer |= (self.data_[self.position_ + 1] as u32) << 8;
                answer |= (self.data_[self.position_ + 2] as u32) << 16;
                answer |= (self.data_[self.position_ + 3] as u32) << 24;

                let bytes = (answer & 3) + 1;
                self.advance(bytes as usize)?;

                let mask: u32 = 0xffffffffu;
                let mask = mask >> (32 - (bytes << 3));
                answer &= mask;
                answer >>= 2;
                Ok(answer)
            } else {
                Err("GetUint30 out of bounds".to_string())
            }
        }

        pub fn get_uint32(&mut self) -> Result<u32, String> {
            let mut integer_bytes: [u8; 4] = [0; 4];
            self.copy_raw(&mut integer_bytes)?;
            let integer = u32::from_ne_bytes(integer_bytes);
            Ok(integer)
        }

        pub fn get_blob(&mut self) -> Result<&'a [u8], String> {
            let size = self.get_uint30()? as usize;
            if self.position_ + size <= self.length_ {
                let blob = &self.data_[self.position_..self.position_ + size];
                self.advance(size)?;
                Ok(blob)
            } else {
                Err("GetBlob out of bounds".to_string())
            }
        }

        pub fn position(&self) -> usize {
            self.position_
        }

        pub fn set_position(&mut self, position: usize) {
            self.position_ = position;
        }

        pub fn data(&self) -> &'a [u8] {
            self.data_
        }

        pub fn length(&self) -> usize {
            self.length_
        }
    }

    pub struct SnapshotByteSink {
        data_: Vec<u8>,
    }

    impl SnapshotByteSink {
        pub fn new() -> Self {
            SnapshotByteSink { data_: Vec::new() }
        }

        pub fn with_capacity(initial_size: usize) -> Self {
            SnapshotByteSink {
                data_: Vec::with_capacity(initial_size),
            }
        }

        pub fn put(&mut self, b: u8, _description: &str) {
            self.data_.push(b);
        }

        pub fn put_n(&mut self, number_of_bytes: usize, v: u8, _description: &str) {
            self.data_.extend(std::iter::repeat(v).take(number_of_bytes));
        }

        pub fn put_uint30(&mut self, integer: u32, _description: &str) {
            assert!(integer < (1 << 30));
            let mut integer = integer << 2;
            let mut bytes = 1;
            if integer > 0xFF {
                bytes = 2;
            }
            if integer > 0xFFFF {
                bytes = 3;
            }
            if integer > 0xFFFFFF {
                bytes = 4;
            }
            integer |= (bytes - 1) as u32;

            self.put((integer & 0xFF) as u8, "IntPart1");
            if bytes > 1 {
                self.put(((integer >> 8) & 0xFF) as u8, "IntPart2");
            }
            if bytes > 2 {
                self.put(((integer >> 16) & 0xFF) as u8, "IntPart3");
            }
            if bytes > 3 {
                self.put(((integer >> 24) & 0xFF) as u8, "IntPart4");
            }
        }

        pub fn put_uint32(&mut self, integer: u32, description: &str) {
            let bytes = integer.to_ne_bytes();
            self.put_raw(&bytes, description);
        }

        pub fn put_raw(&mut self, data: &[u8], _description: &str) {
            self.data_.extend_from_slice(data);
        }

        pub fn append(&mut self, other: &SnapshotByteSink) {
            self.data_.extend_from_slice(&other.data_);
        }

        pub fn position(&self) -> usize {
            self.data_.len()
        }

        pub fn data(&self) -> &Vec<u8> {
            &self.data_
        }
    }
}
