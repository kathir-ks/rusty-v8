// Converted from V8 C++ source files:
// Header: string-search.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct Vector<T> {
        ptr: *const T,
        length: usize,
    }

    impl<T> Vector<T> {
        pub fn new(ptr: *const T, length: usize) -> Self {
            Vector { ptr, length }
        }

        pub fn begin(&self) -> *const T {
            self.ptr
        }

        pub fn length(&self) -> usize {
            self.length
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            if index < self.length {
                unsafe { Some(&*self.ptr.add(index)) }
            } else {
                None
            }
        }
    }
}

pub mod execution {
    pub struct Isolate {
        bm_max_shift: i32,
        bad_char_shift_table: Vec<i32>,
        good_suffix_shift_table: Vec<i32>,
        suffix_table: Vec<i32>,
    }

    impl Isolate {
        pub const kBMMaxShift: i32 = 64;
        pub const kUC16AlphabetSize: i32 = 256;

        pub fn new() -> Self {
            let bad_char_shift_table_size = 256;
            let good_suffix_shift_table_size = 65;
            let suffix_table_size = 65;

            Isolate {
                bm_max_shift: Self::kBMMaxShift,
                bad_char_shift_table: vec![0; bad_char_shift_table_size],
                good_suffix_shift_table: vec![0; good_suffix_shift_table_size],
                suffix_table: vec![0; suffix_table_size],
            }
        }

        pub fn bad_char_shift_table(&mut self) -> *mut i32 {
            self.bad_char_shift_table.as_mut_ptr()
        }

        pub fn good_suffix_shift_table(&mut self) -> *mut i32 {
            self.good_suffix_shift_table.as_mut_ptr()
        }

        pub fn suffix_table(&mut self) -> *mut i32 {
            self.suffix_table.as_mut_ptr()
        }
    }
}

pub mod objects {
    pub struct String {}

    impl String {
        pub const kMaxOneByteCharCodeU: u16 = 255;

        pub fn IsOneByte(ptr: *const u8, length: usize) -> bool {
            unsafe {
                for i in 0..length {
                    if *ptr.add(i) > String::kMaxOneByteCharCodeU as u8 {
                        return false;
                    }
                }
                true
            }
            
        }
    }
}

pub mod internal {
    use super::base;
    use super::execution::Isolate;
    use super::objects::String;
    use std::cmp;
    use std::mem::size_of;
    use std::os::raw::c_void;

    pub struct StringSearchBase { }

    impl StringSearchBase {
        pub const kBMMaxShift: i32 = Isolate::kBMMaxShift;
        pub const kLatin1AlphabetSize: i32 = 256;
        pub const kUC16AlphabetSize: i32 = Isolate::kUC16AlphabetSize;
        pub const kBMMinPatternLength: i32 = 7;

        #[inline]
        fn IsOneByteString(string: base::Vector<u8>) -> bool {
            true
        }

        #[inline]
        fn IsOneByteString_uc16(string: base::Vector<u16>) -> bool {
            String::IsOneByte(string.begin() as *const u8, string.length())
        }

    }

