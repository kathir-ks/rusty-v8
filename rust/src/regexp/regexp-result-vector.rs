pub mod regexp_result_vector {
    use std::collections::HashSet;
    use std::ptr::NonNull;
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::sync::Mutex;

    /// Represents the V8 isolate.  This is a simplified version for
    /// demonstration.  A real implementation would need to interface with
    /// the V8 API correctly.
    pub struct Isolate {
        regexp_static_result_offsets_vector: AtomicPtr<i32>,
        active_dynamic_regexp_result_vectors: Mutex<HashSet<NonNull<i32>>>,
    }

    impl Isolate {
        pub fn new() -> Self {
            Isolate {
                regexp_static_result_offsets_vector: AtomicPtr::new(std::ptr::null_mut()),
                active_dynamic_regexp_result_vectors: Mutex::new(HashSet::new()),
            }
        }

        pub fn regexp_static_result_offsets_vector(&self) -> *mut i32 {
            self.regexp_static_result_offsets_vector.load(Ordering::Relaxed)
        }

        pub fn set_regexp_static_result_offsets_vector(&self, value: *mut i32) {
            self.regexp_static_result_offsets_vector.store(value, Ordering::Relaxed);
        }

        pub fn active_dynamic_regexp_result_vectors(&self) -> &Mutex<HashSet<NonNull<i32>>> {
            &self.active_dynamic_regexp_result_vectors
        }
    }

    /// RAII scope for managing RegExp result vectors.
    pub struct RegExpResultVectorScope<'a> {
        isolate_: &'a Isolate,
        value_: *mut i32,
        is_dynamic_: bool,
    }

    impl<'a> RegExpResultVectorScope<'a> {
        pub fn new(isolate: &'a Isolate) -> Self {
            RegExpResultVectorScope {
                isolate_: isolate,
                value_: std::ptr::null_mut(),
                is_dynamic_: false,
            }
        }

        pub fn with_size(isolate: &'a Isolate, size: usize) -> Self {
            let mut scope = RegExpResultVectorScope::new(isolate);
            scope.initialize(size);
            scope
        }

        pub fn initialize(&mut self, size: usize) -> *mut i32 {
            assert!(self.value_.is_null());
            const JS_REGEXP_STATIC_OFFSETS_VECTOR_SIZE: usize = 64; //Example Value

            let static_vector_or_null = self.isolate_.regexp_static_result_offsets_vector();

            if size > JS_REGEXP_STATIC_OFFSETS_VECTOR_SIZE || static_vector_or_null.is_null() {
                self.is_dynamic_ = true;
                self.value_ = RegExpResultVector::allocate(self.isolate_, size);
            } else {
                self.value_ = static_vector_or_null;
                // Take ownership of the static vector.
                self.isolate_
                    .set_regexp_static_result_offsets_vector(std::ptr::null_mut());
            }
            assert!(!self.value_.is_null());
            self.value_
        }
    }

    impl<'a> Drop for RegExpResultVectorScope<'a> {
        fn drop(&mut self) {
            if self.is_dynamic_ {
                RegExpResultVector::free(self.isolate_, self.value_);
            } else if !self.value_.is_null() {
                // Return ownership of the static vector.
                self.isolate_
                    .set_regexp_static_result_offsets_vector(self.value_);
            } else {
                // The scope was created but Initialize never called. Nothing to do.
            }
        }
    }

    pub struct RegExpResultVector {}

    impl RegExpResultVector {
        pub fn allocate(isolate: &Isolate, size: usize) -> *mut i32 {
            let vector = unsafe {
                let layout = std::alloc::Layout::array::<i32>(size).unwrap();
                let ptr = std::alloc::alloc(layout) as *mut i32;
                if ptr.is_null() {
                    std::alloc::handle_alloc_error(layout);
                }
                ptr
            };

            let mut guard = isolate.active_dynamic_regexp_result_vectors().lock().unwrap();
            guard.insert(NonNull::new(vector).unwrap());

            vector
        }

        pub fn free(isolate: &Isolate, vector: *mut i32) {
            assert!(!vector.is_null());

            let mut guard = isolate.active_dynamic_regexp_result_vectors().lock().unwrap();
            guard.remove(&NonNull::new(vector).unwrap());

            unsafe {
                let layout = std::alloc::Layout::array::<i32>(0).unwrap(); //Placeholder to avoid crash.
                std::alloc::dealloc(vector as *mut u8, layout); //Original size is lost during allocation!
            }
        }
    }
}