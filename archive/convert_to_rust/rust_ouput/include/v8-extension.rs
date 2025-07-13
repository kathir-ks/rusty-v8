// Converted from V8 C++ source files:
// Header: v8-extension.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8 {
    pub struct Extension {
        name_: *const i8,
        source_length_: usize,
        source_: *mut String_ExternalOneByteStringResource,
        dep_count_: i32,
        deps_: *const *const i8,
        auto_enable_: bool,
    }

    impl Extension {
        pub fn new(
            name: *const i8,
            source: *const i8,
            dep_count: i32,
            deps: *const *const i8,
            source_length: i32,
        ) -> Self {
            let source_length_usize = if source_length == -1 {
                if source.is_null() {
                    0
                } else {
                    unsafe {
                        let mut len: usize = 0;
                        while *source.offset(len as isize) != 0 {
                            len += 1;
                        }
                        len
                    }
                }
            } else {
                source_length as usize
            };

            let source_ptr = if !source.is_null() {
                let source_string = unsafe {
                    let slice = std::slice::from_raw_parts(source as *const u8, source_length_usize);
                    String::from_utf8_lossy(slice).into_owned()
                };
                let resource = Box::new(String_ExternalOneByteStringResource {
                    data_: source_string.as_ptr() as *const i8,
                    length_: source_string.len(),
                    string_: source_string,
                });
                Box::into_raw(resource)
            } else {
                std::ptr::null_mut()
            };

            Extension {
                name_: name,
                source_length_: source_length_usize,
                source_: source_ptr,
                dep_count_: dep_count,
                deps_: deps,
                auto_enable_: false,
            }
        }

        pub fn get_native_function_template(
            &self,
            _isolate: *mut Isolate,
            _name: Local<'static, String>,
        ) -> Local<'static, FunctionTemplate> {
            Local::empty()
        }

        pub fn name(&self) -> *const i8 {
            self.name_
        }

        pub fn source_length(&self) -> usize {
            self.source_length_
        }

        pub fn source(&self) -> *const String_ExternalOneByteStringResource {
            self.source_
        }

        pub fn dependency_count(&self) -> i32 {
            self.dep_count_
        }

        pub fn dependencies(&self) -> *const *const i8 {
            self.deps_
        }

        pub fn set_auto_enable(&mut self, value: bool) {
            self.auto_enable_ = value;
        }

        pub fn auto_enable(&self) -> bool {
            self.auto_enable_
        }
    }

    impl Drop for Extension {
        fn drop(&mut self) {
            if !self.source_.is_null() {
                unsafe {
                    drop(Box::from_raw(self.source_));
                }
                self.source_ = std::ptr::null_mut();
            }
        }
    }

    struct String_ExternalOneByteStringResource {
        data_: *const i8,
        length_: usize,
        string_: String,
    }

    pub struct Isolate {}
    pub struct Local<'a, T> {
        _phantom: std::marker::PhantomData<&'a T>,
    }

    impl<'a, T> Local<'a, T> {
        pub fn empty() -> Self {
            Local {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    pub struct String {}

    pub fn register_extension(extension: std::unique_ptr<Extension>) {}

    pub mod std {
        pub mod unique_ptr {
            pub struct UniquePtr<T> {
                ptr: *mut T,
            }

            impl<T> UniquePtr<T> {
                pub fn new(ptr: *mut T) -> Self {
                    UniquePtr { ptr }
                }
            }

            impl<T> Drop for UniquePtr<T> {
                fn drop(&mut self) {
                    if !self.ptr.is_null() {
                        unsafe {
                            drop(Box::from_raw(self.ptr));
                        }
                    }
                }
            }
        }
    }
}
