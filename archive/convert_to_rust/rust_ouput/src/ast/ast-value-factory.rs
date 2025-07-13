// Converted from V8 C++ source files:
// Header: ast-value-factory.h
// Implementation: ast-value-factory.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/ast/ast_value_factory.rs

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::hash::{BuildHasherDefault, Hasher};
use std::ops::Deref;
use std::rc::Rc;

use crate::init::bootstrapper::Isolate;
use crate::strings::string_hasher::StringHasher;
use crate::strings::uri::V8;
use crate::torque::torque_parser::ParseResult;
use crate::init::bootstrapper::BigInt;

pub struct AstValueFactory {}

pub struct AstRawString {
    next_: RefCell<Option<AstRawStringPtr>>,
    string_: RefCell<Option<IndirectHandle<String>>>,
    literal_bytes_: Vec<u8>,
    raw_hash_field_: u32,
    is_one_byte_: bool,
    has_string_: RefCell<bool>,
}

impl AstRawString {
    pub fn equal(lhs: &AstRawString, rhs: &AstRawString) -> bool {
        if lhs.raw_hash_field() != rhs.raw_hash_field() {
            return false;
        }

        if lhs.length() != rhs.length() {
            return false;
        }
        if lhs.length() == 0 {
            return true;
        }

        let l = &lhs.literal_bytes_;
        let r = &rhs.literal_bytes_;

        if lhs.is_one_byte() {
            if rhs.is_one_byte() {
                return l == r;
            } else {
                // Assuming UTF-16 for rhs, compare byte by byte
                if l.len() * 2 != r.len() {
                    return false;
                }
                for i in 0..l.len() {
                    if l[i] != r[i * 2] {
                        return false;
                    }
                }
                return true;
            }
        } else {
            if rhs.is_one_byte() {
                // Assuming UTF-16 for lhs, compare byte by byte
                if r.len() * 2 != l.len() {
                    return false;
                }
                for i in 0..r.len() {
                    if r[i] != l[i * 2] {
                        return false;
                    }
                }
                return true;
            } else {
                return l == r;
            }
        }
    }

    pub fn compare(lhs: &AstRawString, rhs: &AstRawString) -> i32 {
        if lhs as *const _ == rhs as *const _ {
            return 0;
        }

        let lhs_data = &lhs.literal_bytes_;
        let rhs_data = &rhs.literal_bytes_;
        let length = std::cmp::min(lhs.length(), rhs.length());

        if lhs.is_one_byte() {
            if rhs.is_one_byte() {
                for i in 0..length {
                    let cmp = lhs_data[i as usize].cmp(&rhs_data[i as usize]);
                    if cmp != std::cmp::Ordering::Equal {
                        return match cmp {
                            std::cmp::Ordering::Less => -1,
                            std::cmp::Ordering::Greater => 1,
                            std::cmp::Ordering::Equal => 0,
                        };
                    }
                }
            } else {
                let rhs_data_u16: Vec<u16> = rhs_data.chunks(2).map(|chunk| {
                    ((chunk[1] as u16) << 8) | (chunk[0] as u16)
                }).collect();
                for i in 0..length {
                    let cmp = (lhs_data[i as usize] as u16).cmp(&rhs_data_u16[i as usize]);
                    if cmp != std::cmp::Ordering::Equal {
                        return match cmp {
                            std::cmp::Ordering::Less => -1,
                            std::cmp::Ordering::Greater => 1,
                            std::cmp::Ordering::Equal => 0,
                        };
                    }
                }
            }
        } else {
            if rhs.is_one_byte() {
                let lhs_data_u16: Vec<u16> = lhs_data.chunks(2).map(|chunk| {
                    ((chunk[1] as u16) << 8) | (chunk[0] as u16)
                }).collect();
                for i in 0..length {
                    let cmp = lhs_data_u16[i as usize].cmp(&(rhs_data[i as usize] as u16));
                    if cmp != std::cmp::Ordering::Equal {
                        return match cmp {
                            std::cmp::Ordering::Less => -1,
                            std::cmp::Ordering::Greater => 1,
                            std::cmp::Ordering::Equal => 0,
                        };
                    }
                }
            } else {
                let lhs_data_u16: Vec<u16> = lhs_data.chunks(2).map(|chunk| {
                    ((chunk[1] as u16) << 8) | (chunk[0] as u16)
                }).collect();
                let rhs_data_u16: Vec<u16> = rhs_data.chunks(2).map(|chunk| {
                    ((chunk[1] as u16) << 8) | (chunk[0] as u16)
                }).collect();

                for i in 0..length {
                    let cmp = lhs_data_u16[i as usize].cmp(&rhs_data_u16[i as usize]);
                    if cmp != std::cmp::Ordering::Equal {
                        return match cmp {
                            std::cmp::Ordering::Less => -1,
                            std::cmp::Ordering::Greater => 1,
                            std::cmp::Ordering::Equal => 0,
                        };
                    }
                }
            }
        }

        (lhs.byte_length() as i32) - (rhs.byte_length() as i32)
    }

