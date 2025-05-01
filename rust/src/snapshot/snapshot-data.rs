// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::mem::size_of;
use std::ptr::null_mut;

pub mod internal {
    use super::*;
    use byteorder::{ByteOrder, LittleEndian};

    pub struct SerializedData {
        data_: *mut u8,
        size_: u32,
        owns_data_: bool,
    }

    impl SerializedData {
        pub fn new(data: *mut u8, size: i32) -> Self {
            SerializedData {
                data_: data,
                size_: size as u32,
                owns_data_: false,
            }
        }

        pub fn default() -> Self {
            SerializedData {
                data_: null_mut(),
                size_: 0,
                owns_data_: false,
            }
        }

        pub fn move_from(other: &mut SerializedData) -> Self {
            let data_ = other.data_;
            let size_ = other.size_;
            let owns_data_ = other.owns_data_;

            other.owns_data_ = false;

            SerializedData {
                data_: data_,
                size_: size_,
                owns_data_: owns_data_,
            }
        }

        pub fn get_magic_number(&self) -> u32 {
            self.get_header_value(Self::K_MAGIC_NUMBER_OFFSET)
        }

        const K_MAGIC_NUMBER_OFFSET: u32 = 0;
        const K_MAGIC_NUMBER: u32 = 0xC0DE0000 ^ crate::internal::ExternalReferenceTable::K_SIZE;

        fn set_header_value(&self, offset: u32, value: u32) {
            let data_ptr = self.data_ as *mut u8;
            let offset_ptr = unsafe { data_ptr.add(offset as usize) };
            let value_bytes = value.to_le_bytes();
            unsafe {
                std::ptr::copy_nonoverlapping(
                    value_bytes.as_ptr(),
                    offset_ptr,
                    value_bytes.len(),
                );
            }
        }

        fn get_header_value(&self, offset: u32) const -> u32 {
            let data_ptr = self.data_ as *const u8;
            let offset_ptr = unsafe { data_ptr.add(offset as usize) };
            let mut buffer = [0u8; 4];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    offset_ptr,
                    buffer.as_mut_ptr(),
                    buffer.len(),
                );
            }
            LittleEndian::read_u32(&buffer)
        }

        fn allocate_data(&mut self, size: u32) {
            self.data_ = unsafe {
                let layout = std::alloc::Layout::array::<u8>(size as usize).unwrap();
                std::alloc::alloc(layout)
            };
            self.size_ = size;
            self.owns_data_ = true;
        }

        fn set_magic_number(&self) {
            self.set_header_value(Self::K_MAGIC_NUMBER_OFFSET, Self::K_MAGIC_NUMBER);
        }
    }

    impl Drop for SerializedData {
        fn drop(&mut self) {
            if self.owns_data_ {
                unsafe {
                    let layout = std::alloc::Layout::array::<u8>(self.size_ as usize).unwrap();
                    std::alloc::dealloc(self.data_, layout);
                }
            }
        }
    }

    pub struct SnapshotData {
        serialized_data: SerializedData,
    }

    impl SnapshotData {
        //Used when producing
        pub fn new(serializer: *const ()) -> Self {
            // Serializer is unused in the current implementation,
            // but the signature is kept to match the C++ code.
            SnapshotData {
                serialized_data: SerializedData::default(),
            }
        }

        // Used when consuming.
        pub fn from_snapshot(snapshot: &[u8]) -> Self {
            SnapshotData {
                serialized_data: SerializedData::new(snapshot.as_ptr() as *mut u8, snapshot.len() as i32),
            }
        }

        pub fn payload(&self) -> &[u8] {
            let data_ptr = self.serialized_data.data_ as *const u8;
            let payload_start = unsafe { data_ptr.add(Self::K_HEADER_SIZE as usize) };
            unsafe { std::slice::from_raw_parts(payload_start, (self.serialized_data.size_ - Self::K_HEADER_SIZE) as usize) }
        }

        pub fn raw_data(&self) -> &[u8] {
            unsafe { std::slice::from_raw_parts(self.serialized_data.data_ as *const u8, self.serialized_data.size_ as usize) }
        }
    }

    impl SnapshotData {
        const K_PAYLOAD_LENGTH_OFFSET: u32 = SerializedData::K_MAGIC_NUMBER_OFFSET + Self::K_UINT32_SIZE;
        const K_HEADER_SIZE: u32 = Self::K_PAYLOAD_LENGTH_OFFSET + Self::K_UINT32_SIZE;
        const K_UINT32_SIZE: u32 = size_of::<u32>() as u32;
    }

    impl SnapshotData {
        // Resize used by SnapshotCompression so it can shrink the compressed
        // SnapshotData.
        fn resize(&mut self, size: u32) {
            self.serialized_data.size_ = size;
        }

        fn default() -> Self {
            SnapshotData {
                serialized_data: SerializedData::default(),
            }
        }
    }

    pub mod ExternalReferenceTable {
        pub const K_SIZE: u32 = 1234;
    }
}