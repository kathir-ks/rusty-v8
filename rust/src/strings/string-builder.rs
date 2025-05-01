// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod string_builder {
    use std::rc::Rc;
    use std::string::String as StdString;
    use std::convert::TryInto;

    const K_MAX_INT: u32 = i32::MAX as u32;

    #[derive(Debug)]
    pub struct FixedArray {
        data: Vec<Object>,
    }

    impl FixedArray {
        pub fn new(capacity: usize) -> Self {
            FixedArray {
                data: vec![Object::Smi(0); capacity], // Initialize with Smi(0) or a default value
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Object {
        Smi(i32),
        String(String),
        // Add other object types as needed
    }

    #[derive(Debug, Clone)]
    pub struct String {
        data: StdString,
        encoding: Encoding,
    }

    impl String {
        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        pub const K_MAX_LENGTH: u32 = K_MAX_INT - 1;

        pub fn encoding(&self) -> Encoding {
            self.encoding
        }

        pub fn new(data: StdString, encoding: Encoding) -> Self {
            String { data, encoding }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Encoding {
        Utf8,
        Utf16,
    }

    #[derive(Debug)]
    pub struct Isolate {}

    impl Isolate {
        pub fn new() -> Self {
            Isolate {}
        }
    }

    #[derive(Debug, Clone)]
    pub struct Handle<T> {
        value: Rc<T>,
    }

    impl<T> Handle<T> {
        pub fn new(value: T) -> Self {
            Handle {
                value: Rc::new(value),
            }
        }

        pub fn value(&self) -> Rc<T> {
            self.value.clone()
        }
    }

    #[derive(Debug)]
    pub struct DirectHandle<T> {
        value: Rc<T>,
    }

    impl<T> DirectHandle<T> {
        pub fn new(value: T) -> Self {
            DirectHandle {
                value: Rc::new(value),
            }
        }

        pub fn value(&self) -> Rc<T> {
            self.value.clone()
        }

        pub fn set_value(&mut self, new_value: T) {
            self.value = Rc::new(new_value);
        }
    }
    
    #[derive(Debug)]
    pub struct FixedArrayBuilder {
        array_: DirectHandle<FixedArray>,
        length_: usize,
        has_non_smi_elements_: bool,
        isolate: Option<Isolate>,
        lazy: bool,
        initial_capacity: usize,
    }

    impl FixedArrayBuilder {
        pub fn new(isolate: &Isolate, initial_capacity: usize) -> Self {
            FixedArrayBuilder {
                array_: DirectHandle::new(FixedArray::new(initial_capacity)),
                length_: 0,
                has_non_smi_elements_: false,
                isolate: Some(isolate.clone()),
                lazy: false,
                initial_capacity,
            }
        }

        pub fn from_backing_store(backing_store: DirectHandle<FixedArray>) -> Self {
            FixedArrayBuilder {
                array_: backing_store,
                length_: 0,
                has_non_smi_elements_: false,
                isolate: None, // Isolate not available in this constructor.
                lazy: false,
                initial_capacity: 0, //Initial capacity not tracked
            }
        }

        pub fn lazy(isolate: &Isolate) -> Self {
             FixedArrayBuilder {
                array_: DirectHandle::new(FixedArray::new(0)),
                length_: 0,
                has_non_smi_elements_: false,
                isolate: Some(isolate.clone()),
                lazy: true,
                initial_capacity: 0,
            }
        }

        pub fn has_capacity(&self, elements: usize) -> bool {
            self.capacity() >= self.length_ + elements
        }

        pub fn ensure_capacity(&mut self, isolate: &Isolate, elements: usize) {
            if self.lazy {
                if self.isolate.is_none() {
                    self.isolate = Some(isolate.clone());
                    self.initial_capacity = 16; // Choose a reasonable default.
                    self.array_ = DirectHandle::new(FixedArray::new(self.initial_capacity));
                    self.lazy = false;
                    
                } else if !self.has_capacity(elements){
                  let current_capacity = self.capacity();
                  let new_capacity = std::cmp::max(current_capacity * 2, self.length_ + elements);
                  let mut new_array = FixedArray::new(new_capacity);
                  
                  // Copy elements from the old array to the new array.
                  for i in 0..self.length_ {
                      new_array.data[i] = self.array_.value().data[i].clone();
                  }

                  // Update the backing store.
                  self.array_.set_value(new_array);
                }
            }
             else if !self.has_capacity(elements) {
                let current_capacity = self.capacity();
                let new_capacity = std::cmp::max(current_capacity * 2, self.length_ + elements);
                let mut new_array = FixedArray::new(new_capacity);

                // Copy elements from the old array to the new array.
                for i in 0..self.length_ {
                    new_array.data[i] = self.array_.value().data[i].clone();
                }

                // Update the backing store.
                self.array_.set_value(new_array);
            }
        }

        pub fn add(&mut self, value: Object) {
            let isolate = self.isolate.as_ref().expect("Isolate should be available when adding elements.");
            self.ensure_capacity(isolate, 1);
            self.array_.value().data[self.length_] = value;
            if let Object::Smi(_) = value {
                // Do nothing.
            } else {
                self.has_non_smi_elements_ = true;
            }
            self.length_ += 1;
        }

        pub fn add_smi(&mut self, value: i32) {
            self.add(Object::Smi(value));
        }

        pub fn array(&self) -> DirectHandle<FixedArray> {
            self.array_.clone()
        }

        pub fn length(&self) -> usize {
            self.length_
        }

        pub fn capacity(&self) -> usize {
            self.array_.value().data.len()
        }
    }

    #[derive(Debug)]
    pub struct ReplacementStringBuilder {
        heap_: Heap,
        array_builder_: FixedArrayBuilder,
        subject_: DirectHandle<String>,
        character_count_: u32,
        is_one_byte_: bool,
    }

    impl ReplacementStringBuilder {
        pub fn new(heap: &Heap, subject: DirectHandle<String>, estimated_part_count: usize) -> Self {
            let isolate = Isolate::new();
            ReplacementStringBuilder {
                heap_: heap.clone(),
                array_builder_: FixedArrayBuilder::new(&isolate, estimated_part_count),
                subject_: subject,
                character_count_: 0,
                is_one_byte_: true,
            }
        }

        pub fn add_subject_slice(builder: &mut FixedArrayBuilder, from: usize, to: usize) {
            // Dummy implementation.  In V8 this adds a slice of the subject string to the builder.
            // Needs adaptation to correctly represent the FixedArrayBuilder and String types.
            let dummy_string = String {
                data: format!("Slice from {} to {}", from, to),
                encoding: Encoding::Utf8,
            };
            builder.add(Object::String(dummy_string));
        }

        pub fn add_subject_slice_self(&mut self, from: usize, to: usize) {
             let isolate = Isolate::new();
             self.array_builder_.ensure_capacity(&isolate, 1);

            let slice = String {
                data: self.subject_.value().data[from..to].to_string(),
                encoding: self.subject_.value().encoding(),
            };
             self.add_element(DirectHandle::new(slice));
        }

        pub fn add_string(&mut self, string: DirectHandle<String>) {
            let isolate = Isolate::new();
            self.array_builder_.ensure_capacity(&isolate, 1);
            self.add_element(string);
        }

        pub fn to_string(&mut self) -> Result<DirectHandle<String>, &'static str> {
           let isolate = Isolate::new();
           self.array_builder_.ensure_capacity(&isolate, 1);
            // Dummy implementation.  Needs adaptation based on the actual logic.
            let mut result = StdString::new();
            for i in 0..self.array_builder_.length() {
                match &self.array_builder_.array().value().data[i] {
                    Object::String(s) => result.push_str(&s.data),
                    _ => return Err("Non-string object found in array builder"),
                }
            }
            Ok(DirectHandle::new(String {
                data: result,
                encoding: Encoding::Utf8,
            }))
        }

        pub fn increment_character_count(&mut self, by: u32) {
            if self.character_count_ > String::K_MAX_LENGTH - by {
                self.character_count_ = K_MAX_INT;
            } else {
                self.character_count_ += by;
            }
        }

        fn add_element(&mut self, element: DirectHandle<String>) {
             let isolate = Isolate::new();
            self.array_builder_.ensure_capacity(&isolate, 1);
            self.array_builder_.add(Object::String(element.value().clone()));
            self.character_count_ += element.value().len() as u32;
        }

        fn ensure_capacity(&mut self, elements: usize) {
           let isolate = Isolate::new();
           self.array_builder_.ensure_capacity(&isolate, elements);
        }
    }

    #[derive(Debug, Clone)]
    pub struct Heap {}

    impl Heap {
        pub fn new() -> Self {
            Heap {}
        }
    }

    const K_INITIAL_PART_LENGTH: usize = 32;
    const K_MAX_PART_LENGTH: usize = 16 * 1024;
    const K_PART_LENGTH_GROWTH_FACTOR: usize = 2;
    const K_INT_TO_STRING_VIEW_BUFFER_SIZE: usize = 12; //sizeof("-2147483648") - 1;

    #[derive(Debug)]
    pub struct IncrementalStringBuilder {
        isolate_: Isolate,
        encoding_: Encoding,
        overflowed_: bool,
        part_length_: usize,
        current_index_: usize,
        accumulator_: DirectHandle<String>,
        current_part_: DirectHandle<String>,
    }

    impl IncrementalStringBuilder {
        pub fn new(isolate: &Isolate) -> Self {
            let initial_part = String {
                data: StdString::with_capacity(K_INITIAL_PART_LENGTH),
                encoding: Encoding::Utf8,
            };

            IncrementalStringBuilder {
                isolate_: isolate.clone(),
                encoding_: Encoding::Utf8,
                overflowed_: false,
                part_length_: K_INITIAL_PART_LENGTH,
                current_index_: 0,
                accumulator_: DirectHandle::new(String {
                    data: StdString::new(),
                    encoding: Encoding::Utf8,
                }),
                current_part_: DirectHandle::new(initial_part),
            }
        }

        pub fn current_encoding(&self) -> Encoding {
            self.encoding_
        }

        pub fn append<SrcChar, DestChar>(&mut self, c: SrcChar)
        where
            SrcChar: Into<u32>,
            DestChar: From<u32>,
        {
            let c_u32: u32 = c.into();

            if self.current_index_ >= self.part_length_ {
                self.extend();
            }

            if self.encoding_ == Encoding::Utf8 {
                 if let Ok(c_u8) = c_u32.try_into() {
                    self.current_part_.value().data.push(c_u8 as char);
                    self.current_index_ += 1;
                 }
                 else {
                    self.change_encoding();
                     if let Ok(c_u16) = c_u32.try_into() {
                        if let Some(ch) = char::from_u32(c_u32) {
                            self.current_part_.value().data.push(ch);
                            self.current_index_ += 1;
                        }
                        else {
                            self.overflowed_ = true;
                        }
                    }
                     else {
                         self.overflowed_ = true;
                     }
                 }
            } else {
               if let Some(ch) = char::from_u32(c_u32) {
                    self.current_part_.value().data.push(ch);
                    self.current_index_ += 1;
                }
                else {
                    self.overflowed_ = true;
                }
            }
        }

        pub fn append_character(&mut self, c: u8) {
            if self.current_index_ >= self.part_length_ {
                self.extend();
            }

            self.current_part_.value().data.push(c as char);
            self.current_index_ += 1;

           
        }

        pub fn append_cstring_literal<const N: usize>(&mut self, literal: &[u8; N]) {
            let s = std::str::from_utf8(literal).unwrap();
            self.append_cstring(s);
        }

        pub fn append_cstring<SrcChar>(&mut self, s: &str)
        {
            for c in s.chars() {
                self.append::<char, char>(c);
            }
        }
    
        pub fn append_string(&mut self, str: &str) {
            for c in str.chars() {
                self.append::<char, char>(c);
            }
        }

        pub fn append_int(&mut self, i: i32) {
            let s = i.to_string();
            self.append_cstring(&s);
        }

        pub fn current_part_can_fit(&self, length: usize) -> bool {
            self.part_length_ - self.current_index_ > length
        }

        pub fn escaped_length_if_current_part_fits(&self, length: usize) -> i32 {
            if self.current_part_can_fit(length) {
                length as i32
            } else {
                -1
            }
        }

        pub fn append_string_handle(&mut self, string: DirectHandle<String>) {
            if self.can_append_by_copy(string.clone()) {
                self.append_string_by_copy(string);
            } else {
                let data = string.value().data.clone();
                self.append_string(&data);
            }
        }

        pub fn finish(&mut self) -> Result<DirectHandle<String>, &'static str> {
            self.shrink_current_part();
            self.accumulate(self.current_part_.clone());
            let result = self.accumulator_.clone();
            self.accumulator_.set_value(String {
                data: StdString::new(),
                encoding: Encoding::Utf8,
            }); // Reset the accumulator.
            Ok(result)
        }

        pub fn has_overflowed(&self) -> bool {
            self.overflowed_
        }

        pub fn length(&self) -> usize {
            let accumulator_len = self.accumulator_.value().data.len();
            let current_part_len = self.current_part_.value().data.len();
            accumulator_len + current_part_len
        }

        pub fn change_encoding(&mut self) {
            if self.encoding_ != Encoding::Utf16 {
                self.encoding_ = Encoding::Utf16;
            }
        }

        fn factory(&self) -> Isolate {
            self.isolate_.clone()
        }

        fn accumulate(&mut self, new_part: DirectHandle<String>) {
            let mut acc_data = self.accumulator_.value().data.clone();
            acc_data.push_str(&new_part.value().data);
            self.accumulator_.set_value(String {
                data: acc_data,
                encoding: self.encoding_,
            });
            self.current_index_ = 0;
        }

        fn extend(&mut self) {
            self.shrink_current_part();
            self.accumulate(self.current_part_.clone());

            let new_part_length = std::cmp::min(
                self.part_length_ * K_PART_LENGTH_GROWTH_FACTOR,
                K_MAX_PART_LENGTH,
            );

            let new_part = String {
                data: StdString::with_capacity(new_part_length),
                encoding: self.encoding_,
            };

            self.part_length_ = new_part_length;
            self.current_part_.set_value(new_part);
            self.current_index_ = 0;
        }

        fn has_valid_current_index(&self) -> bool {
            self.current_index_ <= self.part_length_
        }

        fn shrink_current_part(&mut self) {
             let current_part = self.current_part_.value().data.clone();
             let shrunk_string = String {
                 data: current_part[0..self.current_index_].to_string(),
                 encoding: self.encoding_,
             };
            
             self.current_part_.set_value(shrunk_string);
        }

        fn append_string_by_copy(&mut self, string: DirectHandle<String>) {
            let mut current_data = self.current_part_.value().data.clone();
            current_data.push_str(&string.value().data);
            self.current_part_.set_value(String {
                data: current_data,
                encoding: self.encoding_,
            });
            self.current_index_ += string.value().len();
        }

        fn can_append_by_copy(&self, string: DirectHandle<String>) -> bool {
            self.current_part_can_fit(string.value().len())
        }
    }
}