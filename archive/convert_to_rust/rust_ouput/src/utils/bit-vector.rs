// Converted from V8 C++ source files:
// Header: bit-vector.h
// Implementation: bit-vector.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod zone {
    pub struct Zone {}
    impl Zone {
        pub fn allocate_array<T>(&mut self, count: usize) -> *mut T {
            let mut v: Vec<T> = Vec::with_capacity(count);
            let ptr = v.as_mut_ptr();
            std::mem::forget(v); // Prevent deallocation by Rust
            ptr
        }
    }
}

pub mod base {
    pub mod bits {
        pub fn count_trailing_zeros(x: usize) -> usize {
            x.trailing_zeros() as usize
        }

        pub fn count_population(x: usize) -> usize {
            x.count_ones() as usize
        }

        pub fn round_up_to_power_of_two32(x: u32) -> u32 {
            let mut x = x;
            x -= 1;
            x |= x >> 1;
            x |= x >> 2;
            x |= x >> 4;
            x |= x >> 8;
            x |= x >> 16;
            x += 1;
            x
        }
    }
}

pub mod utils {
    pub struct Utils {}
    impl Utils {
        pub fn round_down(i: i32, alignment: i32) -> i32 {
            i - (i % alignment + alignment) % alignment
        }
    }
}

pub use utils::*;

#[macro_export]
macro_rules! DCHECK {
    ($cond:expr) => {
        if !$cond {
            panic!("DCHECK failed: {}", stringify!($cond));
        }
    };
}

#[macro_export]
macro_rules! CHECK_GE {
    ($left:expr, $right:expr) => {
        if $left < $right {
            panic!("CHECK_GE failed: {} >= {}", stringify!($left), stringify!($right));
        }
    };
}

#[macro_export]
macro_rules! V8_INLINE {
    () => {};
}

#[macro_export]
macro_rules! V8_ASSUME {
    ($cond:expr) => {
        assume($cond);
    };
}

#[cfg(target_arch = "x86_64")]
const K_BITS_PER_SYSTEM_POINTER: usize = 64;
#[cfg(target_arch = "x86")]
const K_BITS_PER_SYSTEM_POINTER: usize = 32;

const K_BITS_PER_SYSTEM_POINTER_LOG2: usize = match K_BITS_PER_SYSTEM_POINTER {
    64 => 6,
    32 => 5,
    _ => panic!("Unsupported architecture"),
};

pub struct V8_EXPORT_PRIVATE {}

pub struct BitVector {
    length_: i32,
    data_: DataStorage,
    data_begin_: *mut usize,
    data_end_: *mut usize,
}

union DataStorage {
    ptr_: *mut usize,
    inline_: usize,
}

impl Copy for DataStorage {}

impl Clone for DataStorage {
    fn clone(&self) -> Self {
        Self { ptr_: self.ptr_ }
    }
}

impl BitVector {
    const K_DATA_BITS: usize = K_BITS_PER_SYSTEM_POINTER;
    const K_DATA_BIT_SHIFT: usize = K_BITS_PER_SYSTEM_POINTER_LOG2;

    pub fn new() -> Self {
        BitVector {
            length_: 0,
            data_: DataStorage { inline_: 0 },
            data_begin_: &mut (unsafe { &mut DataStorage { inline_: 0 }.inline_ }),
            data_end_: &mut (unsafe { &mut DataStorage { inline_: 0 }.inline_ }) as *mut usize,
        }
    }

    pub fn with_length(length: i32, zone: &mut zone::Zone) -> Self {
        DCHECK!(length >= 0);
        let data_length = ((length + Self::K_DATA_BITS as i32 - 1) >> Self::K_DATA_BIT_SHIFT) as usize;
        let mut data_begin_: *mut usize;
        let mut data_end_: *mut usize;
        let data_: DataStorage;
        if data_length > 1 {
            let ptr = zone.allocate_array::<usize>(data_length);
            let slice = unsafe { std::slice::from_raw_parts_mut(ptr, data_length) };
            for i in 0..data_length {
                slice[i] = 0;
            }
            data_begin_ = ptr;
            data_end_ = unsafe { ptr.add(data_length) };
            data_ = DataStorage { ptr_: ptr };
        } else {
            let mut inline_data: usize = 0;
            data_begin_ = &mut inline_data;
            data_end_ = unsafe { data_begin_.add(1) };
            data_ = DataStorage { inline_: 0 };
        }

        BitVector {
            length_: length,
            data_: data_,
            data_begin_: data_begin_,
            data_end_: data_end_,
        }
    }

