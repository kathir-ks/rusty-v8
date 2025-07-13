// Converted from V8 C++ source files:
// Header: string-stream.h
// Implementation: string-stream.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct Vector<T>(Vec<T>);

    impl<T> Vector<T> {
        pub fn new() -> Self {
            Vector(Vec::new())
        }

        pub fn with_capacity(capacity: usize) -> Self {
            Vector(Vec::with_capacity(capacity))
        }

        pub fn push(&mut self, value: T) {
            self.0.push(value);
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            self.0.get(index)
        }

        pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            self.0.get_mut(index)
        }

        pub fn insert(&mut self, index: usize, element: T) {
            self.0.insert(index, element);
        }

        pub fn remove(&mut self, index: usize) -> T {
            self.0.remove(index)
        }

        pub fn clear(&mut self) {
            self.0.clear();
        }

        pub fn as_slice(&self) -> &[T] {
            self.0.as_slice()
        }

        pub fn as_mut_slice(&mut self) -> &mut [T] {
            self.0.as_mut_slice()
        }
    }

    impl Vector<const char> {
        pub fn begin(&self) -> *const char {
            self.0.as_ptr() as *const char
        }
    }

    impl From<&str> for Vector<const char> {
        fn from(s: &str) -> Self {
            let vec: Vec<const char> = s.chars().map(|c| c as const char).collect();
            Vector(vec)
        }
    }
    pub struct SmallVector<T, const SIZE: usize> {
        data: Vec<T>,
    }

    impl<T, const SIZE: usize> SmallVector<T, const SIZE> {
        pub fn new() -> Self {
            SmallVector { data: Vec::new() }
        }

        pub fn resize(&mut self, new_size: usize)
            where
                T: Default + Clone,
        {
            self.data.resize(new_size, T::default());
        }

        pub fn data(&mut self) -> *mut T {
            self.data.as_mut_ptr()
        }
    }
    pub struct CStrVector(String);

    impl CStrVector {
        pub fn new(s: String) -> Self {
            CStrVector(s)
        }
    }

    impl From<&str> for CStrVector {
        fn from(s: &str) -> Self {
            CStrVector(s.to_string())
        }
    }
    pub struct ArrayVector<T>(Vec<T>);

    impl<T> ArrayVector<T> {
        pub fn new(vec: Vec<T>) -> Self {
            ArrayVector(vec)
        }
    }
    pub type uc16 = u16;
}

pub mod handles {
    pub struct Handle<T> {
        location: *mut T,
    }

    impl<T> Handle<T> {
        pub fn location(&self) -> *mut T {
            self.location
        }
    }

    pub struct DirectHandle<T> {
        value: T,
    }
}

pub mod objects {
    use crate::init::bootstrapper::Isolate;

    #[derive(Clone, Copy)]
    pub struct Tagged<T> {
        ptr: *mut T,
    }

    impl<T> Tagged<T> {
        pub fn ptr(&self) -> *mut T {
            self.ptr
        }
    }

    pub struct String {
        length: i32,
    }

    impl String {
        pub const kMaxShortPrintLength: i32 = 32;
        pub fn length(&self) -> i32 {
            self.length
        }
    }

