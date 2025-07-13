// Converted from V8 C++ source files:
// Header: snapshot-compression.h
// Implementation: snapshot-compression.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct Vector<T> {
        data: Vec<T>,
    }

    impl<T> Vector<T> {
        pub fn new() -> Self {
            Vector { data: Vec::new() }
        }

        pub fn with_capacity(capacity: usize) -> Self {
            Vector {
                data: Vec::with_capacity(capacity),
            }
        }

        pub fn push(&mut self, value: T) {
            self.data.push(value);
        }

        pub fn resize(&mut self, new_size: usize)
        where
            T: Default + Clone,
        {
            self.data.resize(new_size, T::default());
        }

        pub fn size(&self) -> usize {
            self.data.len()
        }

        pub fn capacity(&self) -> usize {
            self.data.capacity()
        }

        pub fn reserve(&mut self, additional: usize) {
            self.data.reserve(additional);
        }

        pub fn data(&self) -> &Vec<T> {
            &self.data
        }

        pub fn data_mut(&mut self) -> &mut Vec<T> {
            &mut self.data
        }

        pub fn begin(&self) -> *const T {
            self.data.as_ptr()
        }

        pub fn begin_mut(&mut self) -> *mut T {
            self.data.as_mut_ptr()
        }

        pub fn iter(&self) -> std::slice::Iter<T> {
            self.data.iter()
        }
    }

    impl<T: Copy> Vector<T> {
        pub fn from_slice(slice: &[T]) -> Self {
            Vector {
                data: slice.to_vec(),
            }
        }
    }
}

pub mod snapshot {
    use crate::base::Vector;
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct SnapshotData {
        data: Vector<u8>,
    }

    impl SnapshotData {
        pub fn new() -> Self {
            SnapshotData {
                data: Vector::new(),
            }
        }

        pub fn allocate_data(&mut self, size: u32) {
            self.data.resize(size as usize, 0);
        }

        pub fn raw_data(&self) -> &Vector<u8> {
            &self.data
        }

        pub fn raw_data_mut(&mut self) -> &mut Vector<u8> {
            &mut self.data
        }

        pub fn resize(&mut self, new_size: u32) {
            self.data.resize(new_size as usize, 0);
        }

        pub fn from_vec(vec: Vec<u8>) -> Self {
            let mut snapshot_data = SnapshotData::new();
            snapshot_data.data = Vector { data: vec };
            snapshot_data
        }

        pub fn to_vec(&self) -> Vec<u8> {
            self.data.data().clone()
        }
    }

    impl fmt::Display for SnapshotData {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "SnapshotData with size: {}", self.data.size())
        }
    }
}

pub mod utils {
    pub fn memcopy(dst: *mut u8, src: *const u8, size: usize) {
        unsafe {
            std::ptr::copy_nonoverlapping(src, dst, size);
        }
    }
}

pub mod flags {
    pub static mut profile_deserialization: bool = false;
}

pub mod base_platform {
    use std::time::{Duration, Instant};

    pub struct ElapsedTimer {
        start_time: Option<Instant>,
    }

    impl ElapsedTimer {
        pub fn new() -> Self {
            ElapsedTimer { start_time: None }
        }

        pub fn start(&mut self) {
            self.start_time = Some(Instant::now());
        }

        pub fn elapsed(&self) -> Duration {
            match self.start_time {
                Some(start) => start.elapsed(),
                None => Duration::from_secs(0),
            }
        }
    }
}

pub mod compression_utils_portable {
    use std::mem::MaybeUninit;

    const Z_OK: i32 = 0;
    const Z_DEFAULT_COMPRESSION: i32 = 6;
    const ZRAW: i32 = -15; // Raw deflate compression

    pub fn compress_bound(source_len: u64) -> u64 {
        source_len + (source_len / 1000) + 12
    }

