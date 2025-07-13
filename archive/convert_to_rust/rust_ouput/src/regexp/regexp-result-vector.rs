// Converted from V8 C++ source files:
// Header: regexp-result-vector.h
// Implementation: regexp-result-vector.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/regexp/regexp-result-vector.h

use crate::execution::isolate::Isolate;
use crate::regexp::regexp_dotprinter::AllStatic;

pub struct RegExpResultVectorScope<'a> {
    isolate: &'a mut Isolate,
    is_dynamic: bool,
    value: Option<Box<[i32]>>,
}

impl<'a> RegExpResultVectorScope<'a> {
    pub fn new(isolate: &'a mut Isolate) -> Self {
        RegExpResultVectorScope {
            isolate,
            is_dynamic: false,
            value: None,
        }
    }

    pub fn with_size(isolate: &'a mut Isolate, size: usize) -> Self {
        let mut scope = RegExpResultVectorScope::new(isolate);
        scope.initialize(size);
        scope
    }

    pub fn initialize(&mut self, size: usize) {
        if self.value.is_some() {
            panic!("RegExpResultVectorScope::Initialize called twice");
        }

        let static_vector_or_null = self.isolate.regexp_static_result_offsets_vector.take();

        if size > Isolate::kJSRegexpStaticOffsetsVectorSize || static_vector_or_null.is_none() {
            self.is_dynamic = true;
            self.value = Some(RegExpResultVector::allocate(self.isolate, size).into_boxed_slice());
        } else {
            self.value = static_vector_or_null;
            // Take ownership of the static vector. See also:
            // RegExpBuiltinsAssembler::TryLoadStaticRegExpResultVector.
            self.isolate.regexp_static_result_offsets_vector = None;
        }

        if self.value.is_none() {
            panic!("Value is still None after initialization")
        }
    }

    pub fn value(&self) -> &Box<[i32]> {
        self.value.as_ref().expect("Value should not be null")
    }
}

impl<'a> Drop for RegExpResultVectorScope<'a> {
    fn drop(&mut self) {
        if self.is_dynamic {
            if let Some(vector) = self.value.take() {
                RegExpResultVector::free(self.isolate, vector.into_vec());
            }
        } else if self.value.is_some() {
            // Return ownership of the static vector.
            self.isolate.regexp_static_result_offsets_vector = self.value.take();
        } else {
            // The scope was created but Initialize never called. Nothing to do.
        }
    }
}

pub struct RegExpResultVector {}

impl RegExpResultVector {
    pub fn allocate(isolate: &mut Isolate, size: usize) -> Vec<i32> {
        //DisallowGarbageCollection no_gc;  // Assuming DisallowGarbageCollection is not needed in Rust.
        let mut vector = vec![0i32; size];
        isolate.active_dynamic_regexp_result_vectors.insert(vector.as_ptr() as usize); // Store as usize
        vector
    }

    pub fn free(isolate: &mut Isolate, vector: Vec<i32>) {
        //DisallowGarbageCollection no_gc;
        let ptr = vector.as_ptr() as usize;
        isolate.active_dynamic_regexp_result_vectors.remove(&ptr);
        //vector will be deallocated by rust at the end of the scope
    }
}
