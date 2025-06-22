// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod ring_buffer {

/// A ring buffer implementation.
#[derive(Default)]
pub struct RingBuffer<T, const SIZE: usize = 10> {
    elements: [T; SIZE],
    pos: u8,
    is_full: bool,
}

impl<T, const SIZE: usize> RingBuffer<T, SIZE>
where
    T: Copy + Default,
{
    /// The size of the ring buffer.
    pub const K_SIZE: usize = SIZE;

    /// Creates a new ring buffer.
    pub fn new() -> Self {
        RingBuffer {
            elements: [T::default(); SIZE],
            pos: 0,
            is_full: false,
        }
    }

    /// Pushes a value into the ring buffer.
    pub fn push(&mut self, value: T) {
        let pos = self.pos as usize;
        self.elements[pos] = value;
        self.pos += 1;
        if (self.pos as usize) == SIZE {
            self.pos = 0;
            self.is_full = true;
        }
    }

    /// Returns the current size of the ring buffer.
    pub fn size(&self) -> u8 {
        if self.is_full {
            SIZE as u8
        } else {
            self.pos
        }
    }

    /// Returns true if the ring buffer is empty.
    pub fn empty(&self) -> bool {
        self.size() == 0
    }

    /// Clears the ring buffer.
    pub fn clear(&mut self) {
        self.pos = 0;
        self.is_full = false;
    }

    /// Reduces the elements in the ring buffer using a callback function.
    pub fn reduce<F>(&self, callback: F, initial: T) -> T
    where
        F: Fn(T, T) -> T,
    {
        let mut result = initial;
        let pos = self.pos;

        for i in (0..pos).rev() {
            result = callback(result, self.elements[i as usize]);
        }

        if !self.is_full {
            return result;
        }

        for i in (pos..(SIZE as u8)).rev() {
            result = callback(result, self.elements[i as usize]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer() {
        let mut rb: RingBuffer<i32, 5> = RingBuffer::new();
        assert_eq!(rb.size(), 0);
        assert!(rb.empty());

        rb.push(1);
        rb.push(2);
        rb.push(3);
        assert_eq!(rb.size(), 3);
        assert!(!rb.empty());

        rb.push(4);
        rb.push(5);
        assert_eq!(rb.size(), 5);

        rb.push(6);
        assert_eq!(rb.size(), 5);

        let sum = rb.reduce(|acc, x| acc + x, 0);
        assert_eq!(sum, 20);

        rb.clear();
        assert_eq!(rb.size(), 0);
        assert!(rb.empty());
    }
}

} // mod ring_buffer