    pub struct Number {}
    pub struct Oddball {}
    pub struct HeapObject {}
    pub struct JSObject {
        map_: Map,
    }
    impl JSObject{
        pub fn map(&self) -> &Map{
            &self.map_
        }
        pub fn RawFastPropertyAt(&self, index: FieldIndex) -> Tagged<Object>{
            Tagged{ptr: std::ptr::null_mut()}
        }
    }
    pub struct JSPrimitiveWrapper {
        value_: Object,
    }
    impl JSPrimitiveWrapper{
        pub fn value(&self) -> &Object{
            &self.value_
        }
    }
    pub struct JSArray {
        elements_: FixedArray,
        length_: Number,
    }
    impl JSArray{
        pub fn HasObjectElements(&self) -> bool{
            true
        }
        pub fn elements(&self) -> &FixedArray{
            &self.elements_
        }
        pub fn length(&self) -> &Number{
            &self.length_
        }
    }
    pub struct ByteArray {
        length_: u32,
    }
    impl ByteArray{
        pub fn get(&self, index: u32) -> u8{
            0
        }
        pub fn length(&self) -> u32{
            self.length_
        }
    }
    pub struct FixedArray {
        length_: u32,
    }
    impl FixedArray{
        pub fn length(&self) -> u32{
            self.length_
        }
        pub fn get(&self, index: u32) -> Tagged<Object>{
            Tagged{ptr: std::ptr::null_mut()}
        }
    }
    pub struct Map {
        instance_descriptors_: DescriptorArray,
    }
    impl Map{
        pub fn instance_descriptors(&self, isolate: *mut Isolate) -> &DescriptorArray{
            &self.instance_descriptors_
        }
        pub fn IterateOwnDescriptors(&self) -> InternalIndex{
            InternalIndex{}
        }
    }
    pub struct DescriptorArray {}
    impl DescriptorArray{
        pub fn GetDetails(&self, i: InternalIndex) -> PropertyDetails{
            PropertyDetails{}
        }
        pub fn GetKey(&self, i: InternalIndex) -> Tagged<Object>{
            Tagged{ptr: std::ptr::null_mut()}
        }
    }
    pub struct JSFunction {
        shared_: SharedFunctionInfo,
        native_context_: Context,
    }
    impl JSFunction{
        pub fn shared(&self) -> &SharedFunctionInfo{
            &self.shared_
        }
        pub fn native_context(&self) -> &Context{
            &self.native_context_
        }
    }
    pub struct SharedFunctionInfo {
        name_: Object,
    }
    impl SharedFunctionInfo{
        pub fn Name(&self) -> &Object{
            &self.name_
        }
    }
    pub struct Context {
        security_token_: Object,
    }
    impl Context{
        pub fn security_token(&self) -> &Object{
            &self.security_token_
        }
    }
    #[derive(Clone, Copy)]
    pub struct Object {}
    pub struct Null {}
    pub struct Undefined {}
    pub struct TheHole {}
    pub struct PropertyDetails {}
    pub enum PropertyLocation{
        kField,
    }
    pub enum PropertyKind{
        kData,
    }
    pub struct FieldIndex {}
    impl FieldIndex{
        pub fn ForDescriptor(map: &Map, i: InternalIndex) -> FieldIndex{
            FieldIndex{}
        }
    }
    pub struct InternalIndex {}
    pub struct JSProxy {}
    pub struct WasmObject {}
}

pub mod utils {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr::NonNull;

    pub struct AllocationResult<T> {
        ptr: NonNull<T>,
    }

    impl<T> AllocationResult<T> {
        pub fn new(size: usize) -> Option<Self> {
            let layout = Layout::array::<T>(size).ok()?;
            if layout.size() == 0 {
                return NonNull::dangling().wrap_none();
            }
            unsafe {
                let ptr = alloc(layout) as *mut T;
                NonNull::new(ptr).map(|ptr| AllocationResult { ptr })
            }
        }

        pub fn as_ptr(&self) -> *mut T {
            self.ptr.as_ptr()
        }
    }
    trait WrapNone {
        fn wrap_none<T>(self) -> Option<T>;
    }
    impl WrapNone for Option<std::ptr::NonNull<u8>> {
        fn wrap_none<T>(self) -> Option<T> {
            None
        }
    }
}

pub mod init {
    pub mod bootstrapper {
        pub struct Isolate {
            string_stream_debug_object_cache_: *mut DebugObjectCache,
            string_stream_current_security_token_: objects::Object,
        }
        impl Isolate{
            pub fn string_stream_debug_object_cache(&mut self) -> &mut DebugObjectCache{
                unsafe { &mut *self.string_stream_debug_object_cache_ }
            }
            pub fn set_string_stream_debug_object_cache(&mut self, cache: *mut DebugObjectCache){
                self.string_stream_debug_object_cache_ = cache;
            }
            pub fn set_string_stream_current_security_token(&mut self, token: objects::Object){
                self.string_stream_current_security_token_ = token;
            }
            pub fn string_stream_current_security_token(&self) -> &objects::Object{
                &self.string_stream_current_security_token_
            }
        }
    }
}

