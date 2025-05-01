// src/strings/string_stream.rs

use std::fmt;
use std::fmt::Write;
use std::mem;
use std::ptr;
use std::vec::Vec;
//use libc::FILE; // Assuming FILE is used for file output, requires libc crate
//use std::ffi::CString; // For converting Rust strings to C-style strings if needed

const K_MENTIONED_OBJECT_CACHE_MAX_SIZE: usize = 256;

/// Allocator that uses the heap to allocate strings.
pub struct HeapStringAllocator {
    space: Vec<u8>,
}

impl HeapStringAllocator {
    pub fn new() -> Self {
        HeapStringAllocator { space: Vec::new() }
    }

    pub fn allocate(&mut self, bytes: usize) -> *mut u8 {
        self.space = vec![0u8; bytes];
        self.space.as_mut_ptr()
    }

    pub fn grow(&mut self, bytes: &mut usize) -> *mut u8 {
        let mut new_bytes = *bytes * 2;
        // Check for overflow.
        if new_bytes <= *bytes {
            return self.space.as_mut_ptr();
        }
        let mut new_space = vec![0u8; new_bytes];
        if new_space.is_empty() {
            return self.space.as_mut_ptr();
        }
        unsafe {
            ptr::copy_nonoverlapping(self.space.as_ptr(), new_space.as_mut_ptr(), *bytes);
        }
        *bytes = new_bytes;
        self.space = new_space;
        self.space.as_mut_ptr()
    }
}

/// Allocator that uses a fixed-size buffer to allocate strings.
pub struct FixedStringAllocator {
    buffer: *mut u8,
    length: usize,
}

impl FixedStringAllocator {
    pub fn new(buffer: *mut u8, length: usize) -> Self {
        FixedStringAllocator { buffer, length }
    }

    pub fn allocate(&self, bytes: usize) -> *mut u8 {
        if bytes > self.length {
            panic!("Allocation size exceeds buffer length"); // Or return Result::Err
        }
        self.buffer
    }

    pub fn grow(&mut self, old: &mut usize) -> *mut u8 {
        *old = self.length;
        self.buffer
    }
}

/// A stream for building strings.
pub struct StringStream<'a> {
    buffer: *mut u8,
    length: usize,
    capacity: usize,
    allocator: &'a mut dyn StringAllocator,
    object_print_mode: ObjectPrintMode,
    //isolate: Option<*mut Isolate>, //TODO
    //security_token_current: Option<Tagged<Object>>, //TODO
    //object_cache: Option<DebugObjectCache>, //TODO
}

#[derive(PartialEq, Eq)]
pub enum ObjectPrintMode {
    Concise,
    Verbose,
}

pub trait StringAllocator {
    fn allocate(&mut self, bytes: usize) -> *mut u8;
    fn grow(&mut self, old: &mut usize) -> *mut u8;
}

