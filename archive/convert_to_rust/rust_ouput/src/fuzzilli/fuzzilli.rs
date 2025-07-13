// Converted from V8 C++ source files:
// Header: fuzzilli.h
// Implementation: fuzzilli.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod strings {
        use std::ffi::CString;

        pub fn SNPrintF(buf: crate::base::strings::VectorOf, format: &str, arg: &str) -> i32 {
            let formatted = format.replace("%s", arg);
            let c_string = CString::new(formatted).unwrap();
            let bytes = c_string.as_bytes_with_nul();

            if bytes.len() <= buf.size {
                buf.buffer[..bytes.len()].copy_from_slice(bytes);
                bytes.len() as i32 - 1 // Exclude null terminator from count.
            } else {
                // Indicate truncation by returning negative value.
                -1
            }
        }

        pub struct VectorOf {
            pub buffer: &'static mut [u8],
            pub size: usize,
        }
    }
}

pub mod include {
    pub mod v8 {
        use std::any::Any;
        use std::os::raw::c_void;
        use std::ptr;
        use std::rc::Rc;

        pub struct Isolate {}

        impl Isolate {
            pub fn GetCurrentContext(&mut self) -> Local<Context> {
                Local::new(Context {})
            }
        }

        pub struct Value {}
        pub struct String {}

        impl String {
            pub fn Utf8Value<'a>(_isolate: &Isolate, val: &Value) -> Utf8ValueResult {
                Utf8ValueResult {
                    value: "FUZZILLI_CRASH".to_string()
                }
            }
        }

        #[derive(Debug)]
        pub struct Utf8ValueResult {
            pub value: String,
        }

        impl Utf8ValueResult {
            pub fn as_ptr(&self) -> *const i8 {
                self.value.as_ptr() as *const i8
            }
        }

        pub struct FunctionCallbackInfo<T> {
            pub isolate: *mut Isolate,
            pub args: Vec<Local<Value>>,
        }

        impl<T> FunctionCallbackInfo<T> {
            pub fn GetIsolate(&mut self) -> *mut Isolate {
                self.isolate
            }
        }

        pub struct FunctionTemplate {}

        impl FunctionTemplate {
            pub fn New(
                _isolate: *mut Isolate,
                callback: fn(&FunctionCallbackInfo<Value>),
            ) -> Local<FunctionTemplate> {
                Local::new(FunctionTemplate {})
            }
        }

        pub struct Context {}

        pub struct Local<T> {
            pub value: Rc<T>,
        }

        impl<T> Local<T> {
            pub fn new(value: T) -> Self {
                Local {
                    value: Rc::new(value),
                }
            }
        }

        pub trait ToDetailString {
            fn ToDetailString(&self, _context: &mut Context) -> String {
                "Generic".to_string()
            }
        }

        impl ToDetailString for Value {
            fn ToDetailString(&self, _context: &mut Context) -> String {
                "Value".to_string()
            }
        }

        impl Local<Value> {
            pub fn ToDetailString(&self, context: &mut Context) -> String {
                self.value.ToDetailString(context)
            }

            pub fn Int32Value(&self, _context: &Context) -> Maybe<i32> {
                Maybe { value: 0 }
            }
        }

        pub struct Maybe<T> {
            pub value: T,
        }

        impl<T> Maybe<T> {
            pub fn FromMaybe(self, default_value: T) -> T {
                self.value
            }
        }

        pub struct Extension {
            name: &'static str,
            source: &'static str,
        }

        impl Extension {
            pub fn new(name: &'static str, source: &'static str) -> Self {
                Extension {
                    name,
                    source,
                }
            }
        }

        pub struct String_ExternalOneByteStringResource {}
    }
}

pub mod src {
    pub mod fuzzilli {
        use crate::include::v8;
        use std::ffi::CString;
        use std::os::raw::c_char;

        pub struct FuzzilliExtension {
            extension: v8::Extension,
            buffer: [char; 50],
        }