pub mod logging {
    use crate::init::bootstrapper::Isolate;

    pub struct StringEvent {
        name: String,
        message: String,
    }

    impl StringEvent {
        pub fn new(name: String, message: String) -> Self {
            StringEvent { name, message }
        }
    }

    pub fn LOG(isolate: *mut Isolate, event: StringEvent) {
        println!("{}: {}", event.name, event.message);
    }
}

pub mod strings {
    use std::fmt;
    use crate::base;
    use crate::handles;
    use crate::init::bootstrapper::Isolate;
    use crate::objects;
    use crate::objects::{FixedArray, HeapObject, JSArray, JSObject, JSPrimitiveWrapper, Map, Object, PropertyDetails, String as V8String};
    use crate::objects::PropertyLocation::kField;
    use crate::objects::PropertyKind::kData;
    use crate::ReadOnlyRoots;
    use crate::objects::ByteArray;
    use crate::objects::InternalIndex;
    use crate::objects::FieldIndex;
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr::NonNull;
    use crate::utils::AllocationResult;
    use crate::objects::JSFunction;
    use crate::objects::IsNullOrUndefined;
    use crate::objects::IsTheHole;
    use crate::objects::IsJSProxy;
    use crate::objects::IsWasmObject;
    use crate::PrototypeIterator;
    use crate::objects::IsString;
    use crate::objects::IsNumber;
    use crate::objects::IsOddball;
    use crate::objects::IsHeapObject;

    pub trait StringAllocator {
        fn allocate(&mut self, bytes: usize) -> Result<*mut char, String>;
        fn grow(&mut self, bytes: &mut usize) -> Result<*mut char, String>;
    }

    pub struct HeapStringAllocator {
        space_: *mut char,
    }

    impl HeapStringAllocator {
        pub fn new() -> Self {
            HeapStringAllocator { space_: std::ptr::null_mut() }
        }
    }

    impl Drop for HeapStringAllocator {
        fn drop(&mut self) {
            if !self.space_.is_null() {
                unsafe {
                    let layout = Layout::new::<char>();
                    dealloc(self.space_ as *mut u8, layout);
                }
            }
        }
    }

    impl StringAllocator for HeapStringAllocator {
        fn allocate(&mut self, bytes: usize) -> Result<*mut char, String> {
            unsafe {
                let layout = Layout::array::<char>(bytes)
                    .map_err(|e| format!("Failed to create layout: {}", e))?;
                let ptr = alloc(layout) as *mut char;
                if ptr.is_null() {
                    return Err("Allocation failed".to_string());
                }
                self.space_ = ptr;
                Ok(ptr)
            }
        }

        fn grow(&mut self, bytes: &mut usize) -> Result<*mut char, String> {
            let new_bytes = *bytes * 2;
            if new_bytes <= *bytes {
                return Ok(self.space_);
            }
            unsafe {
                let old_layout = Layout::array::<char>(*bytes)
                    .map_err(|e| format!("Failed to create layout: {}", e))?;
                let new_layout = Layout::array::<char>(new_bytes)
                    .map_err(|e| format!("Failed to create layout: {}", e))?;
                let new_space = alloc(new_layout) as *mut char;

                if new_space.is_null() {
                    return Err("Allocation failed".to_string());
                }

                if !self.space_.is_null() {
                    std::ptr::copy_nonoverlapping(self.space_, new_space, *bytes);
                    dealloc(self.space_ as *mut u8, old_layout);
                }

                self.space_ = new_space;
                *bytes = new_bytes;
                Ok(new_space)
            }
        }
    }

    pub struct FixedStringAllocator {
        buffer_: *mut char,
        length_: usize,
    }

    impl FixedStringAllocator {
        pub fn new(buffer: *mut char, length: usize) -> Self {
            FixedStringAllocator {
                buffer_: buffer,
                length_: length,
            }
        }
    }