    pub fn is_empty(&self) -> bool {
        self.literal_bytes_.is_empty()
    }

    pub fn length(&self) -> usize {
        if self.is_one_byte() {
            self.literal_bytes_.len()
        } else {
            self.literal_bytes_.len() / 2
        }
    }

    pub fn as_array_index(&self, index: &mut u32) -> bool {
        if !self.is_integer_index() {
            return false;
        }

        if self.length() <= (Name::kMaxCachedArrayIndexLength as usize) {
            *index = Name::ArrayIndexValueBits::decode(self.raw_hash_field_);
            return true;
        }

        let mut stream = OneByteStringStream::new(&self.literal_bytes_);
        string_to_index(&mut stream, index)
    }

    pub fn is_integer_index(&self) -> bool {
        Name::is_integer_index(self.raw_hash_field_)
    }

    pub fn is_one_byte_equal_to(&self, data: &str) -> bool {
        if !self.is_one_byte() {
            return false;
        }

        if self.literal_bytes_.len() != data.len() {
            return false;
        }

        let data_bytes: Vec<u8> = data.bytes().collect();
        self.literal_bytes_ == data_bytes
    }

    pub fn first_character(&self) -> u16 {
        if self.is_one_byte() {
            self.literal_bytes_[0] as u16
        } else {
            let bytes = &self.literal_bytes_;
            ((bytes[1] as u16) << 8) | (bytes[0] as u16)
        }
    }

    pub fn internalize<T>(&self, isolate: &mut T)
        where
            T: FactoryAccess,
    {
        if *self.has_string_.borrow() {
            return;
        }

        if self.literal_bytes_.is_empty() {
            let empty_string = isolate.factory().empty_string();
            self.set_string(Some(empty_string));
        } else if self.is_one_byte() {
            let key = OneByteStringKey {
                hash: self.raw_hash_field_,
                data: self.literal_bytes_.clone(),
            };
            let internalized = isolate.factory().internalize_string_with_key(&key);
            self.set_string(Some(internalized));
        } else {
            let data: Vec<u16> = self.literal_bytes_.chunks(2).map(|chunk| {
                ((chunk[1] as u16) << 8) | (chunk[0] as u16)
            }).collect();
            let key = TwoByteStringKey {
                hash: self.raw_hash_field_,
                data,
            };

            let internalized = isolate.factory().internalize_string_with_key(&key);
            self.set_string(Some(internalized));
        }
    }

    pub fn is_one_byte(&self) -> bool {
        self.is_one_byte_
    }

    pub fn byte_length(&self) -> usize {
        self.literal_bytes_.len()
    }

    pub fn raw_data(&self) -> &[u8] {
        &self.literal_bytes_
    }

    pub fn is_private_name(&self) -> bool {
        self.length() > 0 && self.first_character() == '#' as u16
    }

    pub fn raw_hash_field(&self) -> u32 {
        self.raw_hash_field_
    }

    pub fn hash(&self) -> u32 {
        assert_eq!(self.raw_hash_field_ & Name::kHashNotComputedMask, 0);
        Name::HashBits::decode(self.raw_hash_field_)
    }

    pub fn string(&self) -> IndirectHandle<String> {
        assert!(*self.has_string_.borrow());
        self.string_.borrow().clone().unwrap()
    }

    fn new(is_one_byte: bool, literal_bytes: Vec<u8>, raw_hash_field: u32) -> AstRawString {
        AstRawString {
            next_: RefCell::new(None),
            string_: RefCell::new(None),
            literal_bytes_: literal_bytes,
            raw_hash_field_: raw_hash_field,
            is_one_byte_: is_one_byte,
            has_string_: RefCell::new(false),
        }
    }

    fn next(&self) -> Option<AstRawStringPtr> {
        assert!(!*self.has_string_.borrow());
        self.next_.borrow().clone()
    }

    fn next_location(&self) -> *mut Option<AstRawStringPtr> {
        assert!(!*self.has_string_.borrow());
        self.next_.as_ptr() as *mut Option<AstRawStringPtr>
    }

