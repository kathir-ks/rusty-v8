// Converted from V8 C++ source files:
// Header: string-builder.h
// Implementation: string-builder.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod string_builder {
    use crate::base::strings::base::uc16;
    use crate::execution::isolate_inl::isolate::Isolate;
    use crate::init::setup_isolate::V8;
    use crate::objects::fixed_array_inl::FixedArray;
    use crate::objects::js_array_inl::JSArray;
    use crate::objects::string::String;
    use crate::objects::string::String::WriteToFlat;
    use crate::strings::string_builder_inl::StringBuilderSubstringLength;
    use crate::strings::string_builder_inl::StringBuilderSubstringPosition;
    use std::string::String as StdString;
    use std::string::ToString;
    //use crate::objects::string::SeqOneByteString;
    use crate::objects::string::{SeqOneByteString, SeqTwoByteString};
    use crate::snapshot::snapshot_data::DisallowGarbageCollection;
    use std::cmp;
    use std::fmt::Write;
    use std::ptr::null_mut;

    #[derive(Debug)]
    pub enum FixedArrayBuilderError {
        CapacityError,
        AllocationError,
        InvalidLength,
        OutOfBounds,
    }

    #[derive(Debug)]
    pub enum ReplacementStringBuilderError {
        CapacityError,
        AllocationError,
        InvalidLength,
        OutOfBounds,
        FactoryError,
    }

    #[derive(Debug)]
    pub enum IncrementalStringBuilderError {
        AllocationError,
        InvalidStringLengthError,
        FactoryError,
    }

    #[derive(Debug)]
    pub enum ZoneAllocatorError {
        AllocationError,
    }

    pub struct FixedArrayBuilder {
        array_: *mut FixedArray,
        length_: usize,
        has_non_smi_elements_: bool,
    }

    impl FixedArrayBuilder {
        pub fn new(isolate: *mut Isolate, initial_capacity: usize) -> Result<Self, FixedArrayBuilderError> {
            if initial_capacity == 0 {
                return Err(FixedArrayBuilderError::CapacityError);
            }
            unsafe {
                let factory = &mut (*isolate).factory;
                let fixed_array = factory.NewFixedArrayWithHoles(initial_capacity);
                Ok(Self {
                    array_: fixed_array,
                    length_: 0,
                    has_non_smi_elements_: false,
                })
            }
        }

        pub fn from_backing_store(backing_store: *mut FixedArray) -> Result<Self, FixedArrayBuilderError> {
            unsafe {
                if (*backing_store).length() == 0 {
                    return Err(FixedArrayBuilderError::CapacityError);
                }

                Ok(Self {
                    array_: backing_store,
                    length_: 0,
                    has_non_smi_elements_: false,
                })
            }
        }

        pub fn lazy(isolate: *mut Isolate) -> Self {
            FixedArrayBuilder {
                array_: unsafe { (*isolate).factory.empty_fixed_array() },
                length_: 0,
                has_non_smi_elements_: false,
            }
        }

        pub fn has_capacity(&self, elements: usize) -> bool {
            unsafe {
                let length = (*self.array_).length();
                let required_length = self.length_ + elements;
                length >= required_length
            }
        }

        pub fn ensure_capacity(&mut self, isolate: *mut Isolate, elements: usize) -> Result<(), FixedArrayBuilderError> {
            unsafe {
                let length = (*self.array_).length();
                let required_length = self.length_ + elements;

                if length < required_length {
                    if length == 0 {
                        const K_INITIAL_CAPACITY_FOR_LAZY: usize = 16;
                        self.array_ = (*isolate).factory.NewFixedArrayWithHoles(cmp::max(K_INITIAL_CAPACITY_FOR_LAZY, elements));
                        return Ok(());
                    }

                    let mut new_length = length;
                    while new_length < required_length {
                        new_length *= 2;
                    }
                    let extended_array = (*isolate).factory.NewFixedArrayWithHoles(new_length);
                    FixedArray::CopyElements(isolate, *extended_array, 0, *self.array_, 0, self.length_);
                    self.array_ = extended_array;
                }
                Ok(())
            }
        }

        pub fn add(&mut self, value: *mut Object) -> Result<(), FixedArrayBuilderError> {
            unsafe {
                if value == null_mut() {
                    return Err(FixedArrayBuilderError::OutOfBounds);
                }
                (*self.array_).set(self.length_, value);
                self.length_ += 1;
                self.has_non_smi_elements_ = true;
                Ok(())
            }
        }

        pub fn add_smi(&mut self, value: *mut Smi) -> Result<(), FixedArrayBuilderError> {
            unsafe {
                (*self.array_).set(self.length_, value);
                self.length_ += 1;
                Ok(())
            }
        }

        pub fn array(&self) -> *mut FixedArray {
            self.array_
        }

        pub fn length(&self) -> usize {
            self.length_
        }

        pub fn capacity(&self) -> usize {
            unsafe {
                (*self.array_).length()
            }
        }
    }

    pub struct ReplacementStringBuilder {
        heap_: *mut v8::internal::Heap,
        array_builder_: FixedArrayBuilder,
        subject_: *mut String,
        character_count_: u32,
        is_one_byte_: bool,
    }

    impl ReplacementStringBuilder {
        pub fn new(
            heap: *mut v8::internal::Heap,
            subject: *mut String,
            estimated_part_count: usize,
        ) -> Result<Self, ReplacementStringBuilderError> {
            if estimated_part_count == 0 {
                return Err(ReplacementStringBuilderError::CapacityError);
            }
            let isolate = unsafe { Isolate::FromHeap(heap) };
            let array_builder = FixedArrayBuilder::new(isolate, estimated_part_count)
                .map_err(|_| ReplacementStringBuilderError::AllocationError)?;

            unsafe {
                Ok(Self {
                    heap_: heap,
                    array_builder_: array_builder,
                    subject_: subject,
                    character_count_: 0,
                    is_one_byte_: (*subject).IsOneByteRepresentation(),
                })
            }
        }

        pub fn ensure_capacity(&mut self, elements: usize) -> Result<(), ReplacementStringBuilderError> {
            let isolate = unsafe { Isolate::FromHeap(self.heap_) };
            self.array_builder_.ensure_capacity(isolate, elements)
                .map_err(|_| ReplacementStringBuilderError::CapacityError)
        }

        pub fn add_string(&mut self, string: *mut String) -> Result<(), ReplacementStringBuilderError> {
            unsafe {
                let length = (*string).length();
                self.add_element(string)?;
                if !(*string).IsOneByteRepresentation() {
                    self.is_one_byte_ = false;
                }
                self.increment_character_count(length);
                Ok(())
            }
        }

        pub fn to_string(&mut self) -> Result<*mut String, ReplacementStringBuilderError> {
            let isolate = unsafe { Isolate::FromHeap(self.heap_) };

            if self.array_builder_.length() == 0 {
                unsafe {
                    return Ok((*isolate).factory.empty_string());
                }
            }

            if self.is_one_byte_ {
                unsafe {
                    let seq = (*isolate).factory.NewRawOneByteString(self.character_count_ as usize);
                    if seq == null_mut() {
                        return Err(ReplacementStringBuilderError::AllocationError);
                    }

                    let no_gc = DisallowGarbageCollection {};
                    let char_buffer = (*seq).GetChars(no_gc);
                    StringBuilderConcatHelper(
                        self.subject_,
                        char_buffer,
                        self.array_builder_.array(),
                        self.array_builder_.length(),
                    );
                    Ok(seq as *mut String)
                }
            } else {
                unsafe {
                    let seq = (*isolate).factory.NewRawTwoByteString(self.character_count_ as usize);

                    if seq == null_mut() {
                        return Err(ReplacementStringBuilderError::AllocationError);
                    }

                    let no_gc = DisallowGarbageCollection {};
                    let char_buffer = (*seq).GetChars(no_gc);
                    StringBuilderConcatHelper(
                        self.subject_,
                        char_buffer,
                        self.array_builder_.array(),
                        self.array_builder_.length(),
                    );
                    Ok(seq as *mut String)
                }
            }
        }

        fn add_element(&mut self, element: *mut Object) -> Result<(), ReplacementStringBuilderError> {
            unsafe {
                self.ensure_capacity(1)?;
                let no_gc = DisallowGarbageCollection {};
                self.array_builder_.add(element).map_err(|_| ReplacementStringBuilderError::AllocationError)?;
                Ok(())
            }
        }

        fn increment_character_count(&mut self, by: u32) {
            if self.character_count_ > String::K_MAX_LENGTH - by {
                self.character_count_ = String::K_MAX_LENGTH;
            } else {
                self.character_count_ += by;
            }
        }
    }

    pub struct IncrementalStringBuilder {
        isolate_: *mut Isolate,
        encoding_: String::Encoding,
        overflowed_: bool,
        part_length_: usize,
        current_index_: usize,
        accumulator_: *mut String,
        current_part_: *mut String,
    }

    impl IncrementalStringBuilder {
        pub fn new(isolate: *mut Isolate) -> Self {
            unsafe {
                let roots = &ReadOnlyRoots::new(isolate);
                let initial_part_length = 32; // or kInitialPartLength
                let empty_string = roots.empty_string();
                let factory = &mut (*isolate).factory;
                let new_part = factory.NewRawOneByteString(initial_part_length).expect("Failed to create initial string part");

                IncrementalStringBuilder {
                    isolate_: isolate,
                    encoding_: String::Encoding::ONE_BYTE_ENCODING,
                    overflowed_: false,
                    part_length_: initial_part_length,
                    current_index_: 0,
                    accumulator_: empty_string,
                    current_part_: new_part,
                }
            }
        }

        pub fn current_encoding(&self) -> String::Encoding {
            self.encoding_
        }

        pub fn append<SrcChar, DestChar>(&mut self, c: SrcChar) {
            // Placeholder implementation
            println!("Append<SrcChar, DestChar> called with {:?}", c);
        }

        pub fn append_character(&mut self, c: u8) {
            // Placeholder implementation
            println!("AppendCharacter called with {:?}", c);
        }

        pub fn append_cstring_literal<const N: usize>(&mut self, literal: &[char; N]) {
            // Placeholder implementation
            println!("AppendCStringLiteral called with {:?}", literal);
        }

        pub fn append_cstring<SrcChar>(&mut self, s: *const SrcChar) {
            // Placeholder implementation
            println!("AppendCString called with {:?}", s);
        }

        pub fn append_string(&mut self, str: &str) {
            // Placeholder implementation
            println!("AppendString called with {:?}", str);
        }

        pub fn append_int(&mut self, i: i32) {
            // Placeholder implementation
            println!("AppendInt called with {:?}", i);
        }

        pub fn current_part_can_fit(&self, length: usize) -> bool {
            self.part_length_ - self.current_index_ > length
        }

        pub fn escaped_length_if_current_part_fits(&self, length: usize) -> i32 {
            // Placeholder implementation
            println!("EscapedLengthIfCurrentPartFits called with length {}", length);
            0 // Return a default value
        }

        pub fn append_string_handle(&mut self, string: *mut String) {
            unsafe {
                if self.can_append_by_copy(string) {
                    self.append_string_by_copy(string);
                    return;
                }

                self.shrink_current_part();
                self.part_length_ = 32;
                self.extend();
                self.accumulate(string);
            }
        }

        pub fn finish(&mut self) -> Result<*mut String, IncrementalStringBuilderError> {
            unsafe {
                self.shrink_current_part();
                self.accumulate(self.current_part_);
                if self.overflowed_ {
                    return Err(IncrementalStringBuilderError::InvalidStringLengthError);
                }
                let isolate = self.isolate_;
                if (*isolate).serializer_enabled() {
                    return Ok((*(*isolate).factory).InternalizeString(self.accumulator_));
                }
                Ok(self.accumulator_)
            }
        }

        pub fn has_overflowed(&self) -> bool {
            self.overflowed_
        }

        pub fn length(&self) -> usize {
            unsafe {
                (*self.accumulator_).length() + self.current_index_
            }
        }

        pub fn change_encoding(&mut self) {
            self.encoding_ = String::Encoding::TWO_BYTE_ENCODING;
        }

        pub fn isolate(&self) -> *mut Isolate {
            self.isolate_
        }

        fn factory(&mut self) -> *mut Factory {
            unsafe {
                &mut (*self.isolate_).factory
            }
        }

        fn accumulator(&self) -> *mut String {
            self.accumulator_
        }

        fn set_accumulator(&mut self, string: *mut String) {
            self.accumulator_ = string;
        }

        fn current_part(&self) -> *mut String {
            self.current_part_
        }

        fn set_current_part(&mut self, string: *mut String) {
            self.current_part_ = string;
        }

        fn accumulate(&mut self, new_part: *mut String) {
            unsafe {
                let mut new_accumulator: *mut String;
                if (*self.accumulator_).length() + (*new_part).length() > String::K_MAX_LENGTH {
                    new_accumulator = (*self.factory()).empty_string();
                    self.overflowed_ = true;
                } else {
                    new_accumulator = (*self.factory()).NewConsString(self.accumulator_, new_part);
                }
                self.set_accumulator(new_accumulator);
            }
        }

        fn extend(&mut self) {
            unsafe {
                assert_eq!((*self.current_part_).length(), self.current_index_);
                self.accumulate(self.current_part_);
                if self.part_length_ <= 16 * 1024 / 2 {
                    self.part_length_ *= 2;
                }
                let new_part = if self.encoding_ == String::Encoding::ONE_BYTE_ENCODING {
                    (*self.factory()).NewRawOneByteString(self.part_length_)
                } else {
                    (*self.factory()).NewRawTwoByteString(self.part_length_)
                };
                self.set_current_part(new_part);
                self.current_index_ = 0;
            }
        }

        fn has_valid_current_index(&self) -> bool {
            self.current_index_ < self.part_length_
        }

        fn shrink_current_part(&mut self) {
            unsafe {
                let isolate = self.isolate_;
                let factory = &mut (*isolate).factory;
                let current_part = self.current_part_;
                let current_index = self.current_index_;

                if (*current_part).length() != current_index {
                    let encoding = self.encoding_;
                    let shrunk_part = match encoding {
                        String::Encoding::ONE_BYTE_ENCODING => {
                            factory.NewRawOneByteString(current_index).map(|new_string| {
                                let no_gc = DisallowGarbageCollection {};
                                let chars = (*new_string).GetChars(no_gc);
                                let source_chars = (*Cast::<SeqOneByteString>(current_part)).GetChars(no_gc);
                                for i in 0..current_index {
                                    *chars.add(i) = *source_chars.add(i);
                                }
                                new_string
                            })
                        }
                        String::Encoding::TWO_BYTE_ENCODING => {
                            factory.NewRawTwoByteString(current_index).map(|new_string| {
                                let no_gc = DisallowGarbageCollection {};
                                let chars = (*new_string).GetChars(no_gc);
                                let source_chars = (*Cast::<SeqTwoByteString>(current_part)).GetChars(no_gc);
                                for i in 0..current_index {
                                    *chars.add(i) = *source_chars.add(i);
                                }
                                new_string
                            })
                        }
                        _ => panic!("Unexpected string encoding"),
                    };
                    self.set_current_part(shrunk_part.expect("Shrinking string failed"));
                }
            }
        }

        fn append_string_by_copy(&mut self, string: *mut String) {
            unsafe {
                assert!(self.can_append_by_copy(string));

                let length = (*string).length();
                let no_gc = DisallowGarbageCollection {};

                if self.encoding_ == String::Encoding::ONE_BYTE_ENCODING {
                    String::WriteToFlat(
                        string,
                        (*Cast::<SeqOneByteString>(self.current_part_)).GetChars(no_gc).add(self.current_index_),
                        0,
                        length,
                    );
                } else {
                    String::WriteToFlat(
                        string,
                        (*Cast::<SeqTwoByteString>(self.current_part_)).GetChars(no_gc).add(self.current_index_),
                        0,
                        length,
                    );
                }
                self.current_index_ += length;
                assert!(self.current_index_ <= self.part_length_);
                if self.current_index_ == self.part_length_ {
                    self.extend();
                }
            }
        }

        fn can_append_by_copy(&self, string: *mut String) -> bool {
            unsafe {
                let representation_ok = self.encoding_ == String::Encoding::TWO_BYTE_ENCODING
                    || ((*string).IsFlat() && String::IsOneByteRepresentationUnderneath(*string));

                representation_ok && self.current_part_can_fit((*string).length())
            }
        }
    }

    pub struct ReadOnlyRoots {
        pub empty_string: *mut String,
    }

    impl ReadOnlyRoots {
        pub fn new(isolate: *mut Isolate) -> Self {
            unsafe {
                ReadOnlyRoots {
                    empty_string: (*isolate).empty_string
                }
            }
        }
    }

    pub struct Factory {
        pub isolate_: *mut Isolate,
        //empty_string: *mut String,
    }

    impl Factory {
        pub fn NewFixedArrayWithHoles(&mut self, size: usize) -> *mut FixedArray {
            // Placeholder implementation
            println!("NewFixedArrayWithHoles called with size {}", size);
            null_mut() // Return null for now
        }

        pub fn NewRawOneByteString(&mut self, length: usize) -> Result<*mut String, IncrementalStringBuilderError> {
            // Placeholder implementation
            println!("NewRawOneByteString called with length {}", length);
            Ok(null_mut()) // Return null for now
        }

        pub fn NewRawTwoByteString(&mut self, length: usize) -> Result<*mut String, IncrementalStringBuilderError> {
            // Placeholder implementation
            println!("NewRawTwoByteString called with length {}", length);
            Ok(null_mut()) // Return null for now
        }

        pub fn NewConsString(&mut self, left: *mut String, right: *mut String) -> *mut String {
            // Placeholder implementation
            println!("NewConsString called");
            null_mut() // Return null for now
        }

        pub fn InternalizeString(&mut self, string: *mut String) -> *mut String {
            // Placeholder implementation
            println!("InternalizeString called");
            null_mut() // Return null for now
        }

         // Method to simulate creating an empty string.  Since we have a placeholder,
        // just return a null pointer to represent it.
        pub fn empty_string(&self) -> *mut String {
           null_mut() // Return null to simulate an empty string
        }

    }

    pub struct Smi {}
    pub struct Object {}

    unsafe fn Cast<T>(ptr: *mut String) -> *mut T {
        ptr as *mut T
    }

    impl String {
        pub const K_MAX_LENGTH: u32 = 1 << 28;
    }

    pub fn StringBuilderConcatHelper<sinkchar>(
        special: *mut String,
        sink: *mut sinkchar,
        fixed_array: *mut FixedArray,
        array_length: usize,
    ) {
        let no_gc = DisallowGarbageCollection {};
        let mut position = 0;
        unsafe {
            for i in 0..array_length {
                let element = (*fixed_array).get(i);
                if element == null_mut() {
                    continue; // Skip null elements, assuming they are holes.
                }
                if IsSmi(element) {
                    let encoded_slice = Smi::ToInt(element);
                    let pos: i32;
                    let len: i32;
                    if encoded_slice > 0 {
                        pos = StringBuilderSubstringPosition::decode(encoded_slice);
                        len = StringBuilderSubstringLength::decode(encoded_slice);
                    } else {
                        let obj = (*fixed_array).get(i + 1);
                        if obj == null_mut() {
                            continue; // Skip if next object is null
                        }
                        assert!(IsSmi(obj));
                        pos = Smi::ToInt(obj);
                        len = -encoded_slice;
                    }
                    String::WriteToFlat(special, sink.add(position), pos, len);
                    position += len as usize;
                } else {
                    let string = Cast::<String>(element);
                    let element_length = (*string).length();
                    String::WriteToFlat(string, sink.add(position), 0, element_length);
                    position += element_length;
                }
            }
        }
    }

    impl Smi {
        fn ToInt(element: *mut Object) -> i32 {
            0
        }
    }

    unsafe fn IsSmi(element: *mut Object) -> bool {
        false
    }

    unsafe fn IsString(element: *mut Object) -> bool {
        true
    }

    impl String {
        pub fn IsOneByteRepresentation(&self) -> bool {
            true
        }

        pub fn IsOneByteRepresentationUnderneath(string: &String) -> bool {
            true
        }

        pub fn IsFlat(&self) -> bool {
            true
        }

        pub fn length(&self) -> usize {
            0
        }
    }
}