    impl StringAllocator for FixedStringAllocator {
        fn allocate(&mut self, bytes: usize) -> Result<*mut char, String> {
            if bytes <= self.length_ {
                Ok(self.buffer_)
            } else {
                Err("Allocation size exceeds buffer length".to_string())
            }
        }

        fn grow(&mut self, old: &mut usize) -> Result<*mut char, String> {
            *old = self.length_;
            Ok(self.buffer_)
        }
    }

    pub struct SmallStringOptimizedAllocator<const kInlineSize: usize> {
        vector_: *mut base::SmallVector<char, kInlineSize>,
    }

    impl<const kInlineSize: usize> SmallStringOptimizedAllocator<kInlineSize> {
        pub fn new(vector: *mut base::SmallVector<char, kInlineSize>) -> Self {
            SmallStringOptimizedAllocator { vector_: vector }
        }
    }

    impl<const kInlineSize: usize> StringAllocator for SmallStringOptimizedAllocator<kInlineSize> {
        fn allocate(&mut self, bytes: usize) -> Result<*mut char, String> {
            unsafe {
                (*self.vector_).resize(bytes);
                Ok((*self.vector_).data())
            }
        }

        fn grow(&mut self, bytes: &mut usize) -> Result<*mut char, String> {
            let new_bytes = *bytes * 2;
            if new_bytes <= *bytes {
                unsafe {
                    return Ok((*self.vector_).data());
                }
            }
            unsafe {
                (*self.vector_).resize(new_bytes);
                *bytes = new_bytes;
                Ok((*self.vector_).data())
            }
        }
    }

    pub struct StringStream {
        allocator_: Box<dyn StringAllocator>,
        object_print_mode_: ObjectPrintMode,
        capacity_: usize,
        length_: usize,
        buffer_: *mut char,
    }

    impl StringStream {
        const kInitialCapacity: usize = 16;
        const kMentionedObjectCacheMaxSize: usize = 256;

        pub fn new(allocator: Box<dyn StringAllocator>, object_print_mode: ObjectPrintMode) -> Result<Self, String> {
            let mut stream = StringStream {
                allocator_: allocator,
                object_print_mode_: object_print_mode,
                capacity_: Self::kInitialCapacity,
                length_: 0,
                buffer_: std::ptr::null_mut(),
            };

            unsafe {
                stream.buffer_ = stream.allocator_.allocate(Self::kInitialCapacity)?;
                *stream.buffer_ = 0;
            }

            Ok(stream)
        }

        pub fn put(&mut self, c: char) -> Result<bool, String> {
            if self.full() {
                return Ok(false);
            }

            unsafe {
                if self.length_ == self.capacity_ - 2 {
                    let mut new_capacity = self.capacity_;
                    let new_buffer = self.allocator_.grow(&mut new_capacity)?;

                    if new_capacity > self.capacity_ {
                        self.capacity_ = new_capacity;
                        self.buffer_ = new_buffer;
                    } else {
                        self.length_ = self.capacity_ - 1;
                        *self.buffer_.add(self.length_ - 4) = '.';
                        *self.buffer_.add(self.length_ - 3) = '.';
                        *self.buffer_.add(self.length_ - 2) = '.';
                        *self.buffer_.add(self.length_ - 1) = '\n';
                        *self.buffer_.add(self.length_) = '\0';
                        return Ok(false);
                    }
                }

                *self.buffer_.add(self.length_) = c;
                *self.buffer_.add(self.length_ + 1) = '\0';
                self.length_ += 1;
            }

            Ok(true)
        }

        fn is_control_char(c: char) -> bool {
            match c {
                '0'..='9' | '.' | '-' => true,
                _ => false,
            }
        }

