// Copyright 2011 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/strings/string-search.h

mod string_search {
    use std::cmp;
    use std::mem;
    use std::slice;
    use std::os::raw::c_void;

    /// Cap on the maximal shift in the Boyer-Moore implementation.
    /// By setting a limit, we can fix the size of tables. For a needle longer than this limit,
    /// search will not be optimal, since we only build tables for a suffix
    /// of the string, but it is a safe approximation.
    const KB_MMAX_SHIFT: usize = 64; // TODO: Replace with Isolate::KB_MMAX_SHIFT if possible.

    /// Reduce alphabet to this size.
    /// One of the tables used by Boyer-Moore and Boyer-Moore-Horspool has size
    /// proportional to the input alphabet. We reduce the alphabet size by
    /// equating input characters modulo a smaller alphabet size. This gives
    /// a potentially less efficient searching, but is a safe approximation.
    /// For needles using only characters in the same Unicode 256-code point page,
    /// there is no search speed degradation.
    const K_LATIN1_ALPHABET_SIZE: usize = 256;
    const K_UC16_ALPHABET_SIZE: usize = 65536; // TODO: Replace with Isolate::K_UC16_ALPHABET_SIZE if possible.

    /// Bad-char shift table stored in the state. It's length is the alphabet size.
    /// For patterns below this length, the skip length of Boyer-Moore is too short
    /// to compensate for the algorithmic overhead compared to simple brute force.
    const KB_MMIN_PATTERN_LENGTH: usize = 7;

    fn is_one_byte_string_u8(string: &[u8]) -> bool {
        true
    }

    fn is_one_byte_string_u16(string: &[u16]) -> bool {
        string.iter().all(|&c| c <= String::K_MAX_ONE_BYTE_CHAR_CODE_U)
    }

    struct Isolate {
        bad_char_shift_table: Vec<i32>,
        good_suffix_shift_table: Vec<i32>,
        suffix_table: Vec<i32>,
    }

    impl Isolate {
        fn new() -> Self {
            let bad_char_shift_table = vec![0; K_UC16_ALPHABET_SIZE];
            let good_suffix_shift_table = vec![0; KB_MMAX_SHIFT + 1];
            let suffix_table = vec![0; KB_MMAX_SHIFT + 1];
            Isolate {
                bad_char_shift_table,
                good_suffix_shift_table,
                suffix_table,
            }
        }

        fn bad_char_shift_table(&mut self) -> &mut [i32] {
            &mut self.bad_char_shift_table
        }

        fn good_suffix_shift_table(&mut self) -> &mut [i32] {
            &mut self.good_suffix_shift_table
        }

        fn suffix_table(&mut self) -> &mut [i32] {
            &mut self.suffix_table
        }
    }