    pub fn compress(
        dest: &mut [u8],
        dest_len: &mut u64,
        source: &[u8],
        source_len: u64,
        level: i32,
    ) -> i32 {
        let mut stream = zlib::deflate::DeflateStream::new(level as zlib::CompressionLevel);
        let mut input_offset = 0;
        let mut output_offset = 0;

        while input_offset < source_len as usize {
            let input_remaining = source_len as usize - input_offset;
            let output_remaining = dest.len() - output_offset;

            stream.process_block(
                &source[input_offset..],
                &mut dest[output_offset..],
                zlib::Flush::None,
            );

            input_offset += stream.total_in();
            output_offset += stream.total_out();

            if stream.needs_output() {
                break; // Output buffer is full
            }
        }

        // Finalize the stream
        stream.process_block(
            &[],
            &mut dest[output_offset..],
            zlib::Flush::Finish,
        );
        output_offset += stream.total_out();

        *dest_len = output_offset as u64;

        if stream.is_done() {
            Z_OK
        } else {
            -1 // Compression failed (replace with proper zlib error code if needed)
        }
    }

    pub fn uncompress(
        dest: &mut [u8],
        dest_len: &mut u64,
        source: &[u8],
        source_len: u64,
    ) -> i32 {
        let mut stream = zlib::inflate::InflateStream::new();
        let mut input_offset = 0;
        let mut output_offset = 0;

        while input_offset < source_len as usize {
            let input_remaining = source_len as usize - input_offset;
            let output_remaining = dest.len() - output_offset;

            stream.process_block(
                &source[input_offset..],
                &mut dest[output_offset..],
                zlib::Flush::None,
            );

            input_offset += stream.total_in();
            output_offset += stream.total_out();

            if stream.needs_output() {
                break; // Output buffer is full
            }
        }

        *dest_len = output_offset as u64;

        if stream.is_done() {
            Z_OK
        } else {
            -1 // Decompression failed (replace with proper zlib error code if needed)
        }
    }

    pub mod zlib_internal {
        use super::*;
        pub fn compress_helper(
            _strategy: i32,
            dest: *mut u8,
            dest_len: *mut u64,
            source: *const u8,
            source_len: u64,
            level: i32,
            _window_bits: *mut std::ffi::c_void,
            _mem_level: *mut std::ffi::c_void,
        ) -> i32 {
            let dest_slice = unsafe { std::slice::from_raw_parts_mut(dest, (*dest_len) as usize) };
            let source_slice = unsafe { std::slice::from_raw_parts(source, source_len as usize) };

            let mut actual_dest_len = *unsafe { &*dest_len };
            let result =
                compress(dest_slice, &mut actual_dest_len, source_slice, source_len, level);
            unsafe { *dest_len = actual_dest_len };
            result
        }

        pub fn uncompress_helper(
            _strategy: i32,
            dest: *mut u8,
            dest_len: *mut u64,
            source: *const u8,
            source_len: u64,
        ) -> i32 {
            let dest_slice = unsafe { std::slice::from_raw_parts_mut(dest, (*dest_len) as usize) };
            let source_slice = unsafe { std::slice::from_raw_parts(source, source_len as usize) };

            let mut actual_dest_len = *unsafe { &*dest_len };
            let result = uncompress(dest_slice, &mut actual_dest_len, source_slice, source_len);
            unsafe { *dest_len = actual_dest_len };
            result
        }
        pub const ZRAW: i32 = -15;
    }

    pub const Z_OK: i32 = 0;
    pub const Z_DEFAULT_COMPRESSION: i32 = 6;
}

pub mod internal {
    use crate::base::{Vector};
    use crate::base_platform::ElapsedTimer;
    use crate::compression_utils_portable;
    use crate::compression_utils_portable::zlib_internal;
    use crate::flags;
    use crate::snapshot::SnapshotData;
    use crate::utils::memcopy;
    use std::fmt;

    #[derive(Debug)]
    pub enum SnapshotError {
        CompressionError,
        DecompressionError,
        AllocationError,
        ResizingError,
        Other(String),
    }