        pub fn add(&mut self, format: &str, args: &[FmtElm]) -> Result<(), String> {
            if self.full() {
                return Ok(());
            }

            let format_bytes = format.as_bytes();
            let mut offset = 0;
            let mut elm = 0;

            while offset < format_bytes.len() {
                if format_bytes[offset] != b'%' || elm == args.len() {
                    self.put(format_bytes[offset] as char)?;
                    offset += 1;
                    continue;
                }

                let mut temp = base::SmallVector::<char, 24>::new();
                let mut format_length = 0;

                temp.resize(format_length + 1);
                unsafe { *temp.data().add(format_length) = format_bytes[offset] as char; }
                format_length += 1;
                offset += 1;

                while offset < format_bytes.len() && Self::is_control_char(format_bytes[offset] as char) {
                    temp.resize(format_length + 1);
                    unsafe { *temp.data().add(format_length) = format_bytes[offset] as char; }
                    format_length += 1;
                    offset += 1;
                }

                if offset >= format_bytes.len() {
                    return Ok(());
                }

                let r#type = format_bytes[offset] as char;
                temp.resize(format_length + 1);
                unsafe { *temp.data().add(format_length) = r#type; }
                format_length += 1;
                offset += 1;

                unsafe {
                    *temp.data().add(format_length) = '\0';
                }

                let current = args[elm];
                elm += 1;

                match r#type {
                    's' => {
                        if let FmtElmUnion::C_STR(value) = current.data_ {
                            self.add_str(value)?;
                        }
                    }
                    'w' => {
                        if let FmtElmUnion::LC_STR(value) = current.data_ {
                            for i in 0..value.len() {
                                if let Some(c) = value.get(i) {
                                    self.put(*c as char)?;
                                }
                            }
                        }
                    }
                    'o' => {
                        if let FmtElmUnion::OBJ(obj) = current.data_ {
                            self.print_object(objects::Tagged { ptr: obj })?;
                        }
                    }
                    'k' => {
                        if let FmtElmUnion::INT(value) = current.data_ {
                            if 0x20 <= value && value <= 0x7F {
                                self.put(value as char)?;
                            } else if value <= 0xFF {
                                self.add_str(&format!("\\x{:02x}", value))?;
                            } else {
                                self.add_str(&format!("\\u{:04x}", value))?;
                            }
                        }
                    }
                    'i' | 'd' | 'u' | 'x' | 'c' | 'X' => {
                        if let FmtElmUnion::INT(value) = current.data_ {
                            let formatted = format!("{}", value);
                            self.add_str(&formatted)?;
                        }
                    }
                    'f' | 'g' | 'G' | 'e' | 'E' => {
                        if let FmtElmUnion::DOUBLE(value) = current.data_ {
                            if value.is_infinite() {
                                if value.is_sign_negative() {
                                    self.add_str("-inf")?;
                                } else {
                                    self.add_str("inf")?;
                                }
                            } else if value.is_nan() {
                                self.add_str("nan")?;
                            } else {
                                let formatted = format!("{}", value);
                                self.add_str(&formatted)?;
                            }
                        }
                    }
                    'p' => {
                        if let FmtElmUnion::POINTER(value) = current.data_ {
                            let formatted = format!("{:p}", value);
                            self.add_str(&formatted)?;
                        }
                    }
                    _ => {
                        return Err("UNREACHABLE".to_string());
                    }
                }
            }

            Ok(())
        }

        fn add_str(&mut self, s: &str) -> Result<(), String> {
            for c in s.chars() {
                self.put(c)?;
            }
            Ok(())
        }
        fn print_object(&mut self, o: objects::Tagged<Object>) -> Result<(), String> {
            self.short_print(o);
            if objects::IsString(o) {
                unsafe{
                    if objects::Cast::<V8String>(o).length() <= V8String::kMaxShortPrintLength {
                        return Ok(());
                    }
                }
            } else if objects::IsNumber(o) || objects::IsOddball(o) {
                return Ok(());
            }

            if objects::IsHeapObject(o) && self.object_print_mode_ == ObjectPrintMode::kPrintObjectVerbose {
                let isolate = self.get_current_isolate();
                let debug_object_cache = unsafe { &mut *(*isolate).string_stream_debug_object_cache_ };
                for i in 0..debug_object_cache.size() {
                    unsafe {
                        if *(*debug_object_cache)[i] == o {
                            self.add_str(&format!("#{}#", i as i32))?;
                            return Ok(());
                        }
                    }
                }

                if debug_object_cache.size() < Self::kMentionedObjectCacheMaxSize {
                    self.add_str(&format!("#{}#", debug_object_cache.size() as i32))?;
                    unsafe {
                        debug_object_cache.push_back(handles::Handle {
                            location: objects::Cast::<HeapObject>(o).ptr()
                        });
                    }
                } else {
                    self.add_str(&format!("@{:p}", o.ptr()))?;
                }
            }

            Ok(())
        }