        impl FuzzilliExtension {
            pub fn new(fun_name: &str) -> Self {
                let mut buffer: [char; 50] = ['\0'; 50];
                let source = Self::build_source(&mut buffer, fun_name);
                let extension = v8::Extension::new("v8/fuzzilli", source);
                FuzzilliExtension {
                    extension,
                    buffer,
                }
            }

            pub fn get_native_function_template(
                &self,
                isolate: *mut v8::Isolate,
                name: v8::Local<v8::String>,
            ) -> v8::Local<v8::FunctionTemplate> {
                v8::FunctionTemplate::New(isolate, Self::fuzzilli)
            }

            fn build_source(buf: &mut [char; 50], fun_name: &str) -> &'static str {
                use crate::base::strings::SNPrintF;
                use crate::base::strings::VectorOf;

                let format_string = "native function %s();";
                let mut vec_of = VectorOf {
                    buffer: unsafe { std::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut u8, buf.len()) },
                    size: buf.len(),
                };

                SNPrintF(vec_of, format_string, fun_name);

                let ptr = buf.as_ptr();
                let len = buf.len();
                let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
                let str_result = std::str::from_utf8(slice).unwrap();

                str_result
            }

            pub fn fuzzilli(info: &v8::FunctionCallbackInfo<v8::Value>) {
                use std::cmp::PartialEq;
                use std::ffi::CStr;
                use std::os::raw::c_char;

                let isolate_ptr = info.GetIsolate();
                let mut isolate = unsafe { &mut *isolate_ptr };

                let operation_value = &info.args[0];
                let operation = v8::String::Utf8Value(&isolate, operation_value);

                if operation.value.is_empty() {
                    return;
                }

                if operation.value == "FUZZILLI_CRASH" {
                    let context = isolate.GetCurrentContext();
                    let arg_value = &info.args[1];
                    let arg = arg_value.Int32Value(&context).FromMaybe(0);

                    match arg {
                        0 => {
                            std::process::abort();
                        }
                        1 => {
                            assert!(false);
                        }
                        2 => {
                            debug_assert!(false);
                        }
                        3 => {
                            let addr: usize = 0x414141414141;
                            let ptr = addr as *mut char;
                            for i in 0..1024 {
                                unsafe {
                                    *ptr = 'A';
                                }
                            }
                        }
                        4 => {
                            let mut vec = vec![0i32; 4];
                            // No explicit deletion in Rust, UAF is harder.
                            let _val = vec[0];
                        }
                        5 => {
                            let vec = vec![0i32; 5];
                            let _val = vec[5];
                        }
                        6 => {
                            let mut vec = vec![0i32; 6];
                            for i in 0..vec.len() {
                                vec[i] = 42;
                            }
                        }
                        7 => {
                            if false {
                                let addr: usize = 0x414141414141;
                                let ptr = addr as *mut char;
                                for i in 0..1024 {
                                    unsafe {
                                        *ptr = 'A';
                                    }
                                }
                            }
                        }
                        8 => {
                            if cfg!(debug_assertions) {
                                std::process::abort();
                            }
                        }
                        _ => {}
                    }
                } else if operation.value == "FUZZILLI_PRINT" {
                    use std::fs::File;
                    use std::io::Write;

                    let mut fzliout = unsafe { File::from_raw_fd(103) };

                    let string_value = &info.args[1];
                    let string = v8::String::Utf8Value(&isolate, string_value);
                    let string_slice: &str = &string.value;

                    match fzliout.write_all(string_slice.as_bytes()) {
                        Ok(_) => {
                            fzliout.flush().unwrap();
                        }
                        Err(_) => {
                            eprintln!("Fuzzer output channel not available, printing to stdout instead");
                            println!("{}", string_slice);
                        }
                    }
                }
            }
        }
    }
}

const REPRL_CRFD: i32 = 100;
const REPRL_CWFD: i32 = 101;
const REPRL_DRFD: i32 = 102;
const REPRL_DWFD: i32 = 103;
