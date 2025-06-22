// src/snapshot/snapshot_compression.rs

use std::mem::size_of;

use crate::base::platform::elapsed_timer::ElapsedTimer;
use crate::utils::memcopy::memcopy;
use crate::utils::utils::PrintF;
use crate::v8_flags;
use flate2::{Compression, Decompress, FlushDecompress, Status};
use flate2::Compress;

pub mod zlib_internal {
    pub const ZRAW: i32 = -15;
}

/// Gets the uncompressed size from the compressed data.
pub fn get_uncompressed_size(compressed_data: &[u8]) -> u32 {
    let mut size: u32 = 0;
    memcopy(&mut size, compressed_data, size_of::<u32>());
    size
}

/// Represents snapshot data.
#[derive(Debug, PartialEq)]
pub struct SnapshotData {
    data: Vec<u8>,
}

impl SnapshotData {
    /// Allocates memory for the snapshot data.
    pub fn allocate_data(&mut self, size: u32) {
        self.data.resize(size as usize, 0);
    }

    /// Gets a reference to the raw data.
    pub fn raw_data(&self) -> &[u8] {
        &self.data
    }

    /// Gets a mutable reference to the raw data.
    pub fn raw_data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Resizes the snapshot data.
    pub fn resize(&mut self, size: u32) {
        self.data.resize(size as usize, 0);
    }

    /// Creates a new `SnapshotData` with an empty vector
    pub fn new() -> Self {
        SnapshotData { data: Vec::new() }
    }
}

/// Provides methods for compressing and decompressing snapshot data.
pub struct SnapshotCompression {}

impl SnapshotCompression {
    /// Compresses the given snapshot data.
    pub fn compress(uncompressed_data: &SnapshotData) -> SnapshotData {
        let mut snapshot_data = SnapshotData::new();
        let mut timer = ElapsedTimer::new();
        if v8_flags::profile_deserialization() {
            timer.start();
        }

        let input_size = uncompressed_data.raw_data().len();
        let payload_length = input_size as u32;

        let mut compressor = Compress::new(Compression::default(), false);
        let max_compressed_size = compressor.total_out() + input_size as u64 + 6; // Add header size from compressBound

        snapshot_data.allocate_data((size_of::<u32>() as u32) + max_compressed_size as u32);
        
        let compressed_data_ptr = snapshot_data.raw_data_mut();

        // Manually store the uncompressed size.
        memcopy(compressed_data_ptr, &payload_length, size_of::<u32>());

        let result = compressor.compress(
            uncompressed_data.raw_data(), 
            &mut compressed_data_ptr[size_of::<u32>()..],
            FlushDecompress::Finish);

        let compressed_data_size = match result {
            Ok(Status::Ok) | Ok(Status::StreamEnd) => compressor.total_out() as usize,
            Ok(Status::BufError) => panic!("Buffer error during compression"),
            Ok(Status::NeedMoreInput) => panic!("Need more input during compression"),
            Err(e) => panic!("Compression failed: {:?}", e),
        };
        
        snapshot_data.resize(compressed_data_size as u32 + size_of::<u32>() as u32);
        
        assert_eq!(
            payload_length,
            get_uncompressed_size(snapshot_data.raw_data())
        );

        if v8_flags::profile_deserialization() {
            let ms = timer.elapsed().in_milliseconds_f();
            PrintF(format!("[Compressing {} bytes took {:.3} ms]\n", payload_length, ms));
        }
        snapshot_data
    }

    /// Decompresses the given compressed data.
    pub fn decompress(compressed_data: &[u8]) -> SnapshotData {
        let mut snapshot_data = SnapshotData::new();
        let mut timer = ElapsedTimer::new();
        if v8_flags::profile_deserialization() {
            timer.start();
        }

        let uncompressed_payload_length = get_uncompressed_size(compressed_data);
        let mut input_bytef = &compressed_data[size_of::<u32>()..];

        snapshot_data.allocate_data(uncompressed_payload_length);

        let mut decompressor = Decompress::new(false);

        let result = decompressor.decompress(
            input_bytef,
            snapshot_data.raw_data_mut(),
            FlushDecompress::Finish,
        );

        match result {
            Ok(Status::Ok) | Ok(Status::StreamEnd) => (),
            Ok(Status::BufError) => panic!("Buffer error during decompression"),
            Ok(Status::NeedMoreInput) => panic!("Need more input during decompression"),
            Err(e) => panic!("Decompression failed: {:?}", e),
        };

        if v8_flags::profile_deserialization() {
            let ms = timer.elapsed().in_milliseconds_f();
            PrintF(format!(
                "[Decompressing {} bytes took {:.3} ms]\n",
                uncompressed_payload_length, ms
            ));
        }
        snapshot_data
    }
}

// Mock implementations needed for compilation
pub mod base {
    pub mod platform {
        pub mod elapsed_timer {
            use std::time::{Instant, Duration};

            #[derive(Debug)]
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

                pub fn elapsed(&self) -> ElapsedTime {
                    match self.start_time {
                        Some(start) => {
                            let duration = Instant::now().duration_since(start);
                            ElapsedTime { duration }
                        },
                        None => ElapsedTime { duration: Duration::from_secs(0) },
                    }
                }
            }

            #[derive(Debug)]
            pub struct ElapsedTime {
                duration: Duration,
            }

            impl ElapsedTime {
                pub fn in_milliseconds_f(&self) -> f64 {
                    self.duration.as_secs_f64() * 1000.0
                }
            }
        }
    }
}

pub mod utils {
    pub mod memcopy {
        use std::mem::size_of;
        use std::slice;

        pub fn memcopy<T: Sized + Copy>(dest: &mut T, src: &[u8], size: usize) {
            let dest_slice = unsafe {
                slice::from_raw_parts_mut(dest as *mut T as *mut u8, size_of::<T>())
            };

            let copy_size = size.min(dest_slice.len().min(src.len()));
            dest_slice[..copy_size].copy_from_slice(&src[..copy_size]);
        }
    }

    pub mod utils {
        pub fn PrintF(message: String) {
            print!("{}", message);
        }
    }
}

mod v8_flags {
    pub fn profile_deserialization() -> bool {
        false
    }
}