impl StringStream<'_> {
    pub fn new(
        allocator: &mut dyn StringAllocator,
        initial_capacity: usize,
        object_print_mode: ObjectPrintMode,
    ) -> Self {
        let buffer = allocator.allocate(initial_capacity);
        unsafe {
            *buffer = 0;
        }
        StringStream {
            buffer,
            length: 0,
            capacity: initial_capacity,
            allocator,
            object_print_mode,
            //isolate: None, //TODO
            //security_token_current: None, //TODO
            //object_cache: None, //TODO
        }
    }

    pub fn put(&mut self, c: char) -> bool {
        if self.full() {
            return false;
        }
        if self.length == self.capacity - 2 {
            let mut new_capacity = self.capacity;
            let new_buffer = self.allocator.grow(&mut new_capacity);
            if new_capacity > self.capacity {
                self.capacity = new_capacity;
                self.buffer = new_buffer;
            } else {
                if self.capacity < 5 {
                    panic!("Capacity is too small");
                }
                self.length = self.capacity - 1;
                unsafe {
                    *self.buffer.add(self.length - 4) = b'.';
                    *self.buffer.add(self.length - 3) = b'.';
                    *self.buffer.add(self.length - 2) = b'.';
                    *self.buffer.add(self.length - 1) = b'\n';
                    *self.buffer.add(self.length) = 0;
                }
                return false;
            }
        }
        unsafe {
            *self.buffer.add(self.length) = c as u8;
            *self.buffer.add(self.length + 1) = 0;
        }
        self.length += 1;
        true
    }

    pub fn add(&mut self, s: &str) {
        for c in s.chars() {
            if !self.put(c) {
                return;
            }
        }
    }

    fn full(&self) -> bool {
        self.length >= self.capacity - 1
    }

    fn is_control_char(c: char) -> bool {
        matches!(c, '0'..='9' | '.' | '-')
    }

    pub fn add_formatted(&mut self, format: &str, elms: &[FmtElm]) {
        if self.full() {
            return;
        }

        let mut offset = 0;
        let mut elm = 0;

        while offset < format.len() {
            if format.as_bytes()[offset] != b'%' as u8 || elm == elms.len() {
                self.put(format.chars().nth(offset).unwrap());
                offset += 1;
                continue;
            }

            let mut temp = String::new();
            let mut format_length = 0;

            temp.push(format.chars().nth(offset).unwrap());
            offset += 1;
            format_length += 1;

            while offset < format.len() && Self::is_control_char(format.chars().nth(offset).unwrap()) {
                temp.push(format.chars().nth(offset).unwrap());
                offset += 1;
                format_length += 1;
            }

            if offset >= format.len() {
                return;
            }

            let r#type = format.chars().nth(offset).unwrap();
            temp.push(r#type);
            format_length += 1;

            offset += 1;

            let current = elms[elm];
            elm += 1;

            match r#type {
                's' => {
                    if let FmtElmType::CStr = current.type_ {
                        let value = current.data.u_c_str;
                        self.add(value);
                    } else {
                        panic!("Type mismatch for 's'");
                    }
                }
                'w' => {
                    if let FmtElmType::LCStr = current.type_ {
                        let value = current.data.u_lc_str;
                        for i in 0..value.len() {
                            self.put(value[i] as u8 as char);
                        }
                    } else {
                        panic!("Type mismatch for 'w'");
                    }
                }
                'o' => {
                    if let FmtElmType::Obj = current.type_ {
                        //TODO: Tagged Object printing
                        //let obj = Tagged::<Object>::new(current.data.u_obj);
                        //self.print_object(obj);
                         self.add("TODO: Object");
                    } else {
                        panic!("Type mismatch for 'o'");
                    }
                }
                'k' => {
                    if let FmtElmType::Int = current.type_ {
                        let value = current.data.u_int;
                        if (0x20..=0x7F).contains(&value) {
                            self.put(value as u8 as char);
                        } else if value <= 0xFF {
                            self.add(&format!("\\x{:02x}", value));
                        } else {
                            self.add(&format!("\\u{:04x}", value));
                        }
                    } else {
                        panic!("Type mismatch for 'k'");
                    }
                }
                'i' | 'd' | 'u' | 'x' | 'c' | 'X' => {
                    if let FmtElmType::Int = current.type_ {
                        let value = current.data.u_int;
                        let formatted = format!(temp.as_str(), value);
                        self.add(&formatted);
                    } else {
                        panic!("Type mismatch for integer formatting");
                    }
                }
                'f' | 'g' | 'G' | 'e' | 'E' => {
                    if let FmtElmType::Double = current.type_ {
                        let value = current.data.u_double;
                        if value.is_infinite() {
                            if value.is_sign_negative() {
                                self.add("-inf");
                            } else {
                                self.add("inf");
                            }
                        } else if value.is_nan() {
                            self.add("nan");
                        } else {
                            let formatted = format!(temp.as_str(), value);
                            self.add(&formatted);
                        }
                    } else {
                        panic!("Type mismatch for float formatting");
                    }
                }
                'p' => {
                    if let FmtElmType::Pointer = current.type_ {
                        let value = current.data.u_pointer;
                        let formatted = format!(temp.as_str(), value as usize);
                        self.add(&formatted);
                    } else {
                        panic!("Type mismatch for pointer formatting");
                    }
                }
                _ => panic!("Unsupported format type: {}", r#type),
            }
        }

        unsafe {
            assert_eq!(*self.buffer.add(self.length), 0);
        }
    }
    /*
    fn print_object(&mut self, o: Tagged<Object>) {
        //ShortPrint(o, self); //TODO: Implement short print

        //TODO: Implement String, Number, Oddball

        if /*IsHeapObject(o) &&*/ self.object_print_mode == ObjectPrintMode::Verbose {
            //TODO(delphick): Consider whether we can get the isolate without using
            // TLS.
            //Isolate* isolate = Isolate::Current(); //TODO
            //DebugObjectCache* debug_object_cache = //TODO
            //    isolate->string_stream_debug_object_cache(); //TODO

            //TODO: Implement DebugObjectCache and MentionedObjects
            //for i in 0..debug_object_cache.size() {
            //    if *debug_object_cache[i] == o {
            //        self.add(&format!("#{}#", i));
            //        return;
            //    }
            //}
            //if debug_object_cache.size() < K_MENTIONED_OBJECT_CACHE_MAX_SIZE {
            //    self.add(&format!("#{}#", debug_object_cache.size()));
            //    debug_object_cache.push_back(handle(Cast<HeapObject>(o), isolate)); //TODO
            //} else {
            //    self.add(&format!("@{:p}", o));
            //}
        }
    }
    */

    pub fn to_c_string(&self) -> Vec<u8> {
        let mut result = vec![0u8; self.length + 1];
        unsafe {
            ptr::copy_nonoverlapping(self.buffer, result.as_mut_ptr(), self.length);
        }
        result[self.length] = 0;
        result
    }

    /*
    pub fn log(&self, isolate: *mut Isolate) { //TODO
        //LOG(isolate, StringEvent("StackDump", buffer_)); //TODO
    }

    pub fn output_to_file(&self, out: *mut FILE) {
        //TODO: output file implementation
        let string = self.to_c_string();
        println!("{:?}", string);
    }
    */
    /*
    pub fn to_string(&self, isolate: *mut Isolate) -> String { //TODO
        //return isolate->factory() //TODO
        //    ->NewStringFromUtf8(base::Vector<const char>(buffer_, length_)) //TODO
        //    .ToHandleChecked(); //TODO
        String::from_utf8(self.to_c_string()).unwrap()
    }
    */

    /*
    pub fn clear_mentioned_object_cache(&mut self, isolate: *mut Isolate) {
      //TODO
    }

    #[cfg(debug_assertions)]
    pub fn is_mentioned_object_cache_clear(&self, isolate: *mut Isolate) -> bool {
        self.object_print_mode == ObjectPrintMode::Concise
    }
    */
    /*
    pub fn put_string(&mut self, str_: Tagged<String>) -> bool { //TODO
      self.put_string_range(str_, 0, str_.length())
    }

    pub fn put_string_range(&mut self, str_: Tagged<String>, start: i32, end: i32) -> bool { //TODO
      let mut stream = StringCharacterStream::new(str_, start); //TODO
      let mut i = start;
      while i < end {
        let c = stream.get_next() as u8;
        let char_to_print = if c >= 127 || c < 32 { '?' } else { c as char };

        if !self.put(char_to_print) {
          return false;
        }
        i += 1;
      }
      return true;
    }

    pub fn print_name(&mut self, name: Tagged<Object>) { //TODO
        if name.is_string() {
            let str_ = Cast::<String>(name);
            if str_.length() > 0 {
              self.put_string(str_);
            } else {
              self.add("/* anonymous */");
            }
        } else {
            self.add(&format!("%o", name));
        }
    }
    */
    /*
    pub fn print_using_map(&mut self, isolate: *mut Isolate, js_object: Tagged<JSObject>) { //TODO
        //TODO: Implement printing using map
    }

    pub fn print_fixed_array(&mut self, array: Tagged<FixedArray>, limit: u32) { //TODO
      //TODO: Implement fixed array printing
    }

    pub fn print_byte_array(&mut self, byte_array: Tagged<ByteArray>) { //TODO
      //TODO: Implement byte array printing
    }

    pub fn print_mentioned_object_cache(&mut self, isolate: *mut Isolate) { //TODO
      //TODO: Implement object cache printing
    }

    pub fn print_security_token_if_changed(&mut self, isolate: *mut Isolate, fun: Tagged<JSFunction>) { //TODO
      //TODO: Security token
    }

    pub fn print_function(&mut self, isolate: *mut Isolate, fun: Tagged<JSFunction>, receiver: Tagged<Object>) { //TODO
      //TODO: Function printing
    }

    pub fn print_prototype(&mut self, isolate: *mut Isolate, fun: Tagged<JSFunction>, receiver: Tagged<Object>) { //TODO
      //TODO: Prototype printing
    }
    */
}

