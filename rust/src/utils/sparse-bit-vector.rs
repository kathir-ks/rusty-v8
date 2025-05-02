// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::mem;
use std::ops::{BitOr, BitOrAssign};
use std::ptr;
use std::slice;

const K_BITS_PER_BYTE: usize = 8;

/// A sparse bit vector implementation optimized for small sizes.
/// For up to {k_num_bits_per_segment} bits, no additional storage is needed, and
/// accesses should be nearly as fast as {BitVector}.
pub struct SparseBitVector {
    first_segment: Segment,
    zone: Box<Zone>,
}

impl SparseBitVector {
    const K_NUM_WORDS_PER_SEGMENT: usize = 6;
    const K_BITS_PER_WORD: usize = K_BITS_PER_BYTE * mem::size_of::<usize>();
    const K_NUM_BITS_PER_SEGMENT: usize =
        Self::K_BITS_PER_WORD * Self::K_NUM_WORDS_PER_SEGMENT;

    /// An iterator to iterate all set bits.
    pub struct Iterator<'a> {
        segment: Option<&'a Segment>,
        bit_in_segment: usize,
    }

    impl<'a> Iterator<'a> {
        fn new(segment: Option<&'a Segment>) -> Self {
            let mut iter = Iterator {
                segment,
                bit_in_segment: 0,
            };
            if iter.segment.is_some() {
                iter.advance_to_next_set_bit();
            }
            iter
        }

        fn advance_to_next_set_bit(&mut self) {
            if let Some(mut segment) = self.segment {
                loop {
                    let start_word = self.bit_in_segment / Self::K_BITS_PER_WORD;
                    for word in start_word..Self::K_NUM_WORDS_PER_SEGMENT {
                        let word_bits = segment.words[word];
                        if word_bits == 0 {
                            continue;
                        }

                        let bit_in_word = word_bits.trailing_zeros() as usize;
                        self.bit_in_segment = word * Self::K_BITS_PER_WORD + bit_in_word;
                        return;
                    }
                    if let Some(next_segment) = segment.next.as_ref() {
                        segment = next_segment;
                        self.segment = Some(segment);
                        self.bit_in_segment = 0;
                    } else {
                        self.segment = None;
                        self.bit_in_segment = 0;
                        return;
                    }
                }
            }
        }
    }

    impl<'a> std::iter::Iterator for Iterator<'a> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let current_segment = self.segment;
            if current_segment.is_none() {
                return None;
            }

            let segment = current_segment.unwrap();
            let offset = segment.offset + self.bit_in_segment;
            if !Self::contains(Some(segment), offset) {
                return None;
            }

            let result = offset;
            self.advance_to_next_set_bit();
            Some(result)
        }
    }

    pub fn new(zone: Box<Zone>) -> Self {
        SparseBitVector {
            first_segment: Segment::new(),
            zone,
        }
    }

    #[inline]
    pub fn contains(&self, i: usize) -> bool {
        if i >= Self::K_NUM_BITS_PER_SEGMENT {
            let mut segment = self.first_segment.next.as_ref();
            while let Some(s) = segment {
                if s.offset <= i - Self::K_NUM_BITS_PER_SEGMENT {
                    segment = s.next.as_ref();
                    if segment.is_none() {
                        return false;
                    }
                } else {
                    return s.offset <= i && Self::contains(Some(s), i);
                }
            }
            false
        } else {
            Self::contains(Some(&self.first_segment), i)
        }
    }

    #[inline]
    pub fn add(&mut self, i: usize) {
        if i >= Self::K_NUM_BITS_PER_SEGMENT {
            let mut last: Option<&mut Segment> = None;
            let mut segment = self.first_segment.next.as_mut();
            while let Some(s) = segment {
                last = Some(s);
                if s.offset <= i - Self::K_NUM_BITS_PER_SEGMENT {
                    segment = s.next.as_mut();
                    if segment.is_none() {
                        if let Some(l) = last {
                           self.insert_bit_after(l, i);
                        } else {
                            self.insert_bit_after(&mut self.first_segment, i);
                        }
                         return;
                    }
                } else {
                    if s.offset > i {
                        if let Some(l) = last {
                            self.insert_bit_after(l, i);
                        } else {
                            self.insert_bit_after(&mut self.first_segment, i);
                        }
                        return;
                    } else{
                        Self::set(Some(s), i);
                         return;
                    }
                }
            }
            if let Some(l) = last {
               self.insert_bit_after(l, i);
            } else {
                self.insert_bit_after(&mut self.first_segment, i);
            }
             return;

        } else {
            Self::set(Some(&mut self.first_segment), i);
        }
    }

    #[inline]
    pub fn remove(&mut self, i: usize) {
        if i >= Self::K_NUM_BITS_PER_SEGMENT {
            let mut segment = self.first_segment.next.as_mut();
            while let Some(s) = segment {
                if s.offset <= i - Self::K_NUM_BITS_PER_SEGMENT {
                    segment = s.next.as_mut();
                    if segment.is_none() {
                        return;
                    }
                } else {
                    if s.offset > i {
                        return;
                    }
                    Self::unset(Some(s), i);
                    return;
                }
            }
            return;
        } else {
            Self::unset(Some(&mut self.first_segment), i);
        }
    }

    pub fn union(&mut self, other: &SparseBitVector) {
        let mut last: Option<&mut Segment> = None;
        let mut segment = Some(&mut self.first_segment);

        let mut other_segment = other.first_segment.next.as_ref();

        while let Some(other_seg) = other_segment {
            while let Some(seg) = segment {
                if seg.offset < other_seg.offset {
                    last = Some(seg);
                    segment = seg.next.as_mut();
                } else {
                    break;
                }
            }

            if let Some(seg) = segment {
                if seg.offset == other_seg.offset {
                    for (i, &word) in other_seg.words.iter().enumerate() {
                        seg.words[i].bitor_assign(word);
                    }
                } else if !other_seg.is_empty() {
                    if let Some(last_seg) = last {
                        if last_seg.offset < other_seg.offset{
                            let mut new_segment = self.zone.allocate_segment();
                            new_segment.offset = other_seg.offset;
                            new_segment.words.copy_from_slice(&other_seg.words);
                            self.insert_segment_after(last_seg, new_segment);
                            last = Some(new_segment);
                        } else {
                             panic!("last segment's offset should be less than the other_segment's offset");
                        }

                    } else {
                         let mut new_segment = self.zone.allocate_segment();
                            new_segment.offset = other_seg.offset;
                            new_segment.words.copy_from_slice(&other_seg.words);
                           self.insert_segment_after(&mut self.first_segment, new_segment);
                            last = Some(new_segment);
                    }

                }
                other_segment = other_seg.next.as_ref();
            } else {
                  if !other_seg.is_empty() {
                     if let Some(last_seg) = last {
                        if last_seg.offset < other_seg.offset{
                            let mut new_segment = self.zone.allocate_segment();
                            new_segment.offset = other_seg.offset;
                            new_segment.words.copy_from_slice(&other_seg.words);
                            self.insert_segment_after(last_seg, new_segment);
                            last = Some(new_segment);
                        } else {
                             panic!("last segment's offset should be less than the other_segment's offset");
                        }
                     } else {
                         let mut new_segment = self.zone.allocate_segment();
                            new_segment.offset = other_seg.offset;
                            new_segment.words.copy_from_slice(&other_seg.words);
                           self.insert_segment_after(&mut self.first_segment, new_segment);
                            last = Some(new_segment);
                     }
                  }
                 other_segment = other_seg.next.as_ref();
            }
        }
    }

    pub fn begin(&self) -> Iterator {
        Iterator::new(self.first_segment.next.as_ref())
    }

    // The end iterator does not point to any actual memory location
    pub fn end(&self) -> Iterator {
        Iterator::new(None)
    }

    fn get_word_and_bit_in_word(segment: Option<&Segment>, i: usize) -> (usize, usize) {
        let segment_val = segment.unwrap();
        let bit_in_segment = i - segment_val.offset;
        (
            bit_in_segment / Self::K_BITS_PER_WORD,
            bit_in_segment % Self::K_BITS_PER_WORD,
        )
    }

    #[inline(never)]
    fn insert_bit_after(&mut self, segment: &mut Segment, i: usize) {
        let mut new_segment = self.zone.allocate_segment();
        new_segment.offset = i / Self::K_NUM_BITS_PER_SEGMENT * Self::K_NUM_BITS_PER_SEGMENT;
        Self::set(Some(new_segment), i);
        self.insert_segment_after(segment, new_segment);
        //assert!(self.contains(i)); // Assuming contains is implemented
    }

    #[inline(never)]
    fn insert_segment_after(&mut self, segment: &mut Segment, new_segment: &mut Segment) {
        assert!(segment.offset < new_segment.offset);

        let raw_new_segment = new_segment as *mut Segment;

        let next = segment.next.take();
        segment.next = Some(new_segment);

        let new_segment_ref = unsafe { &mut *raw_new_segment };
        new_segment_ref.next = next;

       // assert!(self.check_consistency(segment)); // Assuming check_consistency is implemented
       // assert!(self.check_consistency(new_segment)); // Assuming check_consistency is implemented
    }

    fn contains(segment: Option<&Segment>, i: usize) -> bool {
        if let Some(seg) = segment {
            let (word, bit) = Self::get_word_and_bit_in_word(Some(seg), i);
            (seg.words[word] >> bit) & 1 != 0
        } else {
            false
        }
    }

    fn set(segment: Option<&mut Segment>, i: usize) -> bool {
        if let Some(seg) = segment {
            let (word, bit) = Self::get_word_and_bit_in_word(Some(seg), i);
            seg.words[word] |= 1 << bit;
            true
        } else {
            false
        }
    }

    fn unset(segment: Option<&mut Segment>, i: usize) -> bool {
        if let Some(seg) = segment {
            let (word, bit) = Self::get_word_and_bit_in_word(Some(seg), i);
            seg.words[word] &= !(1 << bit);
            true
        } else {
            false
        }
    }

    fn check_consistency(&self, segment: &Segment) -> bool {
        if (segment.offset % Self::K_NUM_BITS_PER_SEGMENT) != 0 {
            return false;
        }

        if let Some(next_segment) = &segment.next {
            if next_segment.offset <= segment.offset {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
struct Segment {
    // Offset of the first bit in this segment.
    offset: usize,
    // {words} covers bits [{offset}, {offset + k_num_bits_per_segment}).
    words: [usize; SparseBitVector::K_NUM_WORDS_PER_SEGMENT],
    // The next segment (with strict larger offset), or {None}.
    next: Option<Box<Segment>>,
}

impl Segment {
    fn new() -> Self {
        Segment {
            offset: 0,
            words: [0; SparseBitVector::K_NUM_WORDS_PER_SEGMENT],
            next: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.words.iter().all(|&segment| segment == 0)
    }
}

struct Zone {
    segments: Vec<Box<Segment>>,
}

impl Zone {
    fn new() -> Self {
        Zone {
            segments: Vec::new(),
        }
    }

    fn allocate_segment(&mut self) -> &mut Segment {
        let segment = Box::new(Segment::new());
        self.segments.push(segment);
        self.segments.last_mut().unwrap()
    }
}