    pub struct StringSearch<'a, PatternChar, SubjectChar> {
        isolate_: &'a mut Isolate,
        pattern_: base::Vector<PatternChar>,
        start_: i32,
        strategy_: SearchFunction<PatternChar, SubjectChar>,
    }

    type SearchFunction<PatternChar, SubjectChar> = fn(
        search: &mut StringSearch<PatternChar, SubjectChar>,
        subject: base::Vector<SubjectChar>,
        index: i32,
    ) -> i32;

    impl<'a, PatternChar, SubjectChar> StringSearch<'a, PatternChar, SubjectChar>
    where
        PatternChar: PartialEq + Copy + std::fmt::Debug + Into<u32>,
        SubjectChar: PartialEq + Copy + std::fmt::Debug + Into<u32>,
    {
        pub fn new(isolate: &'a mut Isolate, pattern: base::Vector<PatternChar>) -> Self {
            let start_ = cmp::max(0, pattern.length() as i32 - StringSearchBase::kBMMaxShift);
            let mut search = StringSearch {
                isolate_: isolate,
                pattern_: pattern,
                start_: start_,
                strategy_: Self::fail_search,
            };
             if size_of::<PatternChar>() > size_of::<SubjectChar>() {
                if !search.is_one_byte_string(&search.pattern_) {
                    search.strategy_ = Self::fail_search;
                    return search;
                }
            }

            let pattern_length = search.pattern_.length() as i32;
            if pattern_length < StringSearchBase::kBMMinPatternLength {
                if pattern_length == 1 {
                    search.strategy_ = Self::single_char_search;
                    return search;
                }
                search.strategy_ = Self::linear_search;
                return search;
            }
            search.strategy_ = Self::initial_search;
            search
        }

        pub fn search(&mut self, subject: base::Vector<SubjectChar>, index: i32) -> i32 {
            (self.strategy_)(self, subject, index)
        }
        
        fn is_one_byte_string(&self, pattern: &base::Vector<PatternChar>) -> bool{
            true
        }

        fn alphabet_size() -> i32 {
            if size_of::<PatternChar>() == 1 {
                StringSearchBase::kLatin1AlphabetSize
            } else {
                StringSearchBase::kUC16AlphabetSize
            }
        }

        fn fail_search(
            _search: &mut StringSearch<PatternChar, SubjectChar>,
            _subject: base::Vector<SubjectChar>,
            _index: i32,
        ) -> i32 {
            -1
        }

        fn single_char_search(
            search: &mut StringSearch<PatternChar, SubjectChar>,
            subject: base::Vector<SubjectChar>,
            start_index: i32,
        ) -> i32 {
            Self::single_char_search_impl(search, subject, start_index)
        }
        
        fn single_char_search_impl(
            search: &mut StringSearch<PatternChar, SubjectChar>,
            subject: base::Vector<SubjectChar>,
            index: i32,
        ) -> i32 {
            assert_eq!(1, search.pattern_.length());
            let pattern_first_char = search.pattern_.get(0).unwrap();
            if size_of::<PatternChar>() > size_of::<SubjectChar>() {
                if search.exceeds_one_byte(*pattern_first_char) {
                    return -1;
                }
            }
            find_first_character(search.pattern_, subject, index)
        }

        fn linear_search(
            search: &mut StringSearch<PatternChar, SubjectChar>,
            subject: base::Vector<SubjectChar>,
            start_index: i32,
        ) -> i32 {
            Self::linear_search_impl(search, subject, start_index)
        }

        fn linear_search_impl(
            search: &mut StringSearch<PatternChar, SubjectChar>,
            subject: base::Vector<SubjectChar>,
            index: i32,
        ) -> i32 {
            let pattern = search.pattern_;
            assert!(pattern.length() > 1);
            let pattern_length = pattern.length();
            let mut i = index;
            let n = subject.length() - pattern_length;
            while i <= n as i32 {
                i = find_first_character(pattern, subject, i) as i32;
                if i == -1 {
                    return -1;
                }
                assert!(i <= n as i32);
                i += 1;
                if char_compare(
                    pattern.begin() as *const PatternChar,
                    unsafe { subject.begin().add(i as usize) } as *const SubjectChar,
                    pattern_length - 1,
                ) {
                    return i - 1;
                }
            }
            -1
        }
        
        fn exceeds_one_byte(&self, c: PatternChar) -> bool {
             if size_of::<PatternChar>() == 1{
                 false
             }else{
                 let char_code: u32 = c.into();
                 char_code > String::kMaxOneByteCharCodeU as u32
             }
        }

        fn initial_search(
            search: &mut StringSearch<PatternChar, SubjectChar>,
            subject: base::Vector<SubjectChar>,
            start_index: i32,
        ) -> i32 {
           Self::initial_search_impl(search, subject, start_index)
        }
        
        fn initial_search_impl(
            search: &mut StringSearch<PatternChar, SubjectChar>,
            subject: base::Vector<SubjectChar>,
            index: i32,
        ) -> i32 {
            let pattern = search.pattern_;
            let pattern_length = pattern.length();
            let mut badness = -10 - (pattern_length << 2) as i32;
            
            for i in index..=(subject.length() - pattern_length) as i32 {
                badness += 1;
                if badness <= 0 {
                    let i_usize = i as usize;
                    let found_index = find_first_character(pattern, subject, i) as i32;

                    if found_index == -1 {
                        return -1;
                    }
                    
                    let mut j = 1;
                    
                    while j < pattern_length {
                        let pattern_char = unsafe { *(pattern.begin().add(j)) };
                        let subject_char = unsafe { *(subject.begin().add(i_usize + j)) };

                        if pattern_char != subject_char {
                            break;
                        }
                        j += 1;
                    }

                    if j == pattern_length {
                        return i;
                    }
                    badness += j as i32;
                } else {
                    search.populate_boyer_moore_horspool_table();
                    search.strategy_ = Self::boyer_moore_horspool_search;
                    return (search.strategy_)(search, subject, i);
                }
            }
            -1
        }

        fn boyer_moore_horspool_search(
            search: &mut StringSearch<PatternChar, SubjectChar>,
            subject: base::Vector<SubjectChar>,
            start_index: i32,
        ) -> i32 {
            Self::boyer_moore_horspool_search_impl(search, subject, start_index)
        }
        
        fn boyer_moore_horspool_search_impl(
            search: &mut StringSearch<PatternChar, SubjectChar>,
            subject: base::Vector<SubjectChar>,
            start_index: i32,
        ) -> i32 {
            let pattern = search.pattern_;
            let subject_length = subject.length() as i32;
            let pattern_length = pattern.length() as i32;
            let char_occurrences = search.bad_char_table();
            let mut badness = -pattern_length;
            
            let last_char = unsafe { *(pattern.begin().add(pattern_length as usize - 1)) };
            let last_char_shift = pattern_length - 1 - char_occurrence(char_occurrences, last_char);
            
            let mut index = start_index;
            while index <= subject_length - pattern_length {
                let mut j = pattern_length - 1;
                let subject_char;

                unsafe {
                    while last_char != (subject_char = *(subject.begin().add((index + j) as usize))) {
                        let bc_occ = char_occurrence(char_occurrences, subject_char);
                        let shift = j - bc_occ;
                        index += shift;
                        badness += 1 - shift;
                        if index > subject_length - pattern_length {
                            return -1;
                        }
                    }
                }

                j -= 1;
                unsafe {
                    while j >= 0 && *(pattern.begin().add(j as usize)) == *(subject.begin().add((index + j) as usize)) {
                        j -= 1;
                    }
                }

                if j < 0 {
                    return index;
                } else {
                    index += last_char_shift;
                    badness += (pattern_length - j) - last_char_shift;
                    if badness > 0 {
                        search.populate_boyer_moore_table();
                        search.strategy_ = Self::boyer_moore_search;
                        return (search.strategy_)(search, subject, index);
                    }
                }
            }
            -1
        }
        
        fn boyer_moore_search(
            search: &mut StringSearch<PatternChar, SubjectChar>,
            subject: base::Vector<SubjectChar>,
            start_index: i32,
        ) -> i32 {
            Self::boyer_moore_search_impl(search, subject, start_index)
        }
        
        fn boyer_moore_search_impl(
            search: &mut StringSearch<PatternChar, SubjectChar>,
            subject: base::Vector<SubjectChar>,
            start_index: i32,
        ) -> i32 {
            let pattern = search.pattern_;
            let subject_length = subject.length() as i32;
            let pattern_length = pattern.length() as i32;
            let start = search.start_;
        
            let bad_char_occurence = search.bad_char_table();
            let good_suffix_shift = search.good_suffix_shift_table();
        
            let last_char = unsafe { *pattern.begin().add((pattern_length - 1) as usize) };
            let mut index = start_index;
        
            while index <= subject_length - pattern_length {
                let mut j = pattern_length - 1;
                let c: SubjectChar;
                unsafe {
                    while last_char != (c = *subject.begin().add((index + j) as usize)) {
                        let shift = j - char_occurrence(bad_char_occurence, c);
                        index += shift;
                        if index > subject_length - pattern_length {
                            return -1;
                        }
                    }
                }
        
                unsafe {
                    while j >= 0 && *pattern.begin().add(j as usize) == (c = *subject.begin().add((index + j) as usize)) {
                        j -= 1;
                    }
                }
        
                if j < 0 {
                    return index;
                } else if j < start {
                    let shift = pattern_length - 1 - char_occurrence(bad_char_occurence, last_char);
                    index += shift;
                } else {
                    let gs_shift = unsafe { *good_suffix_shift.add((j + 1) as usize) };
                    let c: SubjectChar = unsafe { *subject.begin().add((index + j) as usize) };
                    let bc_occ = char_occurrence(bad_char_occurence, c);
                    let shift = j - bc_occ;
        
                    let shift = if gs_shift > shift { gs_shift } else { shift };
                    index += shift;
                }
            }
        
            -1
        }
        
        fn populate_boyer_moore_table(&mut self) {
            let pattern_length = self.pattern_.length() as i32;
            let pattern = self.pattern_;
            let start = self.start_;
            let length = pattern_length - start;
        
            let shift_table = self.good_suffix_shift_table();
            let suffix_table = self.suffix_table();
        
            unsafe {
                for i in start..pattern_length {
                    *shift_table.add(i as usize) = length;
                }
                *shift_table.add(pattern_length as usize) = 1;
                *suffix_table.add(pattern_length as usize) = pattern_length + 1;
            }
        
            if pattern_length <= start {
                return;
            }
        
            let last_char = unsafe { *pattern.begin().add((pattern_length - 1) as usize) };
            let mut suffix = pattern_length + 1;
            {
                let mut i = pattern_length;
                while i > start {
                    let c = unsafe { *pattern.begin().add((i - 1) as usize) };
                    unsafe {
                        while suffix <= pattern_length && c != *pattern.begin().add((suffix - 1) as usize) {
                            if *shift_table.add(suffix as usize) == length {
                                *shift_table.add(suffix as usize) = suffix - i;
                            }
                            suffix = *suffix_table.add(suffix as usize);
                        }
                        *suffix_table.add((i - 1) as usize) = suffix - 1;
                    }
        
                    suffix -= 1;
                    i -= 1;
                    if suffix == pattern_length {
                        while i > start && unsafe { *pattern.begin().add((i - 1) as usize) != last_char } {
                            unsafe {
                                if *shift_table.add(pattern_length as usize) == length {
                                    *shift_table.add(pattern_length as usize) = pattern_length - i;
                                }
                                *suffix_table.add((i - 1) as usize) = pattern_length;
                            }
                            i -= 1;
                        }
                        if i > start {
                            unsafe { *suffix_table.add((i - 1) as usize) = suffix - 1 };
                            i -= 1;
                        }
                    }
                }
            }
        
            if suffix < pattern_length {
                for i in start..=pattern_length {
                    unsafe {
                        if *shift_table.add(i as usize) == length {
                            *shift_table.add(i as usize) = suffix - start;
                        }
                        if i == suffix {
                            suffix = *suffix_table.add(suffix as usize);
                        }
                    }
                }
            }
        }

        fn populate_boyer_moore_horspool_table(&mut self) {
            let pattern_length = self.pattern_.length() as i32;
            let bad_char_occurrence = self.bad_char_table();
            let start = self.start_;
            let table_size = Self::alphabet_size();
            
            if start == 0 {
                unsafe {
                    std::ptr::write_bytes(bad_char_occurrence as *mut i32 as *mut c_void, 0xff, (table_size * std::mem::size_of::<i32>()) as usize);
                }
            } else {
                for i in 0..table_size {
                    unsafe { *bad_char_occurrence.add(i as usize) = start - 1; }
                }
            }
            
            for i in start..pattern_length - 1 {
                let c = unsafe { *self.pattern_.begin().add(i as usize) };
                let bucket: i32 = if std::mem::size_of::<PatternChar>() == 1 {
                    c.into() as i32
                } else {
                    (c.into() as i32) % Self::alphabet_size()
                };
                unsafe { *bad_char_occurrence.add(bucket as usize) = i; }
            }
        }
        
        fn bad_char_table(&mut self) -> *mut i32 {
            self.isolate_.bad_char_shift_table()
        }

        fn good_suffix_shift_table(&mut self) -> *mut i32 {
            let start = self.start_;
            let table = self.isolate_.good_suffix_shift_table();
            unsafe { table.offset(-(start as isize)) }
        }

        fn suffix_table(&mut self) -> *mut i32 {
            let start = self.start_;
            let table = self.isolate_.suffix_table();
            unsafe { table.offset(-(start as isize)) }
        }
    }
    
    fn char_occurrence<SubjectChar: Into<u32>>(bad_char_occurrence: *mut i32, char_code: SubjectChar) -> i32 {
        if size_of::<SubjectChar>() == 1 {
            unsafe { *bad_char_occurrence.add(char_code.into() as usize) }
        } else {
             if char_code.into() > String::kMaxOneByteCharCodeU as u32{
                -1
             }else{
                 unsafe { *bad_char_occurrence.add(char_code.into() as usize) }
             }
           
        }
    }
    
    #[inline]
    fn align_down<T, U>(value: *const T, alignment: U) -> *const T
    where
        U: std::ops::Sub<Output = U> + std::ops::BitAnd<Output = U> + From<u8>,
        U: Copy,
    {
        let alignment_minus_one = alignment - U::from(1);
        let aligned_value = value as usize & !(alignment_minus_one.into() as usize);
        aligned_value as *const T
    }
    
    #[inline]
    fn get_highest_value_byte<T>(character: T) -> u8
    where T: Into<u32>
    {
        let char_code: u32 = character.into();
        let lower_byte = (char_code & 0xFF) as u8;
        let upper_byte = (char_code >> 8) as u8;
        std::cmp::max(lower_byte, upper_byte)
    }
    
    fn find_first_character<PatternChar, SubjectChar>(
        pattern: base::Vector<PatternChar>,
        subject: base::Vector<SubjectChar>,
        index: i32,
    ) -> i32
    where
        PatternChar: PartialEq + Copy + std::fmt::Debug + Into<u32>,
        SubjectChar: PartialEq + Copy + std::fmt::Debug + Into<u32>,
    {
        let pattern_first_char = unsafe { *pattern.begin() };
        let max_n = (subject.length() - pattern.length() + 1) as i32;
    
        if size_of::<SubjectChar>() == 2 && pattern_first_char.into() == 0 {
            for i in index..max_n {
                let i_usize = i as usize;
                let subject_char = unsafe { *subject.begin().add(i_usize) };
    
                if subject_char.into() == 0 {
                    return i;
                }
            }
            return -1;
        }
        
        let search_byte = get_highest_value_byte(pattern_first_char);
        let search_char = pattern_first_char;
    
        let mut pos = index;
        
        while pos < max_n {
           
            let remaining_length = (max_n - pos) as usize * size_of::<SubjectChar>();
            let char_pos_ptr = unsafe {
                libc::memchr(
                    subject.begin().add(pos as usize) as *const std::ffi::c_void,
                    search_byte as i32,
                    remaining_length,
                ) as *const SubjectChar
            };
            
            if char_pos_ptr.is_null() {
                return -1;
            }
            
            let char_pos = align_down(char_pos_ptr, size_of::<SubjectChar>());
            pos = unsafe { (char_pos.offset_from(subject.begin())) as i32 };
            
            let pos_usize = pos as usize;
            let subject_char = unsafe { *subject.begin().add(pos_usize) };
            
            if subject_char == search_char {
                return pos;
            }
            pos += 1;
        }
        -1
    }
    
    #[inline]
    fn char_compare<PatternChar, SubjectChar>(
        pattern: *const PatternChar,
        subject: *const SubjectChar,
        length: usize,
    ) -> bool
    where
        PatternChar: PartialEq,
        SubjectChar: PartialEq,
    {
        assert!(length > 0);
        let mut pos = 0;
        while pos < length {
            unsafe {
                if *pattern.add(pos) != *subject.add(pos) {
                    return false;
                }
            }
            pos += 1;
        }
        true
    }

    pub fn search_string<SubjectChar, PatternChar>(
        isolate: &mut Isolate,
        subject: base::Vector<SubjectChar>,
        pattern: base::Vector<PatternChar>,
        start_index: i32,
    ) -> i32
    where
        SubjectChar: PartialEq + Copy + std::fmt::Debug + Into<u32>,
        PatternChar: PartialEq + Copy + std::fmt::Debug + Into<u32>,
    {
        let mut search = StringSearch::new(isolate, pattern);
        search.search(subject, start_index)
    }
    
    pub struct DisallowGarbageCollection {}
    
    pub fn search_string_raw<SubjectChar, PatternChar>(
        isolate: &mut Isolate,
        subject_ptr: *const SubjectChar,
        subject_length: i32,
        pattern_ptr: *const PatternChar,
        pattern_length: i32,
        start_index: i32,
    ) -> i32
    where
        SubjectChar: PartialEq + Copy + std::fmt::Debug + Into<u32>,
        PatternChar: PartialEq + Copy + std::fmt::Debug + Into<u32>,
    {
        let subject = base::Vector::new(subject_ptr, subject_length as usize);
        let pattern = base::Vector::new(pattern_ptr, pattern_length as usize);
        search_string(isolate, subject, pattern, start_index)
    }
}
