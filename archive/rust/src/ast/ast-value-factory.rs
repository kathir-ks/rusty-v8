// Copyright 2014 the V8 project authors. All rights reserved.
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
//       notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
//       copyright notice, this list of conditions and the following
//       disclaimer in the documentation and/or other materials provided
//       with the distribution.
//     * Neither the name of Google Inc. nor the names of its
//       contributors may be used to endorse or promote products derived
//       from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// src/ast/ast-value-factory.h equivalent
pub mod ast_value_factory {
    use std::fmt;
    use std::cmp::Ordering;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::ptr;
    use std::sync::atomic::{AtomicPtr, Ordering as AtomicOrdering};

    // Placeholder types for V8 specific classes.  Need proper definitions.
    pub struct Isolate {
        factory: Factory,
        thread_id: usize,
    }

    pub struct LocalIsolate {
        factory: Factory,
    }

    pub struct Factory {

    }

    impl Factory {
        pub fn empty_string(&self) -> StringHandle {
            StringHandle::new()
        }

        pub fn InternalizeStringWithKey(&self, key: &OneByteStringKey) -> StringHandle {
            StringHandle::new()
        }

        pub fn InternalizeStringWithKey(&self, key: &TwoByteStringKey) -> StringHandle {
            StringHandle::new()
        }

        pub fn name(&self) -> StringHandle {
            StringHandle::new()
        }

        pub fn NewConsString(&self, str1: StringHandle, str2: StringHandle, allocation_type: AllocationType) -> Result<StringHandle, String> {
            Ok(StringHandle::new())
        }

        pub fn NewRawOneByteString(&self, length: usize, allocation_type: AllocationType) -> Result<StringHandle, String> {
            Ok(StringHandle::new())
        }

        pub fn NewRawTwoByteString(&self, length: usize, allocation_type: AllocationType) -> Result<StringHandle, String> {
            Ok(StringHandle::new())
        }

