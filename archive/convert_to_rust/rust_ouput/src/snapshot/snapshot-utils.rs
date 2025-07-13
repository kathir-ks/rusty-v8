// Converted from V8 C++ source files:
// Header: snapshot-utils.h
// Implementation: snapshot-utils.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct Vector<T> {
        data: Vec<T>,
    }

    impl<T> Vector<T> {
        pub fn new(data: Vec<T>) -> Self {
            Vector { data }
        }

        pub fn begin(&self) -> *const T {
            self.data.as_ptr()
        }

        pub fn length(&self) -> usize {
            self.data.len()
        }

        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.data.iter()
        }
    }

    impl<'a, T> IntoIterator for &'a Vector<T> {
        type Item = &'a T;
        type IntoIter = std::slice::Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.iter()
        }
    }
}

pub mod internal {
    use crate::base::Vector;

    #[cfg(not(feature = "v8_use_zlib"))]
    pub fn checksum(payload: Vector<u8>) -> u32 {
        let mut sum1: u32 = 0;
        let mut sum2: u32 = 0;

        for &data in payload.iter() {
            sum1 = (sum1 + data as u32) % 65535;
            sum2 = (sum2 + sum1) % 65535;
        }

        (sum2 << 16) | sum1
    }

    #[cfg(feature = "v8_use_zlib")]
    pub fn checksum(payload: Vector<u8>) -> u32 {
        use flate2::Crc;
        use std::io::Write;

        let mut crc = Crc::new();
        crc.write_all(&payload.data).unwrap();
        crc.sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::base::Vector;

        #[test]
        fn test_checksum() {
            let data = vec![1, 2, 3, 4, 5];
            let vector = Vector::new(data);
            let checksum_value = checksum(vector);
            assert_ne!(checksum_value, 0);
        }
    }
}