    pub fn copy(other: &BitVector, zone: &mut zone::Zone) -> Self {
        let length_ = other.length_;
        let data_length = other.data_length();
        let mut data_begin_: *mut usize;
        let mut data_end_: *mut usize;
        let data_: DataStorage;

        if !other.is_inline() {
            DCHECK!(data_length > 1);
            let ptr = zone.allocate_array::<usize>(data_length);
            let dest_slice = unsafe { std::slice::from_raw_parts_mut(ptr, data_length) };
            let src_slice = unsafe { std::slice::from_raw_parts(other.data_begin_, data_length) };
            dest_slice.copy_from_slice(src_slice);
            data_begin_ = ptr;
            data_end_ = unsafe { ptr.add(data_length) };
            data_ = DataStorage { ptr_: ptr };
        } else {
            let mut inline_data: usize = other.data_.inline_;
            data_begin_ = &mut inline_data;
            data_end_ = unsafe { data_begin_.add(1) };
            data_ = DataStorage { inline_: other.data_.inline_ };
        }

        BitVector {
            length_: length_,
            data_: data_,
            data_begin_: data_begin_,
            data_end_: data_end_,
        }
    }

    pub fn copy_from(&mut self, other: &BitVector) {
        DCHECK!(other.length() == self.length());
        DCHECK!(self.is_inline() == other.is_inline());
        let data_length = self.data_length();
        let dest_slice = unsafe { std::slice::from_raw_parts_mut(self.data_begin_, data_length) };
        let src_slice = unsafe { std::slice::from_raw_parts(other.data_begin_, data_length) };
        dest_slice.copy_from_slice(src_slice);
    }

    pub fn resize(&mut self, new_length: i32, zone: &mut zone::Zone) {
        DCHECK!(new_length > self.length());
        let old_data_length = self.data_length();
        DCHECK!(old_data_length >= 1);
        let new_data_length = ((new_length + Self::K_DATA_BITS as i32 - 1) >> Self::K_DATA_BIT_SHIFT) as usize;
        if new_data_length > old_data_length {
            let new_data = zone.allocate_array::<usize>(new_data_length);
            let new_data_slice = unsafe { std::slice::from_raw_parts_mut(new_data, new_data_length) };
            let old_data_slice = unsafe { std::slice::from_raw_parts(self.data_begin_, old_data_length) };
            new_data_slice[..old_data_length].copy_from_slice(old_data_slice);
            for i in old_data_length..new_data_length {
                new_data_slice[i] = 0;
            }

            self.data_begin_ = new_data;
            self.data_end_ = unsafe { new_data.add(new_data_length) };
        }
        self.length_ = new_length;
    }

    pub fn contains(&self, i: i32) -> bool {
        DCHECK!(i >= 0 && i < self.length());
        unsafe { (*self.data_begin_.add(self.word(i) as usize) & self.bit(i)) != 0 }
    }

    pub fn add(&mut self, i: i32) {
        DCHECK!(i >= 0 && i < self.length());
        unsafe {
            *self.data_begin_.add(self.word(i) as usize) |= self.bit(i);
        }
    }

    pub fn add_all(&mut self) {
        let data_length = self.data_length();
        let slice = unsafe { std::slice::from_raw_parts_mut(self.data_begin_, data_length) };
        for i in 0..data_length {
            slice[i] = usize::MAX;
        }
    }

    pub fn remove(&mut self, i: i32) {
        DCHECK!(i >= 0 && i < self.length());
        unsafe {
            *self.data_begin_.add(self.word(i) as usize) &= !self.bit(i);
        }
    }

    pub fn union(&mut self, other: &BitVector) {
        DCHECK!(other.length() == self.length());
        for i in 0..self.data_length() {
            unsafe {
                *self.data_begin_.add(i) |= *other.data_begin_.add(i);
            }
        }
    }

    pub fn union_is_changed(&mut self, other: &BitVector) -> bool {
        DCHECK!(other.length() == self.length());
        let mut changed = false;
        for i in 0..self.data_length() {
            let old_data = unsafe { *self.data_begin_.add(i) };
            unsafe { *self.data_begin_.add(i) |= *other.data_begin_.add(i) };
            if unsafe { *self.data_begin_.add(i) } != old_data {
                changed = true;
            }
        }
        return changed;
    }