        pub fn one_character_string(&self, key: u8) -> StringHandle {
            StringHandle::new()
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum AllocationType {
        kOld,
    }

    pub struct StringHandle {
        // Placeholder, replace with proper String type
    }

    impl StringHandle {
        pub fn new() -> Self {
            StringHandle {}
        }

        pub fn raw_hash_field(&self) -> u32 {
            0
        }

        pub fn length(&self) -> usize {
            0
        }

        pub fn GetFlatContent(&self, no_gc: DisallowGarbageCollection, access_guard: SharedStringAccessGuardIfNeeded) -> FlatContent {
            FlatContent::new()
        }

        pub fn is_null(&self) -> bool {
            false
        }

        pub fn is_empty(&self) -> bool {
            false
        }

        pub fn ToHandleChecked(&self) -> StringHandle {
            StringHandle::new()
        }

        pub fn raw_data(&self) -> *const u8 {
            ptr::null()
        }

        pub fn is_one_byte(&self) -> bool {
            false
        }
    }

    pub struct DisallowGarbageCollection {}

    pub struct SharedStringAccessGuardIfNeeded {}

    impl SharedStringAccessGuardIfNeeded {
        pub fn NotNeeded() -> Self {
            SharedStringAccessGuardIfNeeded {}
        }
    }

    pub struct FlatContent {
        // Placeholder, replace with proper FlatContent type
    }

    impl FlatContent {
        pub fn new() -> Self {
            FlatContent {}
        }

        pub fn IsOneByte(&self) -> bool {
            false
        }

        pub fn IsTwoByte(&self) -> bool {
            false
        }

        pub fn ToOneByteVector(&self) -> Vec<u8> {
            Vec::new()
        }

        pub fn ToUC16Vector(&self) -> Vec<u16> {
            Vec::new()
        }
    }

    impl Isolate {
        pub fn new(factory: Factory, thread_id: usize) -> Self {
            Isolate {
                factory,
                thread_id,
            }
        }
        pub fn factory(&mut self) -> &mut Factory {
            &mut self.factory
        }

        pub fn thread_id(&self) -> usize {
            self.thread_id
        }
    }

    impl LocalIsolate {
        pub fn new(factory: Factory) -> Self {
            LocalIsolate {
                factory,
            }
        }
        pub fn factory(&mut self) -> &mut Factory {
            &mut self.factory
        }
    }

    // base/hashmap-entry.h:  No direct translation needed; Rust's HashMap handles this.
    // base/logging.h: Use Rust's standard logging facilities (log crate).
    // common/globals.h:  Configuration constants. Use `const` in Rust.
    // heap/factory-inl.h: Factory pattern already handled above with Factory struct.
    // heap/local-factory-inl.h: Factory pattern already handled above with Factory struct.
    // objects/string-inl.h: Handled with StringHandle and FlatContent above.
    // roots/roots.h:  No direct translation needed (internal data).
    // strings/string-hasher.h: Translated with StringHasher struct and impl.
    // utils/utils-inl.h: General utilities, use Rust's standard library or crates.

    mod string_hasher {
        use std::hash::{Hasher, BuildHasher};
        use std::collections::hash_map::DefaultHasher;

        pub struct StringHasher {}

        impl StringHasher {
            pub fn HashSequentialString<T: std::convert::TryInto<u32> + Copy>(data: &[T], length: usize, seed: u64) -> u32
            where <T as TryInto<u32>>::Error: std::fmt::Debug,
            {
                let mut hasher = DefaultHasher::new();
                hasher.write_u64(seed);

                for i in 0..length {
                    let val: u32 = data[i].try_into().unwrap();
                    hasher.write_u32(val);
                }

                hasher.finish() as u32
            }
        }
    }

    use string_hasher::StringHasher;

    mod utils {
        pub fn CompareCharsEqualUnsigned<T: PartialEq>(a: *const T, b: *const T, length: usize) -> bool {
            if a.is_null() || b.is_null() {
                return false;
            }

            unsafe {
                for i in 0..length {
                    if *a.add(i) != *b.add(i) {
                        return false;
                    }
                }
            }

            true
        }

        pub fn CompareCharsUnsigned<T: PartialOrd>(a: *const T, b: *const T, length: usize) -> i32 {
            if a.is_null() || b.is_null() {
                return 0;
            }

            unsafe {
                for i in 0..length {
                    let a_val = *a.add(i);
                    let b_val = *b.add(i);

                    if a_val < b_val {
                        return -1;
                    } else if a_val > b_val {
                        return 1;
                    }
                }
            }

            return 0;
        }

        pub fn CopyChars<T: Copy>(dest: *mut T, src: *const T, length: usize) {
            if dest.is_null() || src.is_null() {
                return;
            }

            unsafe {
                for i in 0..length {
                    *dest.add(i) = *src.add(i);
                }
            }
        }
    }
    use utils::*;

    mod name {
        pub const kMaxCachedArrayIndexLength: usize = 3;

        pub struct ArrayIndexValueBits {}

        impl ArrayIndexValueBits {
            pub fn decode(raw_hash_field: u32) -> u32 {
                raw_hash_field // Placeholder.  Needs proper decoding.
            }
        }

        pub fn IsIntegerIndex(raw_hash_field: u32) -> bool {
            true // Placeholder. Needs proper logic.
        }
    }

    use name::*;

    mod base {
        pub struct Vector<'a, T> {
            data: &'a [T],
        }

        impl<'a, T> Vector<'a, T> {
            pub fn new(data: &'a [T]) -> Self {
                Vector { data }
            }

            pub fn begin(&self) -> *const T {
                self.data.as_ptr()
            }

            pub fn length(&self) -> usize {
                self.data.len()
            }

            pub fn is_empty(&self) -> bool {
                self.data.is_empty()
            }
        }

        impl<'a> Vector<'a, u8> {
            pub fn cast(self) -> Vector<'a, u16> {
                //This cast is not safe in general, it is only valid in v8's context
                //when the u8 vector represents UTF-16 data.  This is because rust requires
                //that types are properly aligned
                assert!(self.length() % 2 == 0);
                unsafe {
                    let ptr = self.begin() as *const u16;
                    let len = self.length() / 2;
                    Vector::<'a, u16>::new(std::slice::from_raw_parts(ptr, len))
                }
            }
        }

