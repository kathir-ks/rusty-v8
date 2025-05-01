// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod packet {
    /// Represents a GDB server packet.
    pub struct Packet {
        seq: Option<i32>,
        data: String,
        read_index: usize,
    }

    impl Packet {
        /// Creates a new empty packet.
        pub fn new() -> Self {
            Packet {
                seq: None,
                data: String::new(),
                read_index: 0,
            }
        }

        /// Clears the packet, resetting its contents and read pointer.
        pub fn clear(&mut self) {
            self.data.clear();
            self.read_index = 0;
            self.seq = None;
        }

        /// Resets the read pointer to the beginning of the packet.
        pub fn rewind(&mut self) {
            self.read_index = 0;
        }

        /// Checks if the read pointer has reached the end of the packet.
        pub fn end_of_packet(&self) -> bool {
            self.read_index >= self.data.len()
        }

        /// Adds a raw 8-bit character to the packet.
        pub fn add_raw_char(&mut self, ch: char) {
            self.data.push(ch);
        }

        /// Adds a block of data to the packet as hexadecimal pairs.
        pub fn add_block(&mut self, ptr: &[u8]) {
            for byte in ptr {
                self.add_word8(*byte);
            }
        }

        /// Adds a byte to the packet as a two-character hexadecimal representation.
        pub fn add_word8(&mut self, val: u8) {
            self.data.push_str(&format!("{:02x}", val));
        }

        /// Adds a number to the packet as a big-endian hexadecimal string,
        /// removing leading zeros and adding a separator character.
        pub fn add_number_sep(&mut self, val: u64, sep: char) {
            let hex_string = format!("{:x}", val);
            let trimmed_hex_string = hex_string.trim_start_matches('0');
            self.data.push_str(trimmed_hex_string);
            self.data.push(sep);
        }

        /// Adds a raw string to the packet.
        pub fn add_string(&mut self, str: &str) {
            self.data.push_str(str);
        }

        /// Adds a string to the packet as a stream of ASCII hexadecimal digit pairs.
        pub fn add_hex_string(&mut self, str: &str) {
            for ch in str.chars() {
                self.add_word8(ch as u8);
            }
        }

        /// Retrieves a single character from the packet.
        pub fn get_raw_char(&mut self, ch: &mut char) -> bool {
            if self.read_index < self.data.len() {
                if let Some(c) = self.data.chars().nth(self.read_index) {
                    *ch = c;
                    self.read_index += 1;
                    return true;
                }
            }
            false
        }

        /// Retrieves a block of data from the packet as ASCII character pairs.
        pub fn get_block(&mut self, ptr: &mut [u8]) -> bool {
            if self.read_index + (ptr.len() * 2) > self.data.len() {
                return false;
            }

            for i in 0..ptr.len() {
                let mut val: u8 = 0;
                if !self.get_word8(&mut val) {
                    return false;
                }
                ptr[i] = val;
            }
            true
        }

        /// Retrieves an 8-bit word from the packet as a pair of hexadecimal digits.
        pub fn get_word8(&mut self, val: &mut u8) -> bool {
            if self.read_index + 2 > self.data.len() {
                return false;
            }

            let hex_str = &self.data[self.read_index..self.read_index + 2];
            if let Ok(parsed_val) = u8::from_str_radix(hex_str, 16) {
                *val = parsed_val;
                self.read_index += 2;
                true
            } else {
                false
            }
        }

        /// Retrieves a number and separator from the packet.
        pub fn get_number_sep(&mut self, val: &mut u64, sep: &mut char) -> bool {
            let mut hex_str = String::new();
            while self.read_index < self.data.len() {
                let current_char = self.data.chars().nth(self.read_index).unwrap();
                if current_char.is_ascii_hexdigit() {
                    hex_str.push(current_char);
                    self.read_index += 1;
                } else {
                    *sep = current_char;
                    self.read_index += 1;
                    break;
                }
            }

            if hex_str.is_empty() {
                return false;
            }

            if let Ok(parsed_val) = u64::from_str_radix(&hex_str, 16) {
                *val = parsed_val;
                return true;
            }

            false
        }

        /// Retrieves a string from the packet.
        pub fn get_string(&mut self, str: &mut String) -> bool {
            let start_index = self.read_index;
            while self.read_index < self.data.len() {
                if self.data.chars().nth(self.read_index).unwrap() == '\0' {
                    break;
                }
                self.read_index += 1;
            }

            if self.read_index > start_index {
                *str = self.data[start_index..self.read_index].to_string();
                self.read_index += 1; // Skip null terminator
                return true;
            }

            false
        }

        /// Retrieves a string from the packet as a stream of ASCII hexadecimal digits.
        pub fn get_hex_string(&mut self, str: &mut String) -> bool {
            let mut temp_str = String::new();
            while self.read_index + 2 <= self.data.len() {
                let hex_str = &self.data[self.read_index..self.read_index + 2];
                if let Ok(byte) = u8::from_str_radix(hex_str, 16) {
                    temp_str.push(byte as char);
                    self.read_index += 2;
                } else {
                    break;
                }
            }
            *str = temp_str;
            !str.is_empty()
        }

        /// Returns a pointer to the packet payload.
        pub fn get_payload(&self) -> &str {
            &self.data
        }

        /// Returns the size of the packet payload.
        pub fn get_payload_size(&self) -> usize {
            self.data.len()
        }

        /// Gets the sequence number, if it is set.
        pub fn get_sequence(&self, seq: &mut i32) -> bool {
            match self.seq {
                Some(s) => {
                    *seq = s;
                    true
                }
                None => false,
            }
        }

        /// Parses the sequence number from the packet data and moves the read pointer.
        pub fn parse_sequence(&mut self) {
            // The implementation assumes that the sequence number is at the beginning
            // of the packet data and is followed by a comma.
            let mut seq_str = String::new();
            while self.read_index < self.data.len() {
                let current_char = self.data.chars().nth(self.read_index).unwrap();
                if current_char == ',' {
                    self.read_index += 1;
                    break;
                }
                seq_str.push(current_char);
                self.read_index += 1;
            }

            if let Ok(seq) = seq_str.parse::<i32>() {
                self.seq = Some(seq);
            }
        }

        /// Sets the sequence number for the packet.
        pub fn set_sequence(&mut self, seq: i32) {
            self.seq = Some(seq);
        }

        /// Enum to represent different error definitions
        #[derive(Debug, PartialEq)]
        pub enum ErrDef {
            None = 0,
            BadFormat = 1,
            BadArgs = 2,
            Failed = 3,
        }
        
        ///Set an error definition
        pub fn set_error(&mut self, _err: ErrDef) {
            //No specific operation to do, can log the error
        }

        /// Returns the full content of a GDB-remote packet, in the format:
        ///    $payload#checksum
        /// where the two-digit checksum is computed as the modulo 256 sum of all
        /// characters between the leading ‘$’ and the trailing ‘#’.
        pub fn get_packet_data(&self) -> String {
            let payload = &self.data;
            let checksum = self.calculate_checksum(payload);
            format!("${}#{:02x}", payload, checksum)
        }

        fn calculate_checksum(&self, payload: &str) -> u8 {
            let mut checksum: u32 = 0;
            for byte in payload.as_bytes() {
                checksum = (checksum + *byte as u32) % 256;
            }
            checksum as u8
        }
    }
}