    fn set_string(&self, string: Option<IndirectHandle<String>>) {
        assert!(string.is_none() == false);
        assert!(!*self.has_string_.borrow());
        *self.string_.borrow_mut() = string;
        *self.has_string_.borrow_mut() = true;
    }

}

impl fmt::Debug for AstRawString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AstRawString {{ length: {}, is_one_byte: {} }}", self.length(), self.is_one_byte())
    }
}

struct OneByteStringStream<'a> {
    literal_bytes_: &'a [u8],
    pos_: usize,
}

impl<'a> OneByteStringStream<'a> {
    fn new(literal_bytes: &'a [u8]) -> Self {
        OneByteStringStream {
            literal_bytes_: literal_bytes,
            pos_: 0,
        }
    }

    fn has_more(&self) -> bool {
        self.pos_ < self.literal_bytes_.len()
    }

    fn get_next(&mut self) -> u16 {
        self.pos_ += 1;
        self.literal_bytes_[self.pos_ - 1] as u16
    }
}

fn string_to_index(stream: &mut OneByteStringStream, index: &mut u32) -> bool {
    let mut value: u64 = 0;
    let mut digits: i32 = 0;

    while stream.has_more() {
        let next = stream.get_next();
        if next < '0' as u16 || next > '9' as u16 {
            return false;
        }
        value = value * 10 + (next - '0' as u16) as u64;
        digits += 1;

        if value > u32::MAX as u64 {
            return false;
        }
    }
    if digits == 0 {
        return false;
    }
    *index = value as u32;
    true
}

struct AstConsString {
    string_: RefCell<Option<IndirectHandle<String>>>,
    segment_: RefCell<Segment>,
}

impl AstConsString {
    fn add_string(&self, zone: &Zone, s: &AstRawString) -> &Self {
        if s.is_empty() {
            return self;
        }

        let mut segment = self.segment_.borrow_mut();
        if !self.is_empty() {
            let tmp = zone.new_segment(segment.clone());
            segment.next = Some(Box::new(tmp));
        }
        segment.string = Some(Rc::new(s.clone()));
        self
    }

    fn is_empty(&self) -> bool {
        let segment = self.segment_.borrow();
        if segment.string.is_none() {
            assert!(segment.next.is_none());
        }
        segment.string.is_none()
    }

    fn get_string<T>(&self, isolate: &mut T) -> IndirectHandle<String>
        where
            T: FactoryAccess,
    {
        if self.string_.borrow().is_none() {
            let string = self.allocate(isolate);
            *self.string_.borrow_mut() = Some(string);
        }
        self.string_.borrow().clone().unwrap()
    }

    fn allocate_flat<T>(&self, isolate: &mut T) -> Handle<String>
        where
            T: FactoryAccess,
    {
        if self.is_empty() {
            return isolate.factory().empty_string();
        }
        let segment = self.segment_.borrow();
        if segment.next.is_none() {
            return segment.string.as_ref().unwrap().string();
        }

        let mut result_length = 0;
        let mut is_one_byte = true;
        let mut current = segment.clone();
        let mut current_segment = &current;
        while current_segment.string.is_some() {
            result_length += current_segment.string.as_ref().unwrap().length();
            is_one_byte = is_one_byte && current_segment.string.as_ref().unwrap().is_one_byte();
            if current_segment.next.is_some() {
                current_segment = current_segment.next.as_ref().unwrap().deref();
            } else {
                break;
            }
        }

        if is_one_byte {
            let result = isolate.factory().new_raw_one_byte_string(result_length, AllocationType::kOld);
            let mut dest = result.get_chars() + result_length;
            let mut current = segment.clone();
            let mut current_segment = &current;
            while current_segment.string.is_some() {
                let length = current_segment.string.as_ref().unwrap().length();
                dest -= length;
                copy_chars_to_vec(
                    &mut result.data_mut(),
                    dest,
                    current_segment.string.as_ref().unwrap().raw_data(),
                    length,
                );
                if current_segment.next.is_some() {
                    current_segment = current_segment.next.as_ref().unwrap().deref();
                } else {
                    break;
                }
            }
            return result;
        }

        let result = isolate.factory().new_raw_two_byte_string(result_length, AllocationType::kOld);

        let mut dest = result.get_chars() + result_length;
        let mut current = segment.clone();
        let mut current_segment = &current;
        while current_segment.string.is_some() {
            let length = current_segment.string.as_ref().unwrap().length();
            dest -= length;
            if current_segment.string.as_ref().unwrap().is_one_byte() {
                copy_chars_to_vec(
                    &mut result.data_mut(),
                    dest,
                    current_segment.string.as_ref().unwrap().raw_data(),
                    length,
                );
            } else {
                copy_chars_to_vec(
                    &mut result.data_mut(),
                    dest,
                    current_segment.string.as_ref().unwrap().raw_data(),
                    length,
                );
            }
            if current_segment.next.is_some() {
                current_segment = current_segment.next.as_ref().unwrap().deref();
            } else {
                break;
            }
        }

        return result;
    }