        impl<'a> Vector<'a, u16> {
            pub fn cast(self) -> Vector<'a, u8> {
               //This cast is not safe in general, it is only valid in v8's context
               //when the u8 vector represents UTF-16 data.  This is because rust requires
               //that types are properly aligned
                unsafe {
                    let ptr = self.begin() as *const u8;
                    let len = self.length() * 2;
                    Vector::<'a, u8>::new(std::slice::from_raw_parts(ptr, len))
                }
            }
        }

        // Placeholder for HashMap::Entry
        pub struct HashMapEntry {}
    }

    use base::Vector;

    mod zone {
        use std::alloc::{alloc, dealloc, Layout};
        use std::ptr::NonNull;
        use std::marker::PhantomData;

        pub struct Zone {
            // Replace with a proper allocator implementation.
            name: String
        }

        impl Zone {
            pub fn new(name: String) -> Self {
                Zone {
                    name: name
                }
            }

            pub fn New<T>(&self) -> Box<T> {
                // Replace with zone allocation logic
                Box::new(unsafe { std::mem::zeroed() })
            }

             pub fn AllocateArray<T>(&self, count: usize) -> *mut T {
                let layout = Layout::array::<T>(count).unwrap();
                unsafe {
                    let ptr = alloc(layout) as *mut T;
                    if ptr.is_null() {
                        panic!("Allocation failed!");
                    }
                    ptr
                }
            }
        }
    }
    use zone::*;

    // For using StringToIndex.
    mod one_byte_string_stream {
        pub struct OneByteStringStream<'a> {
            literal_bytes_: super::base::Vector<'a, u8>,
            pos_: usize,
        }

        impl<'a> OneByteStringStream<'a> {
            pub fn new(literal_bytes_: super::base::Vector<'a, u8>) -> Self {
                OneByteStringStream {
                    literal_bytes_: literal_bytes_,
                    pos_: 0,
                }
            }

            pub fn HasMore(&self) -> bool {
                self.pos_ < self.literal_bytes_.length()
            }

            pub fn GetNext(&mut self) -> u16 {
                let result = self.literal_bytes_.data[self.pos_] as u16;
                self.pos_ += 1;
                result
            }
        }
    }
    use one_byte_string_stream::OneByteStringStream;

    // StringToIndex function, since it is not included, it's replaced with a placeholder
    fn StringToIndex(stream: &mut OneByteStringStream, index: &mut u32) -> bool {
        *index = 0;
        true
    }

    // AstRawString
    pub struct AstRawString {
        is_one_byte_: bool,
        literal_bytes_: Vector<'static, u8>,
        raw_hash_field_: u32,
        string_: AtomicPtr<StringHandle>,
        next_: AtomicPtr<AstRawString>,
    }

    unsafe impl Send for AstRawString {}
    unsafe impl Sync for AstRawString {}

    impl AstRawString {
        pub fn new(is_one_byte: bool, literal_bytes: Vector<'static, u8>, raw_hash_field: u32) -> Self {
            AstRawString {
                is_one_byte_: is_one_byte,
                literal_bytes_: literal_bytes,
                raw_hash_field_: raw_hash_field,
                string_: AtomicPtr::new(ptr::null_mut()),
                next_: AtomicPtr::new(ptr::null_mut()),
            }
        }

        pub fn Hash(&self) -> u32 {
            self.raw_hash_field_
        }

        pub fn length(&self) -> usize {
            self.literal_bytes_.length()
        }

        pub fn byte_length(&self) -> usize {
            self.literal_bytes_.length()
        }

        pub fn raw_data(&self) -> *const u8 {
            self.literal_bytes_.begin()
        }

        pub fn is_one_byte(&self) -> bool {
            self.is_one_byte_
        }

        pub fn literal_bytes(&self) -> &Vector<'static, u8> {
            &self.literal_bytes_
        }

        pub fn string(&self) -> StringHandle {
            let ptr = self.string_.load(AtomicOrdering::Relaxed);
            if ptr.is_null() {
                StringHandle::new() // Return a default value if not internalized yet
            } else {
                unsafe { ptr::read(ptr) }
            }
        }

        pub fn set_string(&self, string: StringHandle) {
            let boxed = Box::new(string);
            let ptr = Box::into_raw(boxed);
            self.string_.store(ptr, AtomicOrdering::Relaxed);
        }

        pub fn next(&self) -> *mut AstRawString {
            self.next_.load(AtomicOrdering::Relaxed)
        }

        pub fn set_next(&self, next: *mut AstRawString) {
            self.next_.store(next, AtomicOrdering::Relaxed);
        }

        pub fn Internalize<IsolateT>(&self, isolate: &mut IsolateT)
            where IsolateT: AstRawStringInternalizer
        {
            if self.string_.load(AtomicOrdering::Relaxed).is_null() {
                if self.literal_bytes_.is_empty() {
                    self.set_string(isolate.factory().empty_string());
                } else if self.is_one_byte() {
                    let key = OneByteStringKey::new(self.raw_hash_field_, self.literal_bytes_.data);
                    self.set_string(isolate.factory().InternalizeStringWithKey(&key));
                } else {
                    let literal_bytes_u16 = Vector::<u8>::cast(Vector::<'static, u8>::new(self.literal_bytes_.data));
                    let key = TwoByteStringKey::new(self.raw_hash_field_, literal_bytes_u16.data);
                    self.set_string(isolate.factory().InternalizeStringWithKey(&key));
                }
            }
        }

        pub fn AsArrayIndex(&self, index: &mut u32) -> bool {
            if !self.IsIntegerIndex() {
                return false;
            }
            if self.length() <= kMaxCachedArrayIndexLength {
                *index = ArrayIndexValueBits::decode(self.raw_hash_field_);
                return true;
            }

            let mut stream = OneByteStringStream::new(self.literal_bytes_);
            StringToIndex(&mut stream, index)
        }

        pub fn IsIntegerIndex(&self) -> bool {
            Name::IsIntegerIndex(self.raw_hash_field_)
        }

        pub fn IsOneByteEqualTo(&self, data: &str) -> bool {
            if !self.is_one_byte() {
                return false;
            }

            let length = self.literal_bytes_.length();
            if length != data.len() {
                return false;
            }

            let data_bytes = data.as_bytes();
            unsafe {
                0 == libc::strncmp(self.literal_bytes_.begin() as *const i8,
                                    data_bytes.as_ptr() as *const i8,
                                    length)
            }
        }

        pub fn FirstCharacter(&self) -> u16 {
            if self.is_one_byte() {
                self.literal_bytes_.data[0] as u16
            } else {
                let c = self.literal_bytes_.begin() as *const u16;
                unsafe { *c }
            }
        }

        pub fn Equal(lhs: &AstRawString, rhs: &AstRawString) -> bool {
            if lhs.Hash() != rhs.Hash() {
                return false;
            }

            if lhs.length() != rhs.length() {
                return false;
            }

            if lhs.length() == 0 {
                return true;
            }

            let l = lhs.raw_data();
            let r = rhs.raw_data();
            let length = rhs.length();

            if lhs.is_one_byte() {
                if rhs.is_one_byte() {
                    CompareCharsEqualUnsigned(l, r, length)
                } else {
                    let r_u16 = Vector::<u8>::cast(Vector::<'static, u8>::new(rhs.literal_bytes_.data)).begin();
                    CompareCharsEqualUnsigned(l, r_u16, length)
                }
            } else {
                let l_u16 = Vector::<u8>::cast(Vector::<'static, u8>::new(lhs.literal_bytes_.data)).begin();
                if rhs.is_one_byte() {
                    CompareCharsEqualUnsigned(l_u16, r, length)
                } else {
                    let r_u16 = Vector::<u8>::cast(Vector::<'static, u8>::new(rhs.literal_bytes_.data)).begin();
                    CompareCharsEqualUnsigned(l_u16, r_u16, length)
                }
            }
        }

        pub fn Compare(lhs: &AstRawString, rhs: &AstRawString) -> i32 {
            // Fast path for equal pointers.
            if lhs as *const _ == rhs as *const _ {
                return 0;
            }

            let lhs_data = lhs.raw_data();
            let rhs_data = rhs.raw_data();
            let length = std::cmp::min(lhs.length(), rhs.length());

            // Code point order by contents.
            if lhs.is_one_byte() {
                if rhs.is_one_byte() {
                    let result = CompareCharsUnsigned(lhs_data, rhs_data, length);
                    if result != 0 {
                        return result;
                    }
                } else {
                    let rhs_data_u16 = Vector::<u8>::cast(Vector::<'static, u8>::new(rhs.literal_bytes_.data)).begin();
                    let result = CompareCharsUnsigned(lhs_data, rhs_data_u16, length);
                    if result != 0 {
                        return result;
                    }
                }
            } else {
                let lhs_data_u16 = Vector::<u8>::cast(Vector::<'static, u8>::new(lhs.literal_bytes_.data)).begin();
                if rhs.is_one_byte() {
                   let result = CompareCharsUnsigned(lhs_data_u16, rhs_data, length);
                    if result != 0 {
                        return result;
                    }
                } else {
                    let rhs_data_u16 = Vector::<u8>::cast(Vector::<'static, u8>::new(rhs.literal_bytes_.data)).begin();
                    let result = CompareCharsUnsigned(lhs_data_u16, rhs_data_u16, length);
                    if result != 0 {
                        return result;
                    }
                }
            }

            (lhs.byte_length() as i32) - (rhs.byte_length() as i32)
        }
    }

    impl fmt::Debug for AstRawString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AstRawString: {} bytes", self.byte_length())
        }
    }

    // AstConsString
    pub struct AstConsString {
        segment_: Segment,
        string_: AtomicPtr<StringHandle>, //std::sync::atomic::AtomicPtr<String>,
    }

    unsafe impl Send for AstConsString {}
    unsafe impl Sync for AstConsString {}

    impl AstConsString {
        pub fn new() -> Self {
            AstConsString {
                segment_: Segment {
                    string: ptr::null(),
                    next: ptr::null_mut(),
                },
                string_: AtomicPtr::new(ptr::null_mut()),
            }
        }

        pub fn IsEmpty(&self) -> bool {
            self.segment_.string.is_null()
        }

        pub fn AddString<'a>(&mut self, zone: &Zone, str: *const AstRawString) -> &mut Self {
            if self.segment_.string.is_null() {
                self.segment_.string = str;
            } else {
                let new_segment = zone.New::<Segment>();
                unsafe {
                   (*new_segment).string = str;
                   (*new_segment).next = self.segment_.next;
                   self.segment_.next = new_segment;
                }
            }
            self
        }

        pub fn Allocate<IsolateT>(&self, isolate: &mut IsolateT) -> StringHandle
            where IsolateT: AstConsStringAllocator
        {
            if self.IsEmpty() {
                return isolate.factory().empty_string();
            }

            let mut tmp = unsafe { (*self.segment_.string).string() };
            let mut current = self.segment_.next;
            while !current.is_null() {
                let current_segment = unsafe { &*current };
                let str_handle = unsafe { (*current_segment.string).string() };
                tmp = isolate.factory().NewConsString(str_handle, tmp, AllocationType::kOld).unwrap();
                current = current_segment.next;
            }
            tmp
        }

        pub fn AllocateFlat<IsolateT>(&self, isolate: &mut IsolateT) -> StringHandle
            where IsolateT: AstConsStringAllocator
        {
            if self.IsEmpty() {
                return isolate.factory().empty_string();
            }

            if self.segment_.next.is_null() {
                return unsafe { (*self.segment_.string).string() };
            }

            let mut result_length = 0;
            let mut is_one_byte = true;
            let mut current = &self.segment_;

            while !current.string.is_null() {
                let raw_string = unsafe { &*current.string };
                result_length += raw_string.length();
                is_one_byte = is_one_byte && raw_string.is_one_byte();
                if current.next.is_null() {
                    break;
                }
                current = unsafe { &*current.next };
            }

            if is_one_byte {
                let result = isolate.factory().NewRawOneByteString(result_length, AllocationType::kOld).unwrap();
                //DisallowGarbageCollection no_gc; //Placeholder, no equivalent needed in Rust
                let mut dest = Vec::new(); //result.GetChars(no_gc, SharedStringAccessGuardIfNeeded::NotNeeded()) + result_length;
                 for _ in 0..result_length {
                    dest.push(0);
                 }
                for current in self.ToRawStrings() {
                    let length = current.length();
                    let raw_data = current.raw_data();
                    for i in 0..length {
                        dest[result_length - length + i] = unsafe { *raw_data.add(i) };
                    }
                    //CopyChars(dest, current->string->raw_data(), length);
                 }
               return result;
            } else {
                 let result = isolate.factory().NewRawTwoByteString(result_length, AllocationType::kOld).unwrap();
                 //DisallowGarbageCollection no_gc; //Placeholder, no equivalent needed in Rust
                 let mut dest = Vec::new(); //result.GetChars(no_gc, SharedStringAccessGuardIfNeeded::NotNeeded()) + result_length;
                for _ in 0..result_length {
                    dest.push(0u16);
                }
                 for current in self.ToRawStrings() {
                    let length = current.length();
                    let raw_data = current.raw_data();
                     if unsafe { (*current).is_one_byte() } {
                        for i in 0..length {
                            dest[result_length - length + i] = unsafe { *raw_data.add(i) } as u16;
                        }
                    } else {
                        let raw_data_u16 = Vector::<u8>::cast(Vector::<'static, u8>::new(unsafe { (*current).literal_bytes() }.data)).begin();
                        for i in 0..length {
                            dest[result_length - length + i] = unsafe { *raw_data_u16.add(i) };
                        }
                    }

                 }

                return result;
            }
        }

        pub fn ToRawStrings(&self) -> Vec<*const AstRawString> {
            let mut result = Vec::new();
            if self.IsEmpty() {
                return result;
            }

            result.push(self.segment_.string);

            let mut current = self.segment_.next;
            while !current.is_null() {
                let segment = unsafe { &*current };
                result.push(segment.string);
                current = segment.next;
            }
            result
        }
    }

    impl Drop for AstConsString {
        fn drop(&mut self) {
            // let ptr = self.string_.load(AtomicOrdering::Relaxed);
            // if !ptr.is_null() {
            //     unsafe {
            //         drop(Box::from_raw(ptr));
            //     }
            // }
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct Segment {
        string: *const AstRawString, // *const AstRawString
        next: *mut Segment,
    }

    // AstStringConstants
    pub struct AstStringConstants {
        zone_: Zone,
        string_table_: AstRawStringMap,
        hash_seed_: u64,
        empty_string_: *const AstRawString,
        space_string_: *const AstRawString,
        one_char_strings_: [*const AstRawString; 256], // Assuming u8 range
        dot_string_: *const AstRawString,
        this_string_: *const AstRawString,
        proto_string_: *const AstRawString,
        constructor_string_: *const AstRawString,
        prototype_string_: *const AstRawString,
        length_string_: *const AstRawString,
        name_string_: *const AstRawString,
        message_string_: *const AstRawString,
        arguments_string_: *const AstRawString,
        caller_string_: *const AstRawString,
        eval_string_: *const AstRawString,
        valueOf_string_: *const AstRawString,
        toString_string_: *const AstRawString,
        instanceOf_string_: *const AstRawString,
        apply_string_: *const AstRawString,
        call_string_: *const AstRawString,
        unscopables_string_: *const AstRawString,
        iterator_string_: *const AstRawString,
        asyncIterator_string_: *const AstRawString,
        hasInstance_string_: *const AstRawString,
        isConcatSpreadable_string_: *const AstRawString,
        match_string_: *const AstRawString,
        replace_string_: *const AstRawString,
        search_string_: *const AstRawString,
        species_string_: *const AstRawString,
        split_string_: *const AstRawString,
        toPrimitive_string_: *const AstRawString,
        toStringTag_string_: *const AstRawString,
        unShift_string_: *const AstRawString,
        async_string_: *const AstRawString,
        await_string_: *const AstRawString,
        default_string_: *const AstRawString,
        get_string_: *const AstRawString,
        set_string_: *const AstRawString,
        target_string_: *const AstRawString,
        then_string_: *const AstRawString,
        finally_string_: *const AstRawString,
    }

    impl AstStringConstants {
        pub fn new(isolate: &mut Isolate, hash_seed: u64) -> Self {
            let zone_ = Zone::new("AstStringConstantsZone".to_string());
            let mut string_table_ = AstRawStringMap::new();

            let mut one_char_strings_: [*const AstRawString; 256] = [ptr::null(); 256];

            let mut constants = AstStringConstants {
                zone_: zone_,
                string_table_: string_table_,
                hash_seed_: hash_seed,
                empty_string_: ptr::null(),
                space_string_: ptr::null(),
                one_char_strings_: one_char_strings_,
                dot_string_: ptr::null(),
                this_string_: ptr::null(),
                proto_string_: ptr::null(),
                constructor_string_: ptr::null(),
                prototype_string_: ptr::null(),
                length_string_: ptr::null(),
                name_string_: ptr::null(),
                message_string_: ptr::null(),
                arguments_string_: ptr::null(),
                caller_string_: ptr::null(),
                eval_string_: ptr::null(),
                valueOf_string_: ptr::null(),
                toString_string_: ptr::null(),
                instanceOf_string_: ptr::null(),
                apply_string_: ptr::null(),
                call_string_: ptr::null(),
                unscopables_string_: ptr::null(),
                iterator_string_: ptr::null(),
                asyncIterator_string_: ptr::null(),
                hasInstance_string_: ptr::null(),
                isConcatSpreadable_string_: ptr::null(),
                match_string_: ptr::null(),
                replace_string_: ptr::null(),
                search_string_: ptr::null(),
                species_string_: ptr::null(),
                split_string_: ptr::null(),
                toPrimitive_string_: ptr::null(),
                toStringTag_string_: ptr::null(),
                unShift_string_: ptr::null(),
                async_string_: ptr::null(),
                await_string_: ptr::null(),
                default_string_: ptr::null(),
                get_string_: ptr::null(),
                set_string_: ptr::null(),
                target_string_: ptr::null(),
                then_string_: ptr::null(),
                finally_string_: ptr::null(),
            };

            macro_rules! ast_string_constant {
                ($name:ident, $str:literal) => {
                    {
                        static data: &'static [u8] = $str.as_bytes();
                        let literal = Vector::new(data);
                        let handle = isolate.factory().name();
                        let raw_hash_field = handle.raw_hash_field();
                        assert_eq!(raw_hash_field, StringHasher::HashSequentialString::<u8>(literal.data, literal.length(), hash_seed));
                        assert_eq!(literal.length(), handle.length());
                        let ast_raw_string = constants.zone_.New::<AstRawString>();
                        unsafe {
                            (*ast_raw_string) = AstRawString::new(true, Vector::new(literal.data), raw_hash_field);
                            (*ast_raw_string).set_string(handle);
                        };

                        constants.$name##_string_ = ast_raw_string;

                        constants.string_table_.InsertNew(ast_raw_