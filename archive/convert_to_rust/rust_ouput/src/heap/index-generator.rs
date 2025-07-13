// Converted from V8 C++ source files:
// Header: index-generator.h
// Implementation: index-generator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::sync::Mutex;
use std::collections::VecDeque;

pub struct IndexGenerator {
    lock_: Mutex<()>,
    first_use_: bool,
    ranges_to_split_: Mutex<VecDeque<(usize, usize)>>,
}

impl IndexGenerator {
    pub fn new(size: usize) -> Self {
        let first_use_ = size > 0;
        let ranges_to_split_ = Mutex::new(VecDeque::new());

        if size > 0 {
            let mut guard = ranges_to_split_.lock().unwrap();
            guard.push_back((0, size));
        }

        IndexGenerator {
            lock_: Mutex::new(()),
            first_use_: first_use_,
            ranges_to_split_: Mutex::new(ranges_to_split_.into_inner().unwrap()),
        }
    }

    pub fn get_next(&self) -> Option<usize> {
        let _guard = self.lock_.lock().unwrap();

        if self.first_use_ {
            self.first_use_ = false;
            return Some(0);
        }

        let mut ranges_guard = self.ranges_to_split_.lock().unwrap();
        if ranges_guard.is_empty() {
            return None;
        }

        let range = ranges_guard.pop_front().unwrap();
        let size = range.1 - range.0;
        let mid = range.0 + size / 2;

        if mid - range.0 > 1 {
            ranges_guard.push_back((range.0, mid));
        }
        if range.1 - mid > 1 {
            ranges_guard.push_back((mid, range.1));
        }

        Some(mid)
    }
}