        pub fn output_to_file(&mut self, out: *mut std::ffi::c_void) {
            let mut position = 0;
            unsafe {
                while position + 2048 < self.length_ {
                    let next = position + 2048;
                    let save = *self.buffer_.add(next);
                    *self.buffer_.add(next) = '\0';
                    println!("{}", std::ffi::CStr::from_ptr(self.buffer_.add(position)).to_str().unwrap());
                    *self.buffer_.add(next) = save;
                    position = next;
                }
                println!("{}", std::ffi::CStr::from_ptr(self.buffer_.add(position)).to_str().unwrap());
            }
        }

        pub fn output_to_std_out(&mut self) {
            self.output_to_file(std::io::stdout().as_raw_handle());
        }

        pub fn log(&mut self, isolate: *mut Isolate) {
            unsafe {
                let message = std::ffi::CStr::from_ptr(self.buffer_).to_str().unwrap().to_string();
                let event = logging::StringEvent::new("StackDump".to_string(), message);
                logging::LOG(isolate, event);
            }
        }

        pub fn to_string(&mut self, isolate: *mut Isolate) -> handles::DirectHandle<V8String> {
            unsafe {
                let buffer_slice = std::slice::from_raw_parts(self.buffer_ as *const u8, self.length_);
                let utf8_string = String::from_utf8_lossy(buffer_slice).to_string();
                let v8_string = V8String { length: utf8_string.len() as i32 };

                handles::DirectHandle { value: v8_string }
            }
        }