    pub fn intersect(&mut self, other: &BitVector) {
        DCHECK!(other.length() == self.length());
        for i in 0..self.data_length() {
            unsafe {
                *self.data_begin_.add(i) &= *other.data_begin_.add(i);
            }
        }
    }

    pub fn intersect_is_changed(&mut self, other: &BitVector) -> bool {
        DCHECK!(other.length() == self.length());
        let mut changed = false;
        for i in 0..self.data_length() {
            let old_data = unsafe { *self.data_begin_.add(i) };
            unsafe { *self.data_begin_.add(i) &= *other.data_begin_.add(i) };
            if unsafe { *self.data_begin_.add(i) } != old_data {
                changed = true;
            }
        }
        return changed;
    }

    pub fn subtract(&mut self, other: &BitVector) {
        DCHECK!(other.length() == self.length());
        for i in 0..self.data_length() {
            unsafe {
                *self.data_begin_.add(i) &= !*other.data_begin_.add(i);
            }
        }
    }

    pub fn clear(&mut self) {
        let data_length = self.data_length();
        let slice = unsafe { std::slice::from_raw_parts_mut(self.data_begin_, data_length) };
        for i in 0..data_length {
            slice[i] = 0;
        }
    }

    pub fn is_empty(&self) -> bool {
        let slice = unsafe { std::slice::from_raw_parts(self.data_begin_, self.data_length()) };
        slice.iter().all(|&x| x == 0)
    }

    pub fn equals(&self, other: &BitVector) -> bool {
        let slice1 = unsafe { std::slice::from_raw_parts(self.data_begin_, self.data_length()) };
        let slice2 = unsafe { std::slice::from_raw_parts(other.data_begin_, other.data_length()) };
        slice1 == slice2
    }

    pub fn count(&self) -> i32 {
        let slice = unsafe { std::slice::from_raw_parts(self.data_begin_, self.data_length()) };
        slice.iter().fold(0, |cnt, &word| cnt + base::bits::count_population(word) as i32)
    }

    pub fn length(&self) -> i32 {
        self.length_
    }

    fn is_inline(&self) -> bool {
        self.data_begin_ == &self.data_.inline_ as *const usize as *mut usize
    }

    fn data_length(&self) -> usize {
        unsafe { self.data_end_.offset_from(self.data_begin_) as usize }
    }

    #[inline]
    fn word(&self, index: i32) -> i32 {
        V8_ASSUME!(index >= 0);
        index >> Self::K_DATA_BIT_SHIFT
    }

    #[inline]
    fn bit(&self, index: i32) -> usize {
        1usize << (index & (Self::K_DATA_BITS as i32 - 1))
    }

    pub fn begin(&self) -> Iterator {
        Iterator::new(self, IteratorStartTag {})
    }

    pub fn end(&self) -> Iterator {
        Iterator::new(self, IteratorEndTag {})
    }
}

impl Drop for BitVector {
    fn drop(&mut self) {
        if !self.is_inline() {
            unsafe {
                let data_length = self.data_length();
                drop(Vec::from_raw_parts(self.data_begin_, data_length, data_length));
            }
        }
    }
}

impl PartialEq for BitVector {
    fn eq(&self, other: &Self) -> bool {
        if self.length() != other.length() {
            return false;
        }
        if self.data_length() != other.data_length() {
            return false;
        }
        let slice1 = unsafe { std::slice::from_raw_parts(self.data_begin_, self.data_length()) };
        let slice2 = unsafe { std::slice::from_raw_parts(other.data_begin_, other.data_length()) };
        slice1 == slice2
    }
}

#[derive(PartialEq, Eq)]
pub struct Iterator {
    target_: *const BitVector,
    ptr_: *mut usize,
    end_: *mut usize,
    current_index_: i32,
}

impl Iterator {
    fn new(target: &BitVector, tag: IteratorTag) -> Self {
        match tag {
            IteratorTag::Start(start_tag) => {
                let ptr_ = target.data_begin_;
                let end_ = target.data_end_;
                let mut current_index_: i32 = 0;

                let mut ptr = ptr_;
                let end = end_;

                if ptr < end {
                    unsafe {
                        while *ptr == 0 {
                            ptr = ptr.add(1);
                            current_index_ += BitVector::K_DATA_BITS as i32;
                            if ptr == end {
                                break;
                            }
                        }
                        if ptr < end {
                            current_index_ += base::bits::count_trailing_zeros(*ptr) as i32;
                        }
                    }
                }
                Iterator {
                    target_: target,
                    ptr_: ptr,
                    end_: end,
                    current_index_: current_index_,
                }
            }
            IteratorTag::End(end_tag) => Iterator {
                target_: target,
                ptr_: target.data_end_,
                end_: target.data_end_,
                current_index_: target.data_length() as i32 * BitVector::K_DATA_BITS as i32,
            },
        }
    }
}