    /// String Search object.
    pub struct StringSearch<'a, PatternChar, SubjectChar> {
        isolate: &'a mut Isolate,
        pattern: Vec<PatternChar>,
        start: usize,
        strategy: SearchFunction<PatternChar, SubjectChar>,
    }

    type SearchFunction<PatternChar, SubjectChar> = fn(&mut StringSearch<PatternChar, SubjectChar>, &[SubjectChar], usize) -> i32;

    impl<'a, PatternChar, SubjectChar> StringSearch<'a, PatternChar, SubjectChar> {
        /// Creates a new StringSearch object.
        pub fn new(isolate: &'a mut Isolate, pattern: &[PatternChar]) -> Self {
            let pattern_vec = pattern.to_vec();
            let start = cmp::max(0, pattern_vec.len().saturating_sub(KB_MMAX_SHIFT));

            let mut search = StringSearch {
                isolate,
                pattern: pattern_vec,
                start,
                strategy: Self::initial_search,
            };

            if mem::size_of::<PatternChar>() > mem::size_of::<SubjectChar>() {
                if mem::size_of::<PatternChar>() == 1 {
                    search.strategy = Self::fail_search;
                    return search;
                }
                if !is_one_byte_string_u16(unsafe {std::mem::transmute::<&[PatternChar], &[u16]>(search.pattern.as_slice())}) {
                    search.strategy = Self::fail_search;
                    return search;
                }
            }
            let pattern_length = search.pattern.len();
            if pattern_length < KB_MMIN_PATTERN_LENGTH {
                if pattern_length == 1 {
                    search.strategy = Self::single_char_search;
                    return search;
                }
                search.strategy = Self::linear_search;
                return search;
            }
            search.strategy = Self::initial_search;
            search
        }

        /// Searches for the pattern in the subject starting at the given index.
        pub fn search(&mut self, subject: &[SubjectChar], index: usize) -> i32 {
            (self.strategy)(self, subject, index)
        }

        /// Returns the alphabet size based on the pattern character type.
        pub fn alphabet_size() -> usize {
            if mem::size_of::<PatternChar>() == 1 {
                // Latin1 needle.
                K_LATIN1_ALPHABET_SIZE
            } else {
                // UC16 needle.
                assert_eq!(mem::size_of::<PatternChar>(), 2);
                K_UC16_ALPHABET_SIZE
            }
        }

        fn fail_search(&mut self, _subject: &[SubjectChar], _index: usize) -> i32 {
            -1
        }

        fn single_char_search(&mut self, subject: &[SubjectChar], start_index: usize) -> i32 {
            assert_eq!(1, self.pattern.len());
            let pattern_first_char = self.pattern[0];
            if mem::size_of::<PatternChar>() > mem::size_of::<SubjectChar>() {
                if Self::exceeds_one_byte(pattern_first_char) {
                    return -1;
                }
            }
            Self::find_first_character(&self.pattern, subject, start_index)
        }

        fn linear_search(&mut self, subject: &[SubjectChar], start_index: usize) -> i32 {
            let pattern = &self.pattern;
            assert!(pattern.len() > 1);
            let pattern_length = pattern.len();
            let mut i = start_index;
            let n = subject.len().saturating_sub(pattern_length);
            while i <= n {
                i = Self::find_first_character(pattern, subject, i);
                if i == -1 {
                    return -1;
                }
                assert!(i <= n);
                i += 1;
                // Loop extracted to separate function to allow using return to do
                // a deeper break.
                if Self::char_compare(&pattern[1..], &subject[i..], pattern_length - 1) {
                    return (i - 1) as i32;
                }
            }
            -1
        }

        fn initial_search(&mut self, subject: &[SubjectChar], index: usize) -> i32 {
            let pattern = &self.pattern;
            let pattern_length = pattern.len();
            // Badness is a count of how much work we have done.  When we have
            // done enough work we decide it's probably worth switching to a better
            // algorithm.
            let mut badness: i32 = -10 - ((pattern_length << 2) as i32);

            // We know our pattern is at least 2 characters, we cache the first so
            // the common case of the first character not matching is faster.
            for i in index..=(subject.len().saturating_sub(pattern_length)) {
                badness += 1;
                if badness <= 0 {
                    let mut i_mut = i;
                    i_mut = Self::find_first_character(pattern, subject, i_mut);
                    if i_mut == -1 {
                        return -1;
                    }
                    assert!(i_mut <= subject.len().saturating_sub(pattern_length));
                    let mut j = 1;
                    loop {
                        if pattern[j] != subject[i_mut + j] {
                            break;
                        }
                        j += 1;
                        if j == pattern_length {
                            return i_mut as i32;
                        }
                    }
                    badness += j as i32;
                } else {
                    self.populate_boyer_moore_horspool_table();
                    self.strategy = Self::boyer_moore_horspool_search;
                    return (self.strategy)(self, subject, i);
                }
            }
            -1
        }

        fn boyer_moore_horspool_search(&mut self, subject: &[SubjectChar], start_index: usize) -> i32 {
            let pattern = &self.pattern;
            let subject_length = subject.len();
            let pattern_length = pattern.len();
            let char_occurrences = self.bad_char_table();
            let mut badness: i32 = -(pattern_length as i32);

            // How bad we are doing without a good-suffix table.
            let last_char = pattern[pattern_length - 1];
            let last_char_shift =
                (pattern_length - 1) as i32 - Self::char_occurrence(char_occurrences, Self::cast_to_subject_char(last_char));
            // Perform search
            let mut index = start_index; // No matches found prior to this index.
            while index <= subject_length.saturating_sub(pattern_length) {
                let mut j = pattern_length - 1;
                let subject_char;
                if let Some(&char) = subject.get(index + j) {
                    subject_char = char;
                } else {
                    return -1;
                }
                while Self::cast_to_pattern_char(last_char) != Self::cast_to_pattern_char(subject_char) {
                    let bc_occ = Self::char_occurrence(char_occurrences, subject_char);
                    let shift = (j as i32) - bc_occ;
                    if shift < 0 {
                        return -1;
                    }
                    index += shift as usize;
                    badness += 1 - shift; // at most zero, so badness cannot increase.
                    if index > subject_length.saturating_sub(pattern_length) {
                        return -1;
                    }
                    if let Some(&char) = subject.get(index + j) {
                        if Self::cast_to_pattern_char(last_char) != Self::cast_to_pattern_char(char) {
                            continue;
                        }
                    } else {
                        return -1;
                    }
                }
                j -= 1;
                while j >= 0 && pattern[j] == subject[index + j] {
                    j -= 1;
                }
                if j < 0 {
                    return index as i32;
                } else {
                    index += last_char_shift as usize;
                    // Badness increases by the number of characters we have
                    // checked, and decreases by the number of characters we
                    // can skip by shifting. It's a measure of how we are doing
                    // compared to reading each character exactly once.
                    badness += (pattern_length - (j as usize)) as i32 - last_char_shift;
                    if badness > 0 {
                        self.populate_boyer_moore_table();
                        self.strategy = Self::boyer_moore_search;
                        return (self.strategy)(self, subject, index);
                    }
                }
            }
            -1
        }

        fn boyer_moore_search(&mut self, subject: &[SubjectChar], start_index: usize) -> i32 {
            let pattern = &self.pattern;
            let subject_length = subject.len();
            let pattern_length = pattern.len();
            // Only preprocess at most kBMMaxShift last characters of pattern.
            let start = self.start;

            let bad_char_occurence = self.bad_char_table();
            let good_suffix_shift = self.good_suffix_shift_table();

            let last_char = pattern[pattern_length - 1];
            let mut index = start_index;
            // Continue search from i.
            while index <= subject_length.saturating_sub(pattern_length) {
                let mut j = pattern_length - 1;
                let c: SubjectChar;
                if let Some(&char) = subject.get(index + j) {
                    c = char;
                } else {
                    return -1;
                }
                while Self::cast_to_pattern_char(last_char) != Self::cast_to_pattern_char(c) {
                    let shift = (j as i32) - Self::char_occurrence(bad_char_occurence, c);
                    if shift < 0 {
                        return -1;
                    }
                    index += shift as usize;
                    if index > subject_length.saturating_sub(pattern_length) {
                        return -1;
                    }

                    if let Some(&char) = subject.get(index + j) {
                        if Self::cast_to_pattern_char(last_char) != Self::cast_to_pattern_char(char) {
                           continue;
                        }
                    } else {
                        return -1;
                    }
                }
                while j >= 0 && pattern[j] == subject[index + j] {
                    j -= 1;
                }
                if j < 0 {
                    return index as i32;
                } else if j < start {
                    // we have matched more than our tables allow us to be smart about.
                    // Fall back on BMH shift.
                    let shift = (pattern_length - 1) as i32 - Self::char_occurrence(bad_char_occurence, Self::cast_to_subject_char(last_char));
                    if shift < 0 {
                        return -1;
                    }
                    index += shift as usize;
                } else {
                    let gs_shift = good_suffix_shift[j + 1];
                    let c_val: SubjectChar;
                    if let Some(&char) = subject.get(index + j) {
                        c_val = char;
                    } else {
                        return -1;
                    }
                    let bc_occ = Self::char_occurrence(bad_char_occurence, c_val);
                    let shift = (j as i32) - bc_occ;
                    let mut final_shift = shift;
                    if gs_shift > shift {
                        final_shift = gs_shift;
                    }
                    if final_shift < 0 {
                        return -1;
                    }
                    index += final_shift as usize;
                }
            }

            -1
        }

        fn populate_boyer_moore_table(&mut self) {
            let pattern_length = self.pattern.len();
            let pattern = &self.pattern;
            // Only look at the last kBMMaxShift characters of pattern (from start_
            // to pattern_length).
            let start = self.start;
            let length = pattern_length - start;

            // Biased tables so that we can use pattern indices as table indices,
            // even if we only cover the part of the pattern from offset start.
            let shift_table = self.good_suffix_shift_table();
            let suffix_table = self.suffix_table();

            // Initialize table.
            for i in start..pattern_length {
                shift_table[i] = length as i32;
            }
            shift_table[pattern_length] = 1;
            suffix_table[pattern_length] = (pattern_length + 1) as i32;

            if pattern_length <= start {
                return;
            }

            // Find suffixes.
            let last_char = pattern[pattern_length - 1];
            let mut suffix = pattern_length + 1;
            {
                let mut i = pattern_length;
                while i > start {
                    let c = pattern[i - 1];
                    while suffix <= pattern_length && Self::cast_to_pattern_char(c) != Self::cast_to_pattern_char(pattern[suffix - 1]) {
                        if shift_table[suffix] == length as i32 {
                            shift_table[suffix] = (suffix - i) as i32;
                        }
                        suffix = suffix_table[suffix] as usize;
                    }
                    suffix_table[i - 1] = (suffix - 1) as i32;
                    suffix -= 1;
                    if suffix == pattern_length {
                        // No suffix to extend, so we check against last_char only.
                        let mut i_mut = i;
                        while (i_mut > start) && (Self::cast_to_pattern_char(pattern[i_mut - 1]) != Self::cast_to_pattern_char(last_char)) {
                            if shift_table[pattern_length] == length as i32 {
                                shift_table[pattern_length] = (pattern_length - i_mut) as i32;
                            }
                            suffix_table[i_mut - 1] = pattern_length as i32;
                            i_mut -= 1;
                        }
                        if i_mut > start {
                            suffix_table[i_mut - 1] = (suffix - 1) as i32;
                        }
                    }
                }
            }
            // Build shift table using suffixes.
            if suffix < pattern_length {
                for i in start..=pattern_length {
                    if shift_table[i] == length as i32 {
                        shift_table[i] = (suffix - start) as i32;
                    }
                    if i == suffix {
                        suffix = suffix_table[suffix] as usize;
                    }
                }
            }
        }

        fn populate_boyer_moore_horspool_table(&mut self) {
            let pattern_length = self.pattern.len();

            let bad_char_occurrence = self.bad_char_table();

            // Only preprocess at most kBMMaxShift last characters of pattern.
            let start = self.start;
            // Run forwards to populate bad_char_table, so that *last* instance
            // of character equivalence class is the one registered.
            // Notice: Doesn't include the last character.
            let table_size = Self::alphabet_size();
            if start == 0 {
                // All patterns less than kBMMaxShift in length.
                for i in 0..table_size {
                    bad_char_occurrence[i] = -1;
                }
            } else {
                for i in 0..table_size {
                    bad_char_occurrence[i] = (start - 1) as i32;
                }
            }
            for i in start..(pattern_length - 1) {
                let c = self.pattern[i];
                let bucket: usize = if mem::size_of::<PatternChar>() == 1 {
                    Self::cast_to_u8(c) as usize
                } else {
                    (Self::cast_to_u16(c) % Self::alphabet_size() as u16) as usize
                };
                bad_char_occurrence[bucket] = i as i32;
            }
        }

        #[inline]
        fn exceeds_one_byte<T>(c: T) -> bool {
            false
        }

        #[inline]
        fn char_occurrence(bad_char_occurrence: &[i32], char_code: SubjectChar) -> i32 {
            if mem::size_of::<SubjectChar>() == 1 {
                bad_char_occurrence[Self::cast_to_u8(char_code) as usize]
            } else if mem::size_of::<Self::pattern[0]>() == 1 {
                if Self::exceeds_one_byte(char_code) {
                    return -1;
                }
                bad_char_occurrence[Self::cast_to_u16(char_code) as usize]
            } else {
                // Both pattern and subject are UC16. Reduce character to equivalence
                // class.
                let equiv_class = (Self::cast_to_u16(char_code) % K_UC16_ALPHABET_SIZE as u16) as usize;
                bad_char_occurrence[equiv_class]
            }
        }

        fn bad_char_table(&mut self) -> &mut [i32] {
            self.isolate.bad_char_shift_table()
        }

        fn good_suffix_shift_table(&mut self) -> &mut [i32] {
            let start = self.start;
            let table = self.isolate.good_suffix_shift_table();
            &mut table[start..]
        }

        fn suffix_table(&mut self) -> &mut [i32] {
            let start = self.start;
            let table = self.isolate.suffix_table();
            &mut table[start..]
        }

        fn find_first_character(pattern: &[PatternChar], subject: &[SubjectChar], index: usize) -> i32 {
            if pattern.is_empty() {
                return -1;
            }

            let pattern_first_char = pattern[0];
            let max_n = subject.len().saturating_sub(pattern.len()).saturating_add(1);

            if mem::size_of::<SubjectChar>() == 2 && Self::cast_to_u16(pattern_first_char) == 0 {
                // Special-case looking for the 0 char in other than one-byte strings.
                // memchr mostly fails in this case due to every other byte being 0 in text
                // that is mostly ascii characters.
                for i in index..max_n {
                    if Self::cast_to_u16(subject[i]) == 0 {
                        return i as i32;
                    }
                }
                return -1;
            }

            let search_byte = Self::get_highest_value_byte(pattern_first_char);
            let search_char = pattern_first_char;
            let mut pos = index;
            loop {
                if max_n <= pos {
                    return -1;
                }
                let char_pos = unsafe {
                    let ptr = subject.as_ptr().add(pos);
                    let len = max_n - pos;
                    let slice = slice::from_raw_parts(ptr as *const u8, len * mem::size_of::<SubjectChar>());
                    let memchr_ptr = memchr::memchr(search_byte, slice);
                    match memchr_ptr {
                        Some(offset) => {
                            ptr.add(offset / mem::size_of::<SubjectChar>())
                        },
                        None => std::ptr::null()
                    }
                };

                if char_pos.is_null() {
                    return -1;
                }

                let aligned_char_pos = Self::align_down(char_pos as usize, mem::size_of::<SubjectChar>()) as *const SubjectChar;
                pos = unsafe { aligned_char_pos.offset_from(subject.as_ptr()) as usize };
                if subject[pos] == search_char {
                    return pos as i32;
                }

                pos += 1;
                if pos >= max_n {
                    break;
                }
            }
            -1
        }

        fn char_compare(pattern: &[PatternChar], subject: &[SubjectChar], length: usize) -> bool {
            assert!(length > 0);
            for pos in 0..length {
                if pattern[pos] != subject[pos] {
                    return false;
                }
            }
            true
        }

        fn cast_to_pattern_char<T, U>(val: T) -> U
            where
                T: Copy,
                U: From<u8> + From<u16>
        {
            let u8_val = val as u8;
            if mem::size_of::<T>() == 1 {
                U::from(u8_val)
            } else {
                let u16_val = unsafe { *(val as *const _ as *const u16) };
                U::from(u16_val)
            }
        }

        fn cast_to_subject_char<T, U>(val: T) -> U
            where
                T: Copy,
                U: From<u8> + From<u16>
        {
            let u8_val = val as u8;
            if mem::size_of::<T>() == 1 {
                U::from(u8_val)
            } else {
                let u16_val = unsafe { *(val as *const _ as *const u16) };
                U::from(u16_val)
            }
        }

        fn cast_to_u8<T>(val: T) -> u8
            where
                T: Copy,
        {
            unsafe { *(val as *const _ as *const u8) }
        }

        fn cast_to_u16<T>(val: T) -> u16
            where
                T: Copy,
        {
            unsafe { *(val as *const _ as *const u16) }
        }

        fn get_highest_value_byte<T>(character: T) -> u8
        where T: Copy
        {
            if mem::size_of::<T>() == 2 {
                let char_u16: u16 = unsafe { *(std::ptr::addr_of!(character) as *const u16) };
                cmp::max((char_u16 & 0xFF) as u8, (char_u16 >> 8) as u8)
            } else {
                unsafe { *(std::ptr::addr_of!(character) as *const u8) }
            }
        }

        fn align_down(value: usize, alignment: usize) -> usize {
            value & !(alignment - 1)
        }
    }

    struct String {}
    impl String {
        const K_MAX_ONE_BYTE_CHAR_CODE_U: u16 = 255;

        fn is_one_byte(data: *const u16, length: usize) -> bool {
            unsafe {
                for i in 0..length {
                    let value = *data.add(i);
                    if value > String::K_MAX_ONE_BYTE_CHAR_CODE_U {
                        return false;
                    }
                }
                true
            }
        }
    }

    pub fn search_string<SubjectChar, PatternChar>(
        isolate: &mut Isolate,
        subject: &[SubjectChar],
        pattern: &[PatternChar],
        start_index: usize,
    ) -> i32 {
        let mut search: StringSearch<PatternChar, SubjectChar> = StringSearch::new(isolate, pattern);
        search.search(subject, start_index)
    }

    pub fn search_string_raw<SubjectChar, PatternChar>(
        isolate: &mut Isolate,
        subject_ptr: *const SubjectChar,
        subject_length: usize,
        pattern_ptr: *const PatternChar,
        pattern_length: usize,
        start_index: usize,
    ) -> usize {
        let subject: &[SubjectChar] = unsafe { slice::from_raw_parts(subject_ptr, subject_length) };
        let pattern: &[PatternChar] = unsafe { slice::from_raw_parts(pattern_ptr, pattern_length) };
        search_string(isolate, subject, pattern, start_index) as usize
    }
}