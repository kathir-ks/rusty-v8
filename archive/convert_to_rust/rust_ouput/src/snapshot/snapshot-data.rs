// Converted from V8 C++ source files:
// Header: snapshot-data.h
// Implementation: snapshot-data.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct BitField<T, const OFFSET: usize, const WIDTH: usize>(T);
    impl<T, const OFFSET: usize, const WIDTH: usize> BitField<T, OFFSET, WIDTH> {
        pub fn new(value: T) -> Self {
            BitField(value)
        }
    }
    pub type Vector<T> = Vec<T>;
    pub fn WriteLittleEndianValue(ptr: usize, value: u32) {
        unsafe {
            let ptr = ptr as *mut u8;
            (ptr as *mut u32).write_unaligned(value.to_le());
        }
    }
    pub fn ReadLittleEndianValue<T: Copy>(ptr: usize) -> T {
        unsafe {
            let ptr = ptr as *mut u8;
            (ptr as *mut T).read_unaligned()
        }
    }
}

pub mod codegen {
    pub struct ExternalReferenceTable {}
    impl ExternalReferenceTable {
        pub const kSize: u32 = 10;
    }
}

pub mod utils {
    pub fn memcopy(dest: *mut u8, src: *const u8, n: usize) {
        unsafe {
            std::ptr::copy_nonoverlapping(src, dest, n);
        }
    }
}

pub mod common {
    pub struct AssertScope {}
    impl AssertScope {
        pub fn new() -> Self {
            AssertScope {}
        }
    }
}

pub mod snapshot {
    use crate::{
        base,
        codegen::ExternalReferenceTable,
        utils::memcopy,
        common::AssertScope
    };
    use std::mem::size_of;
    use std::ptr::null_mut;

    #[derive(Debug)]
    pub enum SnapshotError {
        AllocationError,
        InvalidData,
    }

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

        pub fn GetMagicNumber(&self) -> u32 {
            self.GetHeaderValue(Self::kMagicNumberOffset)
        }

        pub const kMagicNumberOffset: u32 = 0;
        pub const kMagicNumber: u32 =
            0xC0DE0000 ^ ExternalReferenceTable::kSize;

        fn SetHeaderValue(&mut self, offset: u32, value: u32) {
            base::WriteLittleEndianValue(self.data_ as usize + offset as usize, value);
        }

        fn GetHeaderValue(&self, offset: u32) -> u32 {
            base::ReadLittleEndianValue::<u32>(self.data_ as usize + offset as usize)
        }

        fn AllocateData(&mut self, size: u32) -> Result<(), SnapshotError> {
            if self.owns_data_ {
                return Err(SnapshotError::InvalidData);
            }
            // Allocate a buffer of size bytes
            let buffer = unsafe {
                let layout = std::alloc::Layout::from_size_align(size as usize, 1).unwrap();
                std::alloc::alloc(layout)
            };
            if buffer.is_null() {
                return Err(SnapshotError::AllocationError);
            }
            self.data_ = buffer;
            self.size_ = size;
            self.owns_data_ = true;
            Ok(())
        }

        fn SetMagicNumber(&mut self) {
            self.SetHeaderValue(Self::kMagicNumberOffset, Self::kMagicNumber);
        }
    }

    impl Drop for SerializedData {
        fn drop(&mut self) {
            if self.owns_data_ {
                unsafe {
                    let layout = std::alloc::Layout::from_size_align(self.size_ as usize, 1).unwrap();
                    std::alloc::dealloc(self.data_, layout);
                }
            }
        }
    }

    impl SerializedData {
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
    }

    pub struct SnapshotData {
        serialized_data: SerializedData,
    }

    impl SnapshotData {
        pub fn new(serializer: &Serializer) -> Self {
            let mut snapshot_data = SnapshotData {
                serialized_data: SerializedData {
                    data_: null_mut(),
                    size_: 0,
                    owns_data_: false,
                }
            };
            let no_gc = DisallowGarbageCollection {};
            let payload = serializer.Payload();

            let size = (Self::kHeaderSize + payload.len() as u32) as u32;

            snapshot_data.serialized_data.AllocateData(size).unwrap();

            unsafe {
                std::ptr::write_bytes(snapshot_data.serialized_data.data_, 0, Self::kHeaderSize as usize);
            }

            snapshot_data.serialized_data.SetMagicNumber();
            snapshot_data.serialized_data.SetHeaderValue(
                Self::kPayloadLengthOffset,
                payload.len() as u32,
            );

            unsafe {
                let dest = snapshot_data.serialized_data.data_.add(Self::kHeaderSize as usize);
                let src = payload.as_ptr();
                memcopy(dest, src, payload.len());
            }

            snapshot_data
        }

        pub fn from_snapshot(snapshot: base::Vector<u8>) -> Self {
            SnapshotData {
                serialized_data: SerializedData::new(
                    snapshot.as_ptr() as *mut u8,
                    snapshot.len() as i32,
                ),
            }
        }

        pub fn Payload(&self) -> base::Vector<u8> {
            let payload = unsafe {
                self.serialized_data.data_.add(Self::kHeaderSize as usize)
            };
            let length = self.serialized_data.GetHeaderValue(Self::kPayloadLengthOffset);
            assert_eq!(
                self.serialized_data.data_ as usize + self.serialized_data.size_ as usize,
                payload as usize + length as usize
            );
            unsafe {
                base::Vector::from_raw_parts(payload, length as usize, length as usize)
            }
        }

        pub fn RawData(&self) -> base::Vector<u8> {
            unsafe {
                base::Vector::from_raw_parts(
                    self.serialized_data.data_,
                    self.serialized_data.size_ as usize,
                    self.serialized_data.size_ as usize,
                )
            }
        }

        fn Resize(&mut self, size: u32) {
            self.serialized_data.size_ = size;
        }

        pub const kPayloadLengthOffset: u32 =
            SerializedData::kMagicNumberOffset + Self::kUInt32Size;
        pub const kHeaderSize: u32 = Self::kPayloadLengthOffset + Self::kUInt32Size;
        pub const kUInt32Size: u32 = size_of::<u32>() as u32;
    }

    impl SnapshotData {
        pub fn default() -> Self {
            SnapshotData {
                serialized_data: SerializedData::default()
            }
        }
    }
    pub struct Serializer {
        payload: Vec<u8>,
    }

    impl Serializer {
        pub fn Payload(&self) -> &Vec<u8> {
            &self.payload
        }
    }

    impl Serializer {
        pub fn new(payload: Vec<u8>) -> Self {
            Serializer { payload }
        }
    }

    pub struct DisallowGarbageCollection {}
}