enum IteratorTag {
    Start(IteratorStartTag),
    End(IteratorEndTag),
}

struct IteratorStartTag {}
struct IteratorEndTag {}

impl Iterator {
    pub fn increment(&mut self) {
        let bit_in_word = self.current_index_ & (BitVector::K_DATA_BITS as i32 - 1);
        if bit_in_word < BitVector::K_DATA_BITS as i32 - 1 {
            let remaining_bits;
            unsafe {
                remaining_bits = *self.ptr_ >> (bit_in_word + 1);
            }
            if remaining_bits != 0 {
                let next_bit_in_word = base::bits::count_trailing_zeros(remaining_bits) as i32;
                self.current_index_ += next_bit_in_word + 1;
                return;
            }
        }

        self.current_index_ = Utils::round_down(self.current_index_, BitVector::K_DATA_BITS as i32);
        loop {
            unsafe {
                self.ptr_ = self.ptr_.add(1);
            }
            self.current_index_ += BitVector::K_DATA_BITS as i32;
            if self.ptr_ == self.end_ {
                return;
            }
            unsafe {
                if *self.ptr_ != 0 {
                    break;
                }
            }
        }

        let trailing_zeros;
        unsafe {
            trailing_zeros = base::bits::count_trailing_zeros(*self.ptr_) as i32;
        }
        self.current_index_ += trailing_zeros;
    }

    pub fn deref(&self) -> i32 {
        DCHECK!(self.end_ != self.ptr_);
        unsafe {
            DCHECK!((&*self.target_).contains(self.current_index_));
        }
        self.current_index_
    }
}

impl PartialEq for Iterator {
    fn eq(&self, other: &Self) -> bool {
        DCHECK!(self.target_ == other.target_);
        DCHECK!(self.end_ == other.end_);
        if self.current_index_ == other.current_index_ {
            DCHECK!(self.ptr_ == other.ptr_);
        }
        self.current_index_ == other.current_index_
    }
}

impl GrowableBitVector {
    const K_INITIAL_LENGTH: i32 = 1024;
    const K_MAX_SUPPORTED_VALUE: i32 = (1 << 30) - 1;

    pub fn new() -> Self {
        GrowableBitVector { bits_: BitVector::new() }
    }

    pub fn with_length(length: i32, zone: &mut zone::Zone) -> Self {
        GrowableBitVector {
            bits_: BitVector::with_length(length, zone),
        }
    }

    pub fn contains(&self, value: i32) -> bool {
        if !self.in_bits_range(value) {
            return false;
        }
        self.bits_.contains(value)
    }

    pub fn add(&mut self, value: i32, zone: &mut zone::Zone) {
        if !self.in_bits_range(value) {
            self.grow(value, zone);
        }
        self.bits_.add(value);
    }

    pub fn is_empty(&self) -> bool {
        self.bits_.is_empty()
    }

    pub fn clear(&mut self) {
        self.bits_.clear();
    }

    pub fn length(&self) -> i32 {
        self.bits_.length()
    }

    pub fn equals(&self, other: &GrowableBitVector) -> bool {
        self.length() == other.length() && self.bits_.equals(&other.bits_)
    }

    pub fn begin(&self) -> Iterator {
        self.bits_.begin()
    }

    pub fn end(&self) -> Iterator {
        self.bits_.end()
    }

    fn in_bits_range(&self, value: i32) -> bool {
        self.bits_.length() > value
    }

    fn grow(&mut self, needed_value: i32, zone: &mut zone::Zone) {
        DCHECK!(!self.in_bits_range(needed_value));
        CHECK_GE!(Self::K_MAX_SUPPORTED_VALUE, needed_value);
        let new_length = std::cmp::max(
            Self::K_INITIAL_LENGTH,
            base::bits::round_up_to_power_of_two32((needed_value + 1) as u32) as i32,
        );
        self.bits_.resize(new_length, zone);
    }
}