        pub fn to_c_string(&self) -> Result<std::ffi::CString, String> {
            unsafe {
                let buffer_slice = std::slice::from_raw_parts(self.buffer_ as *const u8, self.length_);
                std::ffi::CString::new(buffer_slice)
                    .map_err(|e| format!("Failed to create CString: {}", e))
            }
        }
        pub fn length(&self) -> usize {
            self.length_
        }
        pub fn print_name(&mut self, name: objects::Tagged<Object>) -> Result<(), String> {
            if objects::IsString(name) {
                unsafe {
                    let str = objects::Cast::<V8String>(name);
                    if str.length() > 0 {
                        self.put_string(str)?;
                    } else {
                        self.add_str("/* anonymous */")?;
                    }
                }
            } else {
                self.add_str(&format!("%o", name.ptr() as usize))?;
            }
            Ok(())
        }
        pub fn print_fixed_array(&mut self, array: objects::Tagged<FixedArray>, limit: u32) -> Result<(), String> {
            let roots = GetReadOnlyRoots();
            for i in 0..std::cmp::min(10, limit) {
                let element = unsafe {array.get(i)};
                if IsTheHole(element, roots) {
                    continue;
                }
                for _ in 1..18 {
                    self.put(' ')?;
                }
                self.add_str(&format!("{}: %o\n", i, element.ptr() as usize))?;
            }
            if limit >= 10 {
                self.add_str("                  ...\n")?;
            }
            Ok(())
        }
        pub fn print_byte_array(&mut self, byte_array: objects::Tagged<ByteArray>) -> Result<(), String> {
            let limit = unsafe {byte_array.length()};
            for i in 0..std::cmp::min(10, limit) {
                let b = unsafe {byte_array.get(i)};
                self.add_str(&format!("             {}: {:3} 0x{:02x}", i, b, b))?;
                if b >= b' ' && b <= b'~' {
                    self.add_str(&format!(" '{}'", b as char))?;
                } else if b == b'\n' {
                    self.add_str(" '\n'")?;
                } else if b == b'\r' {
                    self.add_str(" '\r'")?;
                } else if b >= 1 && b <= 26 {
                    self.add_str(&format!(" ^{}", (b + b'A' - 1) as char))?;
                }
                self.add_str("\n")?;
            }
            if limit >= 10 {
                self.add_str("                  ...\n")?;
            }
            Ok(())
        }
        pub fn print_mentioned_object_cache(&mut self, isolate: *mut Isolate) -> Result<(), String> {
            if self.object_print_mode_ == ObjectPrintMode::kPrintObjectConcise {
                return Ok(());
            }

            let debug_object_cache = unsafe { &mut *(*isolate).string_stream_debug_object_cache_ };
            self.add_str("-- ObjectCacheKey --\n\n")?;

            for i in 0..debug_object_cache.size() {
                let printee = unsafe { *(*debug_object_cache)[i] };
                self.add_str(&format!(" #{0}# {1:p}: ", i, printee.ptr()))?;
                self.short_print(printee);
                self.add_str("\n")?;

                if objects::IsJSObject(printee) {
                    unsafe {
                        if objects::IsJSPrimitiveWrapper(printee) {
                            let js_primitive_wrapper = objects::Cast::<JSPrimitiveWrapper>(printee);
                            self.add_str(&format!("           value(): %o\n", js_primitive_wrapper.value().ptr() as usize))?;
                        }
                        self.print_using_map(isolate, objects::Cast::<JSObject>(printee))?;
                        if objects::IsJSArray(printee) {
                            let array = objects::Cast::<JSArray>(printee);
                            if array.HasObjectElements() {
                                let limit = objects::Cast::<FixedArray>(array.elements()).length();
                                let length = objects::Object::NumberValue(array.length()) as u32;
                                let limit = std::cmp::min(length, limit);
                                self.print_fixed_array(objects::Cast::<FixedArray>(array.elements()), limit)?;
                            }
                        }
                    }
                } else if objects::IsByteArray(printee) {
                    self.print_byte_array(objects::Cast::<ByteArray>(printee))?;
                } else if objects::IsFixedArray(printee) {
                    let limit = unsafe{objects::Cast::<FixedArray>(printee).length()};
                    self.print_fixed_array(objects::Cast::<FixedArray>(printee), limit)?;
                }
            }
            Ok(())
        }
        pub fn print_using_map(&mut self, isolate: *mut Isolate, js_object: objects::Tagged<JSObject>) -> Result<(), String> {
            let map = unsafe{js_object.map()};
            let descs = unsafe{map.instance_descriptors(isolate)};
            for i in unsafe{map.IterateOwnDescriptors()} {
                let details = unsafe{descs.GetDetails(i)};
                if details.location() == kField {
                    if details.kind() == kData {
                        let key = unsafe{descs.GetKey(i)};
                        if objects::IsString(key) || objects::IsNumber(key) {
                            let mut len = 3;
                            if objects::IsString(key) {
                                unsafe{len = objects::Cast::<V8String>(key).length() as i32};
                            }
                            for _ in len..18 {
                                self.put(' ')?;
                            }
                            if objects::IsString(key) {
                                unsafe{self.put_string(objects::Cast::<V8String>(key))?};
                            } else {
                                self.short_print(key);
                            }
                            self.add_str(": ")?;
                            let index = unsafe{FieldIndex::ForDescriptor(map, i)};
                            let value = unsafe{js_object.RawFastPropertyAt(index)};
                            self.add_str(&format!("%o\n", value.ptr() as usize))?;
                        }
                    }
                }
            }
            Ok(())
        }
        pub fn print_security_token_if_changed(&mut self, isolate: *mut Isolate, fun: objects::Tagged<JSFunction>) -> Result<(), String> {
            let token = unsafe{fun.native_context().security_token()};
            if token.ptr() == unsafe{(*isolate).string_stream_current_security_token_.ptr()} {
                self.add_str(&format!("Security context: %o\n", token.ptr() as usize))?;
                unsafe {(*isolate).string_stream_current_security_token_ = *token;};
            }