    fn to_raw_strings(&self) -> Vec<Rc<AstRawString>> {
        let mut result = Vec::new();
        if self.is_empty() {
            return result;
        }
        let segment = self.segment_.borrow();
        result.push(segment.string.clone().unwrap());
        let mut current = segment.next.clone();
        while let Some(next) = current {
            result.push(next.string.clone().unwrap());
            current = next.next.clone();
        }
        result
    }

    fn last(&self) -> Option<Rc<AstRawString>> {
        self.segment_.borrow().string.clone()
    }

    fn new() -> AstConsString {
        AstConsString {
            string_: RefCell::new(None),
            segment_: RefCell::new(Segment {
                string: None,
                next: None,
            }),
        }
    }

    fn allocate<T>(&self, isolate: &mut T) -> Handle<String>
        where
            T: FactoryAccess,
    {
        if self.is_empty() {
            return isolate.factory().empty_string();
        }

        let segment = self.segment_.borrow();
        let mut tmp = segment.string.as_ref().unwrap().string();
        let mut current = segment.next.clone();
        while let Some(next) = current {
            tmp = isolate.factory().new_cons_string(next.string.as_ref().unwrap().string(), tmp, AllocationType::kOld);
            current = next.next.clone();
        }
        tmp
    }
}

#[derive(Clone)]
struct Segment {
    string: Option<Rc<AstRawString>>,
    next: Option<Box<Segment>>,
}

#[derive(Clone)]
struct AstRawStringInternalizationKey {
    hash: u32,
    data: Vec<u8>,
    is_one_byte: bool,
}

struct AstRawStringMapMatcher {}

impl AstRawStringMapMatcher {
    fn call(&self, hash1: u32, hash2: u32, lookup_key: &AstRawString, entry_key: &AstRawString) -> bool {
        hash1 == hash2 && AstRawString::equal(lookup_key, entry_key)
    }
}

struct AstBigInt {
    bigint_: String,
}

impl AstBigInt {
    fn new(bigint: &str) -> AstBigInt {
        AstBigInt {
            bigint_: bigint.to_string(),
        }
    }

    fn c_str(&self) -> &str {
        &self.bigint_
    }
}

struct AstStringConstants {
    zone_: Zone,
    string_table_: AstRawStringMap,
    hash_seed_: u64,
    ascii_nul_string_: AstRawStringPtr,
    a_string_: AstRawStringPtr,
    b_string_: AstRawStringPtr,
    c_string_: AstRawStringPtr,
    d_string_: AstRawStringPtr,
    e_string_: AstRawStringPtr,
    f_string_: AstRawStringPtr,
    g_string_: AstRawStringPtr,
    h_string_: AstRawStringPtr,
    i_string_: AstRawStringPtr,
    j_string_: AstRawStringPtr,
    k_string_: AstRawStringPtr,
    l_string_: AstRawStringPtr,
    m_string_: AstRawStringPtr,
    n_string_: AstRawStringPtr,
    o_string_: AstRawStringPtr,
    p_string_: AstRawStringPtr,
    q_string_: AstRawStringPtr,
    r_string_: AstRawStringPtr,
    s_string_: AstRawStringPtr,
    t_string_: AstRawStringPtr,
    u_string_: AstRawStringPtr,
    v_string_: AstRawStringPtr,
    w_string_: AstRawStringPtr,
    x_string_: AstRawStringPtr,
    y_string_: AstRawStringPtr,
    z_string_: AstRawStringPtr,
    anonymous_string_: AstRawStringPtr,
    arguments_string_: AstRawStringPtr,
    as_string_: AstRawStringPtr,
    assert_string_: AstRawStringPtr,
    async_string_: AstRawStringPtr,
    bigint_string_: AstRawStringPtr,
    boolean_string_: AstRawStringPtr,
    computed_string_: AstRawStringPtr,
    dot_brand_string_: AstRawStringPtr,
    constructor_string_: AstRawStringPtr,
    default_string_: AstRawStringPtr,
    done_string_: AstRawStringPtr,
    dot_default_string_: AstRawStringPtr,
    dot_for_string_: AstRawStringPtr,
    dot_generator_object_string_: AstRawStringPtr,
    dot_home_object_string_: AstRawStringPtr,
    dot_result_string_: AstRawStringPtr,
    dot_repl_result_string_: AstRawStringPtr,
    dot_static_home_object_string_: AstRawStringPtr,
    dot_switch_tag_string_: AstRawStringPtr,
    dot_catch_string_: AstRawStringPtr,
    empty_string_: AstRawStringPtr,
    eval_string_: AstRawStringPtr,
    from_string_: AstRawStringPtr,
    function_string_: AstRawStringPtr,
    get_space_string_: AstRawStringPtr,
    length_string_: AstRawStringPtr,
    let_string_: AstRawStringPtr,
    meta_string_: AstRawStringPtr,
    native_string_: AstRawStringPtr,
    new_target_string_: AstRawStringPtr,
    next_string_: AstRawStringPtr,
    number_string_: AstRawStringPtr,
    object_string_: AstRawStringPtr,
    private_constructor_string_: AstRawStringPtr,
    proto_string_: AstRawStringPtr,
    prototype_string_: AstRawStringPtr,
    return_string_: AstRawStringPtr,
    set_space_string_: AstRawStringPtr,
    source_string_: AstRawStringPtr,
    string_string_: AstRawStringPtr,
    symbol_string_: AstRawStringPtr,
    target_string_: AstRawStringPtr,
    this_string_: AstRawStringPtr,
    this_function_string_: AstRawStringPtr,
    throw_string_: AstRawStringPtr,
    undefined_string_: AstRawStringPtr,
    value_string_: AstRawStringPtr,
}

