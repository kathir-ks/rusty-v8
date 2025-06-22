// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod flags_impl {
    use std::{
        cmp::Ordering,
        ffi::CStr,
        fmt,
        os::raw::c_char,
        ptr,
        sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, Ordering as AtomicOrdering},
        collections::HashSet,
    };
    use lazy_static::lazy_static;

    // Placeholder for src/base/macros.h functionality (e.g., DCHECK)
    macro_rules! dcheck {
        ($condition:expr) => {
            if !$condition {
                panic!("DCHECK failed: {}", stringify!($condition));
            }
        };
    }

    pub struct FlagHelpers;

    impl FlagHelpers {
        pub fn normalize_char(ch: char) -> char {
            ch.to_ascii_lowercase()
        }

        pub fn flag_names_cmp(a: *const c_char, b: *const c_char) -> i32 {
            unsafe {
                let a_cstr = CStr::from_ptr(a);
                let b_cstr = CStr::from_ptr(b);
                match a_cstr.to_bytes().cmp(b_cstr.to_bytes()) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                }
            }
        }

        pub fn equal_names(a: *const c_char, b: *const c_char) -> bool {
            unsafe {
                let a_cstr = CStr::from_ptr(a);
                let b_cstr = CStr::from_ptr(b);
                a_cstr.to_bytes() == b_cstr.to_bytes()
            }
        }

        pub fn equal_name_with_suffix(a: *const c_char, b: *const c_char) -> bool {
            // This function's logic depends on the nature of the suffix being checked,
            // and a direct translation isn't possible without further context.
            // A placeholder is used here, assuming it checks for equality.
            FlagHelpers::equal_names(a, b)
        }
    }

    //extern "C" {
    //    pub fn FindFlagByPointer(ptr: *const std::ffi::c_void) -> *mut Flag;
    //    pub fn FindFlagByName(name: *const c_char) -> *mut Flag;
    //    pub fn FindImplicationFlagByName(name: *const c_char) -> *mut Flag;
    //}
    
    // This requires a global mutable state which is not safe to expose directly.
    // Instead, provide safe wrapper functions that handle synchronization and lifetime issues.
    // pub static mut FLAGS: Vec<Flag> = Vec::new();

    // Need a safe way to access the global flags.  A simple solution is to use a global mutex
    // and lazy initialization to ensure thread safety.
    lazy_static! {
        static ref FLAGS: std::sync::Mutex<Vec<Flag>> = std::sync::Mutex::new(Vec::new());
    }

    pub fn find_flag_by_pointer(ptr: *const std::ffi::c_void) -> Option<*mut Flag> {
        let flags = FLAGS.lock().unwrap();
        for flag in flags.iter() {
            if flag.valptr_ as *const _ == ptr {
                return Some(flag as *const Flag as *mut Flag);
            }
        }
        None
    }

    pub fn find_flag_by_name(name: &str) -> Option<*mut Flag> {
        let flags = FLAGS.lock().unwrap();
        for flag in flags.iter() {
            if flag.name_ == name {
                return Some(flag as *const Flag as *mut Flag);
            }
        }
        None
    }

    pub fn find_implication_flag_by_name(name: &str) -> Option<*mut Flag> {
        // The current C++ implementation does not have any special handling for "implication flags",
        // so the following code is equivalent to `find_flag_by_name`
        let flags = FLAGS.lock().unwrap();
        for flag in flags.iter() {
            if flag.name_ == name {
                return Some(flag as *const Flag as *mut Flag);
            }
        }
        None
    }

    pub fn flags() -> Vec<Flag> {
        let flags = FLAGS.lock().unwrap();
        flags.clone()
    }


    #[derive(Clone, Copy)]
    pub struct FlagName {
        pub name: &'static str,
        pub negated: bool,
    }

    impl FlagName {
        pub const fn new(name: &'static str, negated: bool) -> Self {
            dcheck!(!name.is_empty());
            dcheck!(name.chars().next() != Some('!'));
            FlagName { name, negated }
        }

        pub const fn from_str(name: &'static str) -> Self {
            if name.starts_with('!') {
                FlagName::new(&name[1..], true)
            } else {
                FlagName::new(name, false)
            }
        }
    }

    impl fmt::Display for FlagName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}{}", if self.negated { "!" } else { "" }, self.name)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FlagType {
        TYPE_BOOL,
        TYPE_MAYBE_BOOL,
        TYPE_INT,
        TYPE_UINT,
        TYPE_UINT64,
        TYPE_FLOAT,
        TYPE_SIZE_T,
        TYPE_STRING,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum SetBy {
        kDefault,
        kWeakImplication,
        kImplication,
        kCommandLine,
    }

    impl Flag {
        pub fn is_any_implication(set_by: SetBy) -> bool {
            set_by == SetBy::kWeakImplication || set_by == SetBy::kImplication
        }
    }

    #[derive(Clone)]
    pub struct Flag {
        pub type_: FlagType,
        pub name_: &'static str,
        pub valptr_: *mut std::ffi::c_void, // Consider AtomicPtr if concurrent access is needed
        pub defptr_: *const std::ffi::c_void,
        pub cmt_: &'static str,
        pub owns_ptr_: bool,
        pub set_by_: SetBy,
        pub implied_by_: Option<&'static str>,
        #[cfg(debug_assertions)]
        pub implied_by_ptr_: Option<*const Flag>, // Consider using a Weak pointer to avoid cycles
    }

    impl Flag {
        pub fn new(
            type_: FlagType,
            name_: &'static str,
            valptr_: *mut std::ffi::c_void,
            defptr_: *const std::ffi::c_void,
            cmt_: &'static str,
            owns_ptr_: bool,
        ) -> Self {
            Flag {
                type_,
                name_,
                valptr_,
                defptr_,
                cmt_,
                owns_ptr_,
                set_by_: SetBy::kDefault,
                implied_by_: None,
                #[cfg(debug_assertions)]
                implied_by_ptr_: None,
            }
        }

        pub fn type_(&self) -> FlagType {
            self.type_
        }

        pub fn name(&self) -> &'static str {
            self.name_
        }

        pub fn comment(&self) -> &'static str {
            self.cmt_
        }

        pub fn points_to(&self, ptr: *const std::ffi::c_void) -> bool {
            self.valptr_ as *const _ == ptr
        }

        #[cfg(debug_assertions)]
        pub fn implied_by(&self, ptr: *const std::ffi::c_void) -> bool {
            let mut current = self.implied_by_ptr_;
            let mut visited_flags: HashSet<*const Flag> = HashSet::new();
            while let Some(curr) = current {
                visited_flags.insert(curr);
                if unsafe { (*curr).points_to(ptr) } {
                    return true;
                }
                current = unsafe { (*curr).implied_by_ptr_ };
                if visited_flags.contains(&current.unwrap_or(ptr::null())) {
                    break;
                }
            }
            false
        }

        pub fn bool_variable(&self) -> bool {
            self.get_value::<bool>(FlagType::TYPE_BOOL)
        }

        pub fn set_bool_variable(&mut self, value: bool, set_by: SetBy) {
            self.set_value::<bool>(value, set_by, FlagType::TYPE_BOOL);
        }

        pub fn maybe_bool_variable(&self) -> Option<bool> {
            self.get_value::<Option<bool>>(FlagType::TYPE_MAYBE_BOOL)
        }

        pub fn set_maybe_bool_variable(&mut self, value: Option<bool>, set_by: SetBy) {
            self.set_value::<Option<bool>>(value, set_by, FlagType::TYPE_MAYBE_BOOL);
        }

        pub fn int_variable(&self) -> i32 {
            self.get_value::<i32>(FlagType::TYPE_INT)
        }

        pub fn set_int_variable(&mut self, value: i32, set_by: SetBy) {
            self.set_value::<i32>(value, set_by, FlagType::TYPE_INT);
        }

        pub fn uint_variable(&self) -> u32 {
            self.get_value::<u32>(FlagType::TYPE_UINT)
        }

        pub fn set_uint_variable(&mut self, value: u32, set_by: SetBy) {
            self.set_value::<u32>(value, set_by, FlagType::TYPE_UINT);
        }

        pub fn uint64_variable(&self) -> u64 {
            self.get_value::<u64>(FlagType::TYPE_UINT64)
        }

        pub fn set_uint64_variable(&mut self, value: u64, set_by: SetBy) {
            self.set_value::<u64>(value, set_by, FlagType::TYPE_UINT64);
        }

        pub fn float_variable(&self) -> f64 {
            self.get_value::<f64>(FlagType::TYPE_FLOAT)
        }

        pub fn set_float_variable(&mut self, value: f64, set_by: SetBy) {
            self.set_value::<f64>(value, set_by, FlagType::TYPE_FLOAT);
        }

        pub fn size_t_variable(&self) -> usize {
            self.get_value::<usize>(FlagType::TYPE_SIZE_T)
        }

        pub fn set_size_t_variable(&mut self, value: usize, set_by: SetBy) {
            self.set_value::<usize>(value, set_by, FlagType::TYPE_SIZE_T);
        }

        pub fn string_value(&self) -> *const c_char {
            self.get_value::<*const c_char>(FlagType::TYPE_STRING)
        }

        pub fn set_string_value(&mut self, new_value: *const c_char, owns_new_value: bool, set_by: SetBy) {
            // Note: Requires careful handling of memory ownership to avoid leaks or double frees.
            let current_value = self.get_value::<*const c_char>(FlagType::TYPE_STRING);

            let change_flag = current_value != new_value;
            let change_flag = self.check_flag_change(set_by, change_flag, None);

            if change_flag {
                dcheck!(!self.is_read_only());

                // Release the old string value if it was owned by the flag
                if self.owns_ptr_ {
                    unsafe {
                        if !current_value.is_null() {
                            // Deallocate the old string.  For simplicity, we assume it was allocated with libc::free.
                            libc::free(current_value as *mut libc::c_void);
                        }
                    }
                }

                // Set the new string value and ownership
                unsafe {
                    *(self.valptr_ as *mut *const c_char) = new_value;
                }
                self.owns_ptr_ = owns_new_value;
            }
        }


        pub fn get_default_value<T>(&self) -> T
        where
            T: Copy,
        {
            dcheck!(!self.defptr_.is_null());
            unsafe { *(self.defptr_ as *const T) }
        }

        pub fn bool_default(&self) -> bool {
            dcheck_eq!(self.type_, FlagType::TYPE_BOOL);
            self.get_default_value::<bool>()
        }

        pub fn int_default(&self) -> i32 {
            dcheck_eq!(self.type_, FlagType::TYPE_INT);
            self.get_default_value::<i32>()
        }

        pub fn uint_default(&self) -> u32 {
            dcheck_eq!(self.type_, FlagType::TYPE_UINT);
            self.get_default_value::<u32>()
        }

        pub fn uint64_default(&self) -> u64 {
            dcheck_eq!(self.type_, FlagType::TYPE_UINT64);
            self.get_default_value::<u64>()
        }

        pub fn float_default(&self) -> f64 {
            dcheck_eq!(self.type_, FlagType::TYPE_FLOAT);
            self.get_default_value::<f64>()
        }

        pub fn size_t_default(&self) -> usize {
            dcheck_eq!(self.type_, FlagType::TYPE_SIZE_T);
            self.get_default_value::<usize>()
        }

        pub fn string_default(&self) -> *const c_char {
            dcheck_eq!(self.type_, FlagType::TYPE_STRING);
            self.get_default_value::<*const c_char>()
        }

        pub fn should_check_flag_contradictions() -> bool {
            // Placeholder: Implement logic to determine if flag contradictions should be checked.
            true
        }

        pub fn check_flag_change(&mut self, new_set_by: SetBy, mut change_flag: bool, implied_by: Option<&str>) -> bool {
            if !change_flag {
                return change_flag;
            }
            if new_set_by == SetBy::kWeakImplication {
                if self.set_by_ >= SetBy::kImplication {
                    change_flag = false;
                }
            }
            change_flag
        }

        pub fn is_read_only(&self) -> bool {
            self.valptr_.is_null()
        }

        pub fn get_value<T>(&self, flag_type: FlagType) -> T
        where
            T: Copy,
        {
            dcheck_eq!(flag_type, self.type_);
            if self.is_read_only() {
                self.get_default_value::<T>()
            } else {
                unsafe { *(self.valptr_ as *const T) }
            }
        }

        fn set_value<T>(&mut self, new_value: T, set_by: SetBy, flag_type: FlagType)
        where
            T: Copy + PartialEq,
        {
            dcheck_eq!(flag_type, self.type_);
            let mut change_flag = self.get_value::<T>(flag_type) != new_value;
            change_flag = self.check_flag_change(set_by, change_flag, None);
            if change_flag {
                dcheck!(!self.is_read_only());
                unsafe {
                    *(self.valptr_ as *mut T) = new_value;
                }
                self.set_by_ = set_by;
            }
        }


        pub fn is_default(&self) -> bool {
            match self.type_ {
                FlagType::TYPE_BOOL => self.bool_variable() == self.bool_default(),
                FlagType::TYPE_MAYBE_BOOL => self.maybe_bool_variable() == Some(self.bool_default()),
                FlagType::TYPE_INT => self.int_variable() == self.int_default(),
                FlagType::TYPE_UINT => self.uint_variable() == self.uint_default(),
                FlagType::TYPE_UINT64 => self.uint64_variable() == self.uint64_default(),
                FlagType::TYPE_FLOAT => self.float_variable() == self.float_default(),
                FlagType::TYPE_SIZE_T => self.size_t_variable() == self.size_t_default(),
                FlagType::TYPE_STRING => self.string_value() == self.string_default(),
            }
        }

        pub fn release_dynamic_allocations(&mut self) {
            if self.type_ == FlagType::TYPE_STRING && self.owns_ptr_ {
                let current_value = self.get_value::<*const c_char>(FlagType::TYPE_STRING);
                if !current_value.is_null() {
                    unsafe {
                        // Deallocate the string.  For simplicity, we assume it was allocated with libc::free.
                        libc::free(current_value as *mut libc::c_void);
                    }
                    unsafe {
                        *(self.valptr_ as *mut *const c_char) = ptr::null();
                    }
                    self.owns_ptr_ = false;
                }
            }
        }

        pub fn reset(&mut self) {
            match self.type_ {
                FlagType::TYPE_BOOL => {
                    let default_value = self.bool_default();
                    self.set_bool_variable(default_value, SetBy::kDefault);
                }
                FlagType::TYPE_MAYBE_BOOL => {
                    // Note:  This will panic when compiled in non-debug mode
                    let default_value: bool = self.bool_default();  // TODO: determine what the default optional value should be, since an uninitialized optional is not equivalent to false.
                    self.set_maybe_bool_variable(Some(default_value), SetBy::kDefault);
                }
                FlagType::TYPE_INT => {
                    let default_value = self.int_default();
                    self.set_int_variable(default_value, SetBy::kDefault);
                }
                FlagType::TYPE_UINT => {
                    let default_value = self.uint_default();
                    self.set_uint_variable(default_value, SetBy::kDefault);
                }
                FlagType::TYPE_UINT64 => {
                    let default_value = self.uint64_default();
                    self.set_uint64_variable(default_value, SetBy::kDefault);
                }
                FlagType::TYPE_FLOAT => {
                    let default_value = self.float_default();
                    self.set_float_variable(default_value, SetBy::kDefault);
                }
                FlagType::TYPE_SIZE_T => {
                    let default_value = self.size_t_default();
                    self.set_size_t_variable(default_value, SetBy::kDefault);
                }
                FlagType::TYPE_STRING => {
                    let default_value = self.string_default();
                    self.set_string_value(default_value, false, SetBy::kDefault);
                }
            }
        }

        pub fn allow_overwriting(&mut self) {
            self.set_by_ = SetBy::kDefault;
        }
    }

    macro_rules! dcheck_eq {
        ($left:expr, $right:expr) => {
            if $left != $right {
                panic!("DCHECK failed: {} == {}", stringify!($left), stringify!($right));
            }
        };
    }

    // Atomic Flag Values

    pub struct FlagValue<T> {
        value: T,
    }

    impl<T: Copy> FlagValue<T> {
        pub fn new(value: T) -> Self {
            FlagValue { value }
        }

        pub fn get(&self) -> T {
            self.value
        }

        pub fn set(&mut self, new_value: T) {
            self.value = new_value;
        }
    }

    // Implement specialized AtomicFlagValue for types that allow atomic operations

    pub struct AtomicFlagValueBool {
        value: AtomicBool,
    }

    impl AtomicFlagValueBool {
        pub fn new(value: bool) -> Self {
            AtomicFlagValueBool { value: AtomicBool::new(value) }
        }

        pub fn get(&self) -> bool {
            self.value.load(AtomicOrdering::Relaxed)
        }

        pub fn set(&self, new_value: bool) {
            self.value.store(new_value, AtomicOrdering::Relaxed);
        }
    }

    pub struct AtomicFlagValueI32 {
        value: AtomicI32,
    }

    impl AtomicFlagValueI32 {
        pub fn new(value: i32) -> Self {
            AtomicFlagValueI32 { value: AtomicI32::new(value) }
        }

        pub fn get(&self) -> i32 {
            self.value.load(AtomicOrdering::Relaxed)
        }

        pub fn set(&self, new_value: i32) {
            self.value.store(new_value, AtomicOrdering::Relaxed);
        }
    }

    pub struct AtomicFlagValueU32 {
        value: AtomicU32,
    }

    impl AtomicFlagValueU32 {
        pub fn new(value: u32) -> Self {
            AtomicFlagValueU32 { value: AtomicU32::new(value) }
        }

        pub fn get(&self) -> u32 {
            self.value.load(AtomicOrdering::Relaxed)
        }

        pub fn set(&self, new_value: u32) {
            self.value.store(new_value, AtomicOrdering::Relaxed);
        }
    }

    pub struct AtomicFlagValueU64 {
        value: AtomicU64,
    }

    impl AtomicFlagValueU64 {
        pub fn new(value: u64) -> Self {
            AtomicFlagValueU64 { value: AtomicU64::new(value) }
        }

        pub fn get(&self) -> u64 {
            self.value.load(AtomicOrdering::Relaxed)
        }

        pub fn set(&self, new_value: u64) {
            self.value.store(new_value, AtomicOrdering::Relaxed);
        }
    }
}