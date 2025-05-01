// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Add appropriate Rust crates for any C++ libraries used

pub mod v8 {
    pub struct Isolate {}

    pub struct Context {}

    pub struct Local<'a, T> {
        _phantom: std::marker::PhantomData<&'a T>,
    }

    impl<'a, T> Local<'a, T> {
        pub fn from_slot(_slot: *const ()) -> Self {
            Local {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    pub trait Data {}

    pub struct Value {}

    impl Data for Value {}

    pub struct Primitive {}

    impl Data for Primitive {}
    impl Value for Primitive {}

    pub struct Boolean {}

    impl Data for Boolean {}
    impl Value for Boolean {}
    impl Primitive for Boolean {}

    impl Boolean {
        pub fn value(&self) -> bool {
            todo!()
        }

        pub fn cast<'a>(data: &'a dyn Data) -> &'a Boolean {
            // TODO: Implement CheckCast
            Boolean::check_cast(data);
            data.downcast_ref::<Boolean>().unwrap()
        }

        pub fn new<'a>(_isolate: &Isolate, _value: bool) -> Local<'a, Boolean> {
            todo!()
        }

        fn check_cast(_that: &dyn Data) {
            // TODO: Implement CheckCast
        }
    }

    pub struct PrimitiveArray {}

    impl Data for PrimitiveArray {}

    impl PrimitiveArray {
        pub fn new<'a>(_isolate: &Isolate, _length: i32) -> Local<'a, PrimitiveArray> {
            todo!()
        }
        pub fn length(&self) -> i32 {
            todo!()
        }
        pub fn set<'a>(&self, _isolate: &Isolate, _index: i32, _item: Local<'a, Primitive>) {
            todo!()
        }
        pub fn get<'a>(&self, _isolate: &Isolate, _index: i32) -> Local<'a, Primitive> {
            todo!()
        }

        pub fn cast<'a>(data: &'a dyn Data) -> &'a PrimitiveArray {
            PrimitiveArray::check_cast(data);
            data.downcast_ref::<PrimitiveArray>().unwrap()
        }

        fn check_cast(_obj: &dyn Data) {
            // TODO: Implement CheckCast
        }
    }

    pub struct Name {}

    impl Data for Name {}
    impl Value for Name {}
    impl Primitive for Name {}

    impl Name {
        pub fn get_identity_hash(&self) -> i32 {
            todo!()
        }

        pub fn cast<'a>(data: &'a dyn Data) -> &'a Name {
            Name::check_cast(data);
            data.downcast_ref::<Name>().unwrap()
        }

        fn check_cast(_that: &dyn Data) {
            // TODO: Implement CheckCast
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub enum NewStringType {
        Normal,
        Internalized,
    }

    pub struct String {}

    impl Data for String {}
    impl Value for String {}
    impl Name for String {}
    impl Primitive for String {}

    impl String {
        pub const KMAX_LENGTH: usize = if std::mem::size_of::<usize>() == 4 {
            (1 << 28) - 16
        } else {
            (1 << 29) - 24
        };

        #[derive(Debug, Copy, Clone)]
        pub enum Encoding {
            UnknownEncoding = 0x1,
            TwoByteEncoding = 0x0,
            OneByteEncoding = 0x8,
        }

        pub fn length(&self) -> i32 {
            todo!()
        }

        #[deprecated(since = "0.1.0", note = "Use utf8_length_v2 instead")]
        pub fn utf8_length(&self, _isolate: &Isolate) -> i32 {
            todo!()
        }

        pub fn utf8_length_v2(&self, _isolate: &Isolate) -> usize {
            todo!()
        }

        pub fn is_one_byte(&self) -> bool {
            todo!()
        }

        pub fn contains_only_one_byte(&self) -> bool {
            todo!()
        }

        #[derive(Debug, Copy, Clone)]
        pub enum WriteOptions {
            NoOptions = 0,
            HintManyWritesExpected = 1,
            NoNullTermination = 2,
            PreserveOneByteNull = 4,
            ReplaceInvalidUtf8 = 8,
        }

        #[derive(Debug, Copy, Clone)]
        pub struct WriteFlags {
            // Using a bitflag enum
            bits: u32,
        }

        impl WriteFlags {
            pub const KNONE: i32 = 0;
            pub const KNULL_TERMINATE: i32 = 1;
            pub const KREPLACE_INVALID_UTF8: i32 = 2;

            pub fn new(bits: u32) -> Self {
                Self { bits }
            }

            pub fn is_null_terminate(&self) -> bool {
                (self.bits & Self::KNULL_TERMINATE as u32) != 0
            }

            pub fn is_replace_invalid_utf8(&self) -> bool {
                (self.bits & Self::KREPLACE_INVALID_UTF8 as u32) != 0
            }
        }

        #[deprecated(since = "0.1.0", note = "Use write_v2 instead")]
        pub fn write(
            &self,
            _isolate: &Isolate,
            _buffer: *mut u16,
            _start: i32,
            _length: i32,
            _options: i32,
        ) -> i32 {
            todo!()
        }
        #[deprecated(since = "0.1.0", note = "Use write_one_byte_v2 instead")]
        pub fn write_one_byte(
            &self,
            _isolate: &Isolate,
            _buffer: *mut u8,
            _start: i32,
            _length: i32,
            _options: i32,
        ) -> i32 {
            todo!()
        }
        #[deprecated(since = "0.1.0", note = "Use write_utf8_v2 instead")]
        pub fn write_utf8(
            &self,
            _isolate: &Isolate,
            _buffer: *mut i8,
            _length: i32,
            _nchars_ref: *mut i32,
            _options: i32,
        ) -> i32 {
            todo!()
        }

        pub fn write_v2(
            &self,
            _isolate: &Isolate,
            _offset: u32,
            _length: u32,
            _buffer: *mut u16,
            _flags: WriteFlags,
        ) {
            todo!()
        }

        pub fn write_one_byte_v2(
            &self,
            _isolate: &Isolate,
            _offset: u32,
            _length: u32,
            _buffer: *mut u8,
            _flags: WriteFlags,
        ) {
            todo!()
        }

        pub fn write_utf8_v2(
            &self,
            _isolate: &Isolate,
            _buffer: *mut i8,
            _capacity: usize,
            _flags: WriteFlags,
            _processed_characters_return: *mut usize,
        ) -> usize {
            todo!()
        }

        pub fn empty<'a>(_isolate: &Isolate) -> Local<'a, String> {
            todo!()
        }

        pub fn is_external(&self) -> bool {
            todo!()
        }

        pub fn is_external_two_byte(&self) -> bool {
            todo!()
        }

        pub fn is_external_one_byte(&self) -> bool {
            todo!()
        }

        pub fn internalize_string<'a>(&self, _isolate: &Isolate) -> Local<'a, String> {
            todo!()
        }

        pub struct ExternalStringResourceBase {}

        impl ExternalStringResourceBase {
            pub fn is_cacheable(&self) -> bool {
                true
            }

            pub fn unaccount(&mut self, _isolate: &Isolate) {
                todo!()
            }

            pub fn estimate_memory_usage(&self) -> usize {
                usize::MAX //kDefaultMemoryEstimate
            }

            pub struct SharedMemoryUsageRecorder {}

            impl SharedMemoryUsageRecorder {
                pub fn record_shared_memory_usage(&mut self, _location: *const std::ffi::c_void, _size: usize) {
                    todo!()
                }
            }

            pub fn estimate_shared_memory_usage(&self, _recorder: &mut SharedMemoryUsageRecorder) {
                todo!()
            }

            pub fn lock(&self) const {}

            pub fn unlock(&self) const {}

            // Intentionally blank as ExternalStringResourceBase has no data
            // TODO: This code looks incorrect.  It is calling "delete this" in rust,
            //   which would cause the memory to be deallocated twice.  This virtual
            //   method needs to be carefully rewritten.
            //unsafe fn dispose(&mut self) {
            //    //let ptr = self as *mut Self;
            //    //std::ptr::drop_in_place(ptr);
            //    //std::alloc::dealloc(ptr as *mut u8, Layout::new::<Self>());
            //}

            // This is a placeholder, since `dispose` calls `delete this`.
            // This logic must be rewritten.
            pub fn dispose(&mut self) {
                // Default implementation does nothing to avoid double free.
                // The subclass must implement correct disposal logic.
            }
        }

        pub struct ExternalStringResource {}

        impl ExternalStringResource {
            pub fn data(&self) -> *const u16 {
                todo!()
            }

            pub fn length(&self) -> usize {
                todo!()
            }

            pub fn cached_data(&self) -> *const u16 {
                self.check_cached_data_invariants();
                self.cached_data_
            }

            pub fn update_data_cache(&mut self) {
                todo!()
            }

            fn check_cached_data_invariants(&self) {
                todo!()
            }
        }

        pub struct ExternalOneByteStringResource {}

        impl ExternalOneByteStringResource {
            pub fn data(&self) -> *const i8 {
                todo!()
            }

            pub fn length(&self) -> usize {
                todo!()
            }

            pub fn cached_data(&self) -> *const i8 {
                self.check_cached_data_invariants();
                self.cached_data_
            }

            pub fn update_data_cache(&mut self) {
                todo!()
            }

            fn check_cached_data_invariants(&self) {
                todo!()
            }
        }

        pub fn get_external_string_resource_base<'a>(
            &self,
            _isolate: &Isolate,
            encoding_out: &mut Encoding,
        ) -> *mut ExternalStringResourceBase {
            todo!()
        }
        pub fn get_external_string_resource_base_no_isolate<'a>(
            &self,
            encoding_out: &mut Encoding,
        ) -> *mut ExternalStringResourceBase {
            todo!()
        }

        pub fn get_external_string_resource(&self) -> *mut ExternalStringResource {
            todo!()
        }

        pub fn get_external_one_byte_string_resource(&self) -> *const ExternalOneByteStringResource {
            todo!()
        }

        pub fn cast<'a>(data: &'a dyn Data) -> &'a String {
            String::check_cast(data);
            data.downcast_ref::<String>().unwrap()
        }

        pub fn new_from_utf8_literal<'a, const N: usize>(
            _isolate: &Isolate,
            _literal: &'static [u8; N],
            _type: NewStringType,
        ) -> Local<'a, String> {
            assert!(N <= String::KMAX_LENGTH);
            String::new_from_utf8_literal_internal(_isolate, _literal.as_ptr() as *const i8, _type, N - 1)
        }

        pub fn new_from_utf8<'a>(
            _isolate: &Isolate,
            _data: *const i8,
            _type: NewStringType,
            _length: i32,
        ) -> Result<Local<'a, String>, ()> {
            todo!()
        }

        pub fn new_from_one_byte<'a>(
            _isolate: &Isolate,
            _data: *const u8,
            _type: NewStringType,
            _length: i32,
        ) -> Result<Local<'a, String>, ()> {
            todo!()
        }

        pub fn new_from_two_byte<'a>(
            _isolate: &Isolate,
            _data: *const u16,
            _type: NewStringType,
            _length: i32,
        ) -> Result<Local<'a, String>, ()> {
            todo!()
        }

        pub fn concat<'a>(_isolate: &Isolate, _left: Local<'a, String>, _right: Local<'a, String>) -> Local<'a, String> {
            todo!()
        }

        pub fn new_external_two_byte<'a>(
            _isolate: &Isolate,
            _resource: *mut ExternalStringResource,
        ) -> Result<Local<'a, String>, ()> {
            todo!()
        }

        #[deprecated(since = "0.1.0", note = "Use the version with the isolate argument instead.")]
        pub fn make_external(&self, _resource: *mut ExternalStringResource) -> bool {
            todo!()
        }

        pub fn make_external_with_isolate(&self, _isolate: &Isolate, _resource: *mut ExternalStringResource) -> bool {
            todo!()
        }

        pub fn new_external_one_byte<'a>(
            _isolate: &Isolate,
            _resource: *mut ExternalOneByteStringResource,
        ) -> Result<Local<'a, String>, ()> {
            todo!()
        }

        #[deprecated(since = "0.1.0", note = "Use the version with the isolate argument instead.")]
        pub fn make_external_one_byte(&self, _resource: *mut ExternalOneByteStringResource) -> bool {
            todo!()
        }

        pub fn make_external_one_byte_with_isolate(&self, _isolate: &Isolate, _resource: *mut ExternalOneByteStringResource) -> bool {
            todo!()
        }

        pub fn can_make_external(&self, _encoding: Encoding) -> bool {
            todo!()
        }

        pub fn string_equals(&self, _str: Local<'_, String>) -> bool {
            todo!()
        }

        pub struct Utf8Value {}

        impl Utf8Value {
            pub fn new(_isolate: &Isolate, _obj: Local<'_, Value>, _options: WriteOptions) -> Self {
                todo!()
            }

            pub fn as_ptr(&self) -> *mut i8 {
                self.str_
            }

            pub fn as_ref(&self) -> *const i8 {
                self.str_
            }

            pub fn length(&self) -> usize {
                self.length_
            }
        }

        pub struct ValueString {}

        impl ValueString {
            #[deprecated(
                since = "0.1.0",
                note = "Prefer using String::ValueView if you can, or string->Write to a buffer if you cannot."
            )]
            pub fn new(_isolate: &Isolate, _obj: Local<'_, Value>) -> Self {
                todo!()
            }

            pub fn as_ptr(&self) -> *mut u16 {
                self.str_
            }

            pub fn as_ref(&self) -> *const u16 {
                self.str_
            }

            pub fn length(&self) -> u32 {
                self.length_
            }
        }

        pub struct ValueView {}

        impl ValueView {
            pub fn new(_isolate: &Isolate, _str: Local<'_, String>) -> Self {
                todo!()
            }

            pub fn data8(&self) -> *const u8 {
                self.check_one_byte(true);
                self.data8_
            }

            pub fn data16(&self) -> *const u16 {
                self.check_one_byte(false);
                self.data16_
            }

            pub fn length(&self) -> u32 {
                self.length_
            }

            pub fn is_one_byte(&self) -> bool {
                self.is_one_byte_
            }

            fn check_one_byte(&self, _is_one_byte: bool) {
                todo!()
            }
        }

        fn verify_external_string_resource_base(&self, _v: *mut ExternalStringResourceBase, _encoding: Encoding) {
            todo!()
        }
        fn verify_external_string_resource(&self, _val: *mut ExternalStringResource) {
            todo!()
        }
        fn get_external_string_resource_slow(&self) -> *mut ExternalStringResource {
            todo!()
        }
        fn get_external_string_resource_base_slow(&self, encoding_out: &mut Encoding) -> *mut ExternalStringResourceBase {
            todo!()
        }

        fn new_from_utf8_literal_internal<'a>(
            _isolate: &Isolate,
            _literal: *const i8,
            _type: NewStringType,
            _length: i32,
        ) -> Local<'a, String> {
            todo!()
        }

        fn check_cast(_that: &dyn Data) {
            // TODO: Implement CheckCast
        }
    }

    pub struct ExternalResourceVisitor {}

    impl ExternalResourceVisitor {
        pub fn visit_external_string(&mut self, _string: Local<'_, String>) {}
    }

    pub struct Symbol {}

    impl Data for Symbol {}
    impl Value for Symbol {}
    impl Name for Symbol {}
    impl Primitive for Symbol {}

    impl Symbol {
        pub fn description<'a>(&self, _isolate: &Isolate) -> Local<'a, Value> {
            todo!()
        }

        pub fn new<'a>(_isolate: &Isolate, _description: Local<'a, String>) -> Local<'a, Symbol> {
            todo!()
        }

        pub fn for_symbol<'a>(_isolate: &Isolate, _description: Local<'a, String>) -> Local<'a, Symbol> {
            todo!()
        }

        pub fn for_api<'a>(_isolate: &Isolate, _description: Local<'a, String>) -> Local<'a, Symbol> {
            todo!()
        }

        pub fn get_async_iterator<'a>(_isolate: &Isolate) -> Local<'a, Symbol> {
            todo!()
        }
        pub fn get_has_instance<'a>(_isolate: &Isolate) -> Local<'a, Symbol> {
            todo!()
        }
        pub fn get_is_concat_spreadable<'a>(_isolate: &Isolate) -> Local<'a, Symbol> {
            todo!()
        }
        pub fn get_iterator<'a>(_isolate: &Isolate) -> Local<'a, Symbol> {
            todo!()
        }
        pub fn get_match<'a>(_isolate: &Isolate) -> Local<'a, Symbol> {
            todo!()
        }
        pub fn get_replace<'a>(_isolate: &Isolate) -> Local<'a, Symbol> {
            todo!()
        }
        pub fn get_search<'a>(_isolate: &Isolate) -> Local<'a, Symbol> {
            todo!()
        }
        pub fn get_split<'a>(_isolate: &Isolate) -> Local<'a, Symbol> {
            todo!()
        }
        pub fn get_to_primitive<'a>(_isolate: &Isolate) -> Local<'a, Symbol> {
            todo!()
        }
        pub fn get_to_string_tag<'a>(_isolate: &Isolate) -> Local<'a, Symbol> {
            todo!()
        }
        pub fn get_unscopables<'a>(_isolate: &Isolate) -> Local<'a, Symbol> {
            todo!()
        }

        pub fn cast<'a>(data: &'a dyn Data) -> &'a Symbol {
            Symbol::check_cast(data);
            data.downcast_ref::<Symbol>().unwrap()
        }

        fn check_cast(_that: &dyn Data) {
            // TODO: Implement CheckCast
        }
    }

    pub struct Numeric {}

    impl Data for Numeric {}
    impl Value for Numeric {}
    impl Primitive for Numeric {}

    impl Numeric {
        fn check_cast(_that: &dyn Data) {
            // TODO: Implement CheckCast
        }
    }

    pub struct Number {}

    impl Data for Number {}
    impl Value for Number {}
    impl Primitive for Number {}
    impl Numeric for Number {}

    impl Number {
        pub fn value(&self) -> f64 {
            todo!()
        }
        pub fn new<'a>(_isolate: &Isolate, _value: f64) -> Local<'a, Number> {
            todo!()
        }

        pub fn cast<'a>(data: &'a dyn Data) -> &'a Number {
            Number::check_cast(data);
            data.downcast_ref::<Number>().unwrap()
        }

        fn check_cast(_that: &dyn Data) {
            // TODO: Implement CheckCast
        }
    }

    pub struct Integer {}

    impl Data for Integer {}
    impl Value for Integer {}
    impl Primitive for Integer {}
    impl Numeric for Integer {}
    impl Number for Integer {}

    impl Integer {
        pub fn new<'a>(_isolate: &Isolate, _value: i32) -> Local<'a, Integer> {
            todo!()
        }
        pub fn new_from_unsigned<'a>(_isolate: &Isolate, _value: u32) -> Local<'a, Integer> {
            todo!()
        }
        pub fn value(&self) -> i64 {
            todo!()
        }

        pub fn cast<'a>(data: &'a dyn Data) -> &'a Integer {
            Integer::check_cast(data);
            data.downcast_ref::<Integer>().unwrap()
        }

        fn check_cast(_that: &dyn Data) {
            // TODO: Implement CheckCast
        }
    }

    pub struct Int32 {}

    impl Data for Int32 {}
    impl Value for Int32 {}
    impl Primitive for Int32 {}
    impl Numeric for Int32 {}
    impl Number for Int32 {}
    impl Integer for Int32 {}

    impl Int32 {
        pub fn value(&self) -> i32 {
            todo!()
        }

        pub fn cast<'a>(data: &'a dyn Data) -> &'a Int32 {
            Int32::check_cast(data);
            data.downcast_ref::<Int32>().unwrap()
        }

        fn check_cast(_that: &dyn Data) {
            // TODO: Implement CheckCast
        }
    }

    pub struct Uint32 {}

    impl Data for Uint32 {}
    impl Value for Uint32 {}
    impl Primitive for Uint32 {}
    impl Numeric for Uint32 {}
    impl Number for Uint32 {}
    impl Integer for Uint32 {}

    impl Uint32 {
        pub fn value(&self) -> u32 {
            todo!()
        }

        pub fn cast<'a>(data: &'a dyn Data) -> &'a Uint32 {
            Uint32::check_cast(data);
            data.downcast_ref::<Uint32>().unwrap()
        }

        fn check_cast(_that: &dyn Data) {
            // TODO: Implement CheckCast
        }
    }

    pub struct BigInt {}

    impl Data for BigInt {}
    impl Value for BigInt {}
    impl Primitive for BigInt {}
    impl Numeric for BigInt {}

    impl BigInt {
        pub fn new<'a>(_isolate: &Isolate, _value: i64) -> Local<'a, BigInt> {
            todo!()
        }
        pub fn new_from_unsigned<'a>(_isolate: &Isolate, _value: u64) -> Local<'a, BigInt> {
            todo!()
        }
        pub fn new_from_words<'a>(
            _context: Local<'a, Context>,
            _sign_bit: i32,
            _word_count: i32,
            _words: *const u64,
        ) -> Result<Local<'a, BigInt>, ()> {
            todo!()
        }

        pub fn uint64_value(&self, _lossless: *mut bool) -> u64 {
            todo!()
        }

        pub fn int64_value(&self, _lossless: *mut bool) -> i64 {
            todo!()
        }

        pub fn word_count(&self) -> i32 {
            todo!()
        }

        pub fn to_words_array(&self, _sign_bit: *mut i32, _word_count: *mut i32, _words: *mut u64) {
            todo!()
        }

        pub fn cast<'a>(data: &'a dyn Data) -> &'a BigInt {
            BigInt::check_cast(data);
            data.downcast_ref::<BigInt>().unwrap()
        }

        fn check_cast(_that: &dyn Data) {
            // TODO: Implement CheckCast
        }
    }

    pub fn undefined<'a>(_isolate: &Isolate) -> Local<'a, Primitive> {
        todo!()
    }

    pub fn null<'a>(_isolate: &Isolate) -> Local<'a, Primitive> {
        todo!()
    }

    pub fn boolean_true<'a>(_isolate: &Isolate) -> Local<'a, Boolean> {
        todo!()
    }

    pub fn boolean_false<'a>(_isolate: &Isolate) -> Local<'a, Boolean> {
        todo!()
    }
}

use v8::*;
