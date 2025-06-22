// src/snapshot/snapshot-source-sink.rs

//use std::vec::Vec;

/// A sink for snapshot data, implemented as a byte vector.
pub struct SnapshotByteSink {
    data_: Vec<u8>,
}

impl SnapshotByteSink {
    /// Creates a new empty SnapshotByteSink.
    pub fn new() -> Self {
        SnapshotByteSink { data_: Vec::new() }
    }

    /// Puts a specified number of bytes with a given value into the sink.
    ///
    /// # Arguments
    ///
    /// * `number_of_bytes` - The number of bytes to put.
    /// * `v` - The value of the byte to put.
    /// * `description` - A description of the data being put (for debugging).
    pub fn put_n(&mut self, number_of_bytes: usize, v: u8, description: &str) {
        self.data_.extend(std::iter::repeat(v).take(number_of_bytes));
    }

    /// Puts a 30-bit unsigned integer into the sink, encoded as a variable-length byte sequence.
    ///
    /// # Arguments
    ///
    /// * `integer` - The integer to put (must be less than 2^30).
    /// * `description` - A description of the data being put (for debugging).
    pub fn put_uint30(&mut self, integer: u32, description: &str) {
        assert!(integer < (1 << 30));
        let mut integer = integer << 2;
        let mut bytes = 1;
        if integer > 0xFF { bytes = 2; }
        if integer > 0xFFFF { bytes = 3; }
        if integer > 0xFFFFFF { bytes = 4; }
        integer |= (bytes - 1) as u32;

        self.put(integer as u8 & 0xFF, "IntPart1");
        if bytes > 1 { self.put((integer >> 8) as u8 & 0xFF, "IntPart2"); }
        if bytes > 2 { self.put((integer >> 16) as u8 & 0xFF, "IntPart3"); }
        if bytes > 3 { self.put((integer >> 24) as u8 & 0xFF, "IntPart4"); }
    }

    /// Puts raw byte data into the sink.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice containing the byte data to put.
    /// * `number_of_bytes` - The number of bytes to put.
    /// * `description` - A description of the data being put (for debugging).
    pub fn put_raw(&mut self, data: &[u8], number_of_bytes: usize, description: &str) {
        // #[cfg(feature = "memory_sanitizer")]
        // unsafe {
        //     __msan_check_mem_is_initialized(data.as_ptr() as *const c_void, number_of_bytes);
        // }

        self.data_.extend_from_slice(&data[..number_of_bytes]);
    }

    /// Appends the data from another SnapshotByteSink to this sink.
    ///
    /// # Arguments
    ///
    /// * `other` - The other SnapshotByteSink to append.
    pub fn append(&mut self, other: &SnapshotByteSink) {
        self.data_.extend_from_slice(&other.data_);
    }

    /// Puts a single byte into the sink.
    ///
    /// # Arguments
    ///
    /// * `v` - The byte to put.
    /// * `description` - A description of the data being put (for debugging).
    pub fn put(&mut self, v: u8, description: &str) {
        self.data_.push(v);
    }

    /// Returns a reference to the underlying byte vector.
    pub fn data(&self) -> &Vec<u8> {
        &self.data_
    }

    /// Returns a mutable reference to the underlying byte vector.
    pub fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data_
    }

    /// Consumes the `SnapshotByteSink` and returns the underlying `Vec<u8>`.
    pub fn into_data(self) -> Vec<u8> {
        self.data_
    }
}

/// A source for snapshot data, implemented as a byte slice.
pub struct SnapshotByteSource<'a> {
    data_: &'a [u8],
    position_: usize,
    length_: usize,
}

impl<'a> SnapshotByteSource<'a> {
    /// Creates a new SnapshotByteSource from a byte slice.
    ///
    /// # Arguments
    ///
    /// * `data` - The byte slice to use as the source.
    pub fn new(data: &'a [u8]) -> Self {
        let length_ = data.len();
        SnapshotByteSource {
            data_: data,
            position_: 0,
            length_: length_,
        }
    }

    /// Gets a 30-bit unsigned integer from the source.
    ///
    /// Returns:
    ///
    /// The integer read from the source.
    pub fn get_uint30(&mut self) -> u32 {
        let mut integer = self.data_[self.position_] as u32;
        self.position_ += 1;

        let bytes = (integer & 3) + 1;
        integer >>= 2;

        if bytes > 1 {
            integer |= (self.data_[self.position_] as u32) << 8;
            self.position_ += 1;
        }
        if bytes > 2 {
            integer |= (self.data_[self.position_] as u32) << 16;
            self.position_ += 1;
        }
        if bytes > 3 {
            integer |= (self.data_[self.position_] as u32) << 24;
            self.position_ += 1;
        }
        integer
    }

    /// Gets a blob (a sequence of bytes) from the source.
    ///
    /// Returns:
    ///
    /// The size of the blob. The data itself is returned via the `data` out parameter.
    pub fn get_blob(&mut self) -> (&'a [u8], usize) {
        let size = self.get_uint30() as usize;
        assert!(self.position_ + size <= self.length_);
        let data = &self.data_[self.position_..self.position_ + size];
        self.advance(size);
        (data, size)
    }

    /// Advances the current position by the specified number of bytes.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of bytes to advance.
    fn advance(&mut self, size: usize) {
        self.position_ += size;
    }
}