impl AstStringConstants {
    const kMaxOneCharStringValue: i32 = 122;

    fn new(isolate: &mut Isolate, hash_seed: u64) -> AstStringConstants {
        assert_eq!(0, 0);

        let mut string_table_: AstRawStringMap = AstRawStringMap::default();
        let zone_ = Zone::new(isolate.allocator());

        let mut constants = AstStringConstants {
            zone_: zone_,
            string_table_: string_table_,
            hash_seed_: hash_seed,
            ascii_nul_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            a_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            b_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            c_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            d_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            e_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            f_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            g_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            h_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            i_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            j_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            k_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            l_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            m_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            n_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            o_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            p_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            q_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            r_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            s_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            t_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            u_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            v_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            w_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            x_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            y_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            z_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            anonymous_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            arguments_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            as_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            assert_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            async_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            bigint_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            boolean_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            computed_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            dot_brand_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            constructor_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            default_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            done_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            dot_default_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            dot_for_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            dot_generator_object_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            dot_home_object_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            dot_result_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            dot_repl_result_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            dot_static_home_object_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            dot_switch_tag_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            dot_catch_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            empty_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            eval_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            from_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            function_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            get_space_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            length_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            let_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            meta_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            native_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            new_target_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            next_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            number_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            object_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            private_constructor_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            proto_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            prototype_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            return_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            set_space_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            source_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            string_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            symbol_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            target_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            this_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            this_function_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            throw_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            undefined_string_: Rc::new(AstRawString::new(false, vec![], 0)),
            value_string_: Rc::new(AstRawString::new(false, vec![], 0)),
        };

        macro_rules! define_string_constant {
            ($name:ident, $str:literal) => {
                let data: &[u8] = $str.as_bytes();
                let literal: Vec<u8> = data.to_vec();
                let handle = isolate.factory().name();
                let raw_hash_field = handle.raw_hash_field();
                assert_eq!(
                    raw_hash_field,
                    StringHasher::hash_sequential_string::<u8>(
                        literal.as_slice(),
                        literal.len(),
                        constants.hash_seed_,
                    )
                );
                assert_eq!(literal.len(), handle.length());

                let ast_raw_string = Rc::new(AstRawString::new(true, literal.clone(), raw_hash_field));

                let _ = constants.string_table_.insert(
                    Rc::clone(&ast_raw_string),
                    ast_raw_string.hash(),
                );
                constants.$name = ast_raw_string;

            };
        }

        macro_rules! generate_ascii_constants {
            ($F:ident) => {
                $F!(ascii_nul_string, "\0");
                $F!(a_string, "a");
                $F!(b_string, "b");
                $F!(c_string, "c");
                $F!(d_string, "d");
                $F!(e_string, "e");
                $F!(f_string, "f");
                $F!(g_string, "g");
                $F!(h_string, "h");
                