    impl fmt::Display for SnapshotError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                SnapshotError::CompressionError => write!(f, "Snapshot compression failed"),
                SnapshotError::DecompressionError => write!(f, "Snapshot decompression failed"),
                SnapshotError::AllocationError => write!(f, "Snapshot allocation failed"),
                SnapshotError::ResizingError => write!(f, "Snapshot resizing failed"),
                SnapshotError::Other(msg) => write!(f, "Snapshot error: {}", msg),
            }
        }
    }

    impl std::error::Error for SnapshotError {}

    pub struct SnapshotCompression {}

    impl SnapshotCompression {
        pub fn compress(uncompressed_data: &SnapshotData) -> SnapshotData {
            let mut snapshot_data = SnapshotData::new();
            let mut timer = ElapsedTimer::new();

            unsafe {
                if flags::profile_deserialization {
                    timer.start();
                }
            }

            let input_size = uncompressed_data.raw_data().size() as u64;
            let payload_length = uncompressed_data.raw_data().size() as u32;
            let compressed_data_size = compression_utils_portable::compress_bound(input_size);

            snapshot_data.allocate_data(
                (std::mem::size_of::<u32>() as u32) + compressed_data_size as u32,
            );

            let compressed_data = snapshot_data.raw_data_mut().begin_mut();
            let payload_length_ptr = &payload_length as *const u32 as *const u8;

            unsafe {
                memcopy(
                    compressed_data,
                    payload_length_ptr,
                    std::mem::size_of::<u32>(),
                );
            }

            let mut actual_compressed_size = compressed_data_size;

            let result = unsafe {
                zlib_internal::compress_helper(
                    zlib_internal::ZRAW,
                    compressed_data.add(std::mem::size_of::<u32>()),
                    &mut actual_compressed_size,
                    uncompressed_data.raw_data().begin(),
                    input_size,
                    compression_utils_portable::Z_DEFAULT_COMPRESSION,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                )
            };

            if result != compression_utils_portable::Z_OK {
                eprintln!("Compression failed with code: {}", result);
            }

            snapshot_data.resize(
                actual_compressed_size as u32 + std::mem::size_of::<u32>() as u32,
            );
            let uncompressed_size =
                SnapshotCompression::get_uncompressed_size(snapshot_data.raw_data().begin());

            if payload_length != uncompressed_size {
                eprintln!("payload_length != uncompressed_size");
            }
            unsafe {
                if flags::profile_deserialization {
                    let ms = timer.elapsed().as_secs_f64() * 1000.0;
                    println!(
                        "[Compressing {} bytes took {:.3} ms]",
                        payload_length, ms
                    );
                }
            }

            snapshot_data
        }

        pub fn decompress(compressed_data: &Vector<u8>) -> SnapshotData {
            let mut snapshot_data = SnapshotData::new();
            let mut timer = ElapsedTimer::new();
            unsafe {
                if flags::profile_deserialization {
                    timer.start();
                }
            }

            let input_bytef = compressed_data.begin();

            let uncompressed_payload_length =
                SnapshotCompression::get_uncompressed_size(input_bytef);
            let mut input_bytef_offset = input_bytef as usize;
            input_bytef_offset += std::mem::size_of::<u32>();

            snapshot_data.allocate_data(uncompressed_payload_length);

            let mut uncompressed_size = uncompressed_payload_length as u64;

            let result = unsafe {
                zlib_internal::uncompress_helper(
                    zlib_internal::ZRAW,
                    snapshot_data.raw_data_mut().begin(),
                    &mut uncompressed_size,
                    input_bytef_offset as *const u8,
                    compressed_data.size() as u64 - std::mem::size_of::<u32>() as u64,
                )
            };

            if result != compression_utils_portable::Z_OK {
                eprintln!("Decompression failed with code: {}", result);
            }
            unsafe {
                if flags::profile_deserialization {
                    let ms = timer.elapsed().as_secs_f64() * 1000.0;
                    println!(
                        "[Decompressing {} bytes took {:.3} ms]",
                        uncompressed_payload_length, ms
                    );
                }
            }
            snapshot_data
        }

        fn get_uncompressed_size(compressed_data: *const u8) -> u32 {
            let mut size: u32 = 0;
            unsafe {
                memcopy(
                    &mut size as *mut u32 as *mut u8,
                    compressed_data,
                    std::mem::size_of::<u32>(),
                );
            }
            size
        }
    }
}
