// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::VecDeque;
use std::sync::Mutex;

/// A thread-safe data structure that generates heuristic starting points in a
/// range to process items in parallel.
pub struct IndexGenerator {
    lock: Mutex<IndexGeneratorInternal>,
}

struct IndexGeneratorInternal {
    first_use: bool,
    // Pending [start, end) ranges to split and hand out indices from.
    ranges_to_split: VecDeque<(usize, usize)>,
}

impl IndexGenerator {
    pub fn new(size: usize) -> Self {
        let mut ranges = VecDeque::new();
        ranges.push_back((0, size));
        IndexGenerator {
            lock: Mutex::new(IndexGeneratorInternal {
                first_use: true,
                ranges_to_split: ranges,
            }),
        }
    }

    pub fn get_next(&self) -> Option<usize> {
        let mut internal = self.lock.lock().unwrap();
        if let Some((start, end)) = internal.ranges_to_split.pop_front() {
            if start < end {
                let mid = start + (end - start) / 2;
                if (mid - start) > 1{
                  internal.ranges_to_split.push_back((start,mid));
                }

                if (end - mid) > 1 {
                  internal.ranges_to_split.push_back((mid,end));
                }

                if (mid - start) <= 1 && start < mid {
                   internal.ranges_to_split.push_back((start,mid));
                }
               
                if (end - mid) <= 1 && mid < end {
                   internal.ranges_to_split.push_back((mid,end));
                }

                return Some(mid);
            }
        }
        None
    }
}