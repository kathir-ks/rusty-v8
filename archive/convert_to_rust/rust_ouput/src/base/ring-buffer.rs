// Converted from V8 C++ source files:
// Header: ring-buffer.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

namespace!();
pub mod v8 {
pub mod base {
#[derive(Default, Copy, Clone)]
pub struct RingBuffer<T, const SIZE: usize = 10>
where
    T: Copy,
{
    elements: [T; SIZE],
    pos: u8,
    is_full: bool,
}
impl<T: Copy, const SIZE: usize> RingBuffer<T, SIZE> {
    pub const K_SIZE: u8 = SIZE as u8;
    pub const fn new() -> Self {
        RingBuffer {
            elements: [unsafe { std::mem::zeroed() }; SIZE],
            pos: 0,
            is_full: false,
        }
    }
    pub const fn push(&mut self, value: &T) {
        self.elements[self.pos as usize] = *value;
        self.pos += 1;
        if self.pos == Self::K_SIZE {
            self.pos = 0;
            self.is_full = true;
        }
    }
    pub const fn size(&self) -> u8 {
        if self.is_full {
            Self::K_SIZE
        } else {
            self.pos
        }
    }
    pub const fn empty(&self) -> bool {
        self.size() == 0
    }
    pub fn clear(&mut self) {
        self.pos = 0;
        self.is_full = false;
    }
    pub fn reduce<F>(&self, callback: F, initial: T) -> T
    where
        F: Fn(T, T) -> T,
    {
        let mut result = initial;
        let mut i = self.pos;
        while i > 0 {
            i -= 1;
            result = callback(result, self.elements[i as usize]);
        }
        if !self.is_full {
            return result;
        }
        let mut i = Self::K_SIZE;
        while i > self.pos {
            i -= 1;
            result = callback(result, self.elements[i as usize]);
        }
        result
    }
}
}
}