impl fmt::Write for StringStream<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.add(s);
        Ok(())
    }
}

impl Drop for StringStream<'_> {
    fn drop(&mut self) {
        //drop(self.buffer);
    }
}

#[derive(Copy, Clone)]
pub struct FmtElm {
    pub type_: FmtElmType,
    pub data: FmtElmData,
}

#[derive(Copy, Clone)]
pub enum FmtElmType {
    CStr,
    LCStr,
    Obj,
    Int,
    Double,
    Pointer,
}

#[derive(Copy, Clone)]
pub union FmtElmData {
    pub u_c_str: *const char,
    pub u_lc_str: *const Vec<u16>,
    pub u_obj: *mut u8, // Tagged<Object>,
    pub u_int: i32,
    pub u_double: f64,
    pub u_pointer: *mut std::ffi::c_void,
}

impl FmtElmData {
    pub fn new_c_str(ptr: *const char) -> Self {
        FmtElmData { u_c_str: ptr }
    }

    pub fn new_lc_str(ptr: *const Vec<u16>) -> Self {
        FmtElmData { u_lc_str: ptr }
    }

    pub fn new_obj(ptr: *mut u8) -> Self {
        FmtElmData { u_obj: ptr }
    }

    pub fn new_int(value: i32) -> Self {
        FmtElmData { u_int: value }
    }

    pub fn new_double(value: f64) -> Self {
        FmtElmData { u_double: value }
    }

    pub fn new_pointer(ptr: *mut std::ffi::c_void) -> Self {
        FmtElmData { u_pointer: ptr }
    }
}