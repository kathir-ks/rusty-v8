// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::BTreeMap;
use std::mem;
use std::ptr;

mod base {
    pub mod logging {
        #[macro_export]
        macro_rules! DCHECK {
            ($condition:expr) => {
                if !$condition {
                    panic!("DCHECK failed: {}", stringify!($condition));
                }
            };
            ($condition:expr, $($arg:tt)*) => {
                if !$condition {
                    panic!("DCHECK failed: {}: {}", stringify!($condition), format_args!($($arg)*));
                }
            };
        }

        #[macro_export]
        macro_rules! CHECK_WITH_MSG {
            ($condition:expr, $message:expr) => {
                if $condition {
                    panic!("{}", $message);
                }
            };
        }
    }
}

mod heap {
    pub mod memory_chunk_layout {
        // Placeholder module. The actual layout details are not relevant for
        // the functionality of this code.
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SlotType {
        kCleared,
        kOther, // Placeholder
    }

    mod type_field {
        use super::SlotType;

        pub fn encode(slot_type: SlotType) -> u32 {
            match slot_type {
                SlotType::kCleared => 0,
                SlotType::kOther => 1, // Placeholder
            }
        }

        pub fn decode(encoded: u32) -> SlotType {
            match encoded {
                0 => SlotType::kCleared,
                _ => SlotType::kOther, // Placeholder
            }
        }
    }

    mod offset_field {
        pub fn encode(offset: u32) -> u32 {
            offset
        }

        pub fn decode(encoded: u32) -> u32 {
            encoded
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct TypedSlot {
        pub type_and_offset: u32,
    }

    pub fn cleared_typed_slot() -> TypedSlot {
        TypedSlot {
            type_and_offset: type_field::encode(SlotType::kCleared),
        }
    }

    pub struct TypedSlots {
        head_: Option<Box<Chunk>>,
        tail_: Option<*mut Chunk>,
    }

    struct Chunk {
        next: Option<Box<Chunk>>,
        buffer: Vec<TypedSlot>,
    }

    const K_INITIAL_BUFFER_SIZE: usize = 8; // Example size. Adjust as needed.

    impl TypedSlots {
        pub fn new() -> Self {
            TypedSlots {
                head_: None,
                tail_: None,
            }
        }

        pub fn insert(&mut self, type_: SlotType, offset: u32) {
            let slot = TypedSlot {
                type_and_offset: type_field::encode(type_) | offset_field::encode(offset),
            };
            let chunk = self.ensure_chunk();
            base::logging::DCHECK!(chunk.buffer.len() < chunk.buffer.capacity());
            chunk.buffer.push(slot);
        }

        pub fn merge(&mut self, other: &mut TypedSlots) {
            if other.head_.is_none() {
                return;
            }

            if self.head_.is_none() {
                self.head_ = other.head_.take();
                self.tail_ = other.tail_.take();
            } else {
                if let Some(tail) = self.tail_ {
                    unsafe {
                        (*tail).next = other.head_.take();
                    }
                    self.tail_ = other.tail_.take();
                }
            }

            other.head_ = None;
            other.tail_ = None;
        }

        fn ensure_chunk(&mut self) -> &mut Chunk {
            if self.head_.is_none() {
                let new_chunk = self.new_chunk(None, K_INITIAL_BUFFER_SIZE);
                let chunk_ptr = Box::into_raw(new_chunk);
                self.head_ = Some(unsafe { Box::from_raw(chunk_ptr) });
                self.tail_ = Some(chunk_ptr);

                let head = self.head_.as_mut().unwrap();
                return &mut head;
            }

            let head = self.head_.as_mut().unwrap();
            if head.buffer.len() == head.buffer.capacity() {
                let next_capacity = self.next_capacity(head.buffer.capacity());
                let new_chunk = self.new_chunk(self.head_.take(), next_capacity);

                let chunk_ptr = Box::into_raw(new_chunk);
                self.head_ = Some(unsafe { Box::from_raw(chunk_ptr) });

                let head = self.head_.as_mut().unwrap();
                return &mut head;
            }

            self.head_.as_mut().unwrap()
        }

        fn new_chunk(&self, next: Option<Box<Chunk>>, capacity: usize) -> Box<Chunk> {
            Box::new(Chunk {
                next,
                buffer: Vec::with_capacity(capacity),
            })
        }

        fn next_capacity(&self, current_capacity: usize) -> usize {
            current_capacity * 2 // Example growth factor. Adjust as needed.
        }
    }

    impl Drop for TypedSlots {
        fn drop(&mut self) {
            let mut chunk = self.head_.take();
            while let Some(mut c) = chunk {
                chunk = c.next.take();
            }
            self.tail_ = None;
        }
    }

    pub type FreeRangesMap = BTreeMap<u32, u32>;

    pub struct TypedSlotSet {
        head: Option<Box<Chunk>>,
    }

    impl TypedSlotSet {
        pub fn new() -> Self {
            TypedSlotSet { head: None }
        }

        pub fn clear_invalid_slots(&mut self, invalid_ranges: &FreeRangesMap) {
            self.iterate_slots_in_ranges(
                |slot: &mut TypedSlot| *slot = cleared_typed_slot(),
                invalid_ranges,
            );
        }

        pub fn assert_no_invalid_slots(&self, invalid_ranges: &FreeRangesMap) {
            self.iterate_slots_in_ranges(
                |_slot: &mut TypedSlot| {
                    base::logging::CHECK_WITH_MSG!(true, "No slot in ranges expected.");
                },
                invalid_ranges,
            );
        }

        fn iterate_slots_in_ranges<Callback>(&self, callback: Callback, ranges: &FreeRangesMap)
        where
            Callback: Fn(&mut TypedSlot),
        {
            if ranges.is_empty() {
                return;
            }

            let mut chunk = self.load_head();
            while let Some(c) = chunk {
                for slot in &mut c.buffer {
                    let type_ = type_field::decode(slot.type_and_offset);
                    if type_ == SlotType::kCleared {
                        continue;
                    }
                    let offset = offset_field::decode(slot.type_and_offset);
                    if let Some((&range_start, &range_end)) = ranges.range(..=offset).last() {
                        if range_start <= offset && range_end > offset {
                            callback(slot);
                        }
                    }
                }
                chunk = self.load_next(c);
            }
        }

        fn load_head(&self) -> Option<&mut Chunk> {
            match &self.head {
                Some(head) => Some(unsafe { &mut *(head.as_ref() as *const Chunk as *mut Chunk) }),
                None => None,
            }
        }

        fn load_next<'a>(&self, chunk: &'a Chunk) -> Option<&'a mut Chunk> {
            match &chunk.next {
                Some(next) => Some(unsafe { &mut *(next.as_ref() as *const Chunk as *mut Chunk) }),
                None => None,
            }
        }

        fn set_head(&mut self, new_head: Option<Box<Chunk>>) {
            self.head = new_head;
        }
    }
}