// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::c_char;
use std::sync::{Arc, Mutex, Weak};

// Placeholder types for V8 internal types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Name(*const u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Symbol(*const u8);

impl Name {
    fn is_symbol(&self) -> bool {
        false // Placeholder
    }

    fn as_symbol(&self) -> Option<Symbol> {
        None // Placeholder
    }
}

type Tagged<T> = T;

/// Provides a storage of strings allocated in C++ heap, to hold them
/// forever, even if they disappear from JS heap or external storage.
pub struct StringsStorage {
    names: Mutex<HashMap<Arc<str>, Weak<str>>>,
    string_size: Mutex<usize>,
}

impl StringsStorage {
    /// Constructs a new `StringsStorage`.
    pub fn new() -> Self {
        StringsStorage {
            names: Mutex::new(HashMap::new()),
            string_size: Mutex::new(0),
        }
    }

    /// Copies the given c-string and stores it, returning the stored copy, or just
    /// returns the existing string in storage if it already exists.
    pub fn get_copy(&self, src: *const c_char) -> Result<Arc<str>, std::str::Utf8Error> {
        let c_str = unsafe { CStr::from_ptr(src) };
        let string = c_str.to_str()?;
        self.get_string(string)
    }

    fn get_string(&self, string: &str) -> Result<Arc<str>, std::str::Utf8Error> {
        let mut names = self.names.lock().unwrap();
        if let Some(weak_ptr) = names.get(string) {
            if let Some(arc_ptr) = weak_ptr.upgrade() {
                return Ok(arc_ptr.clone());
            }
        }

        let arc_str = Arc::from(string);
        let weak_str = Arc::downgrade(&arc_str);
        names.insert(arc_str.clone(), weak_str);

        let mut string_size = self.string_size.lock().unwrap();
        *string_size += string.len();

        Ok(arc_str)
    }

    /// Returns a formatted string, de-duplicated via the storage.
    pub fn get_formatted(&self, format: &str, args: &[&dyn fmt::Display]) -> Result<Arc<str>, std::fmt::Error> {
        let formatted = format
            .split("{}")
            .zip(args.iter())
            .fold(String::new(), |acc, (prefix, arg)| {
                format!("{}{}", acc, format!("{}{}", prefix, arg))
            });
        let remaining = format.split("{}").last().unwrap_or("");
        let formatted = format!("{}{}", formatted, remaining);

        self.get_string(&formatted)
    }

    /// Returns a stored string resulting from name, or "<symbol>" for a symbol.
    pub fn get_name(&self, name: Tagged<Name>) -> Result<Arc<str>, std::str::Utf8Error> {
        //Placeholder implementation
        self.get_copy(name.0 as *const c_char)
    }

    /// Returns the string representation of the int from the store.
    pub fn get_name_int(&self, index: i32) -> Result<Arc<str>, std::str::Utf8Error> {
        self.get_string(&index.to_string())
    }

    /// Appends string resulting from name to prefix, then returns the stored
    /// result.
    pub fn get_cons_name(&self, prefix: &str, name: Tagged<Name>) -> Result<Arc<str>, std::str::Utf8Error> {
        //Placeholder implementation
        let name_str = unsafe { CStr::from_ptr(name.0 as *const c_char) }.to_str()?;
        let combined = format!("{}{}", prefix, name_str);
        self.get_string(&combined)
    }

    /// Reduces the refcount of the given string, freeing it if no other
    /// references are made to it. Returns true if the string was successfully
    /// unref'd, or false if the string was not present in the table.
    pub fn release(&self, str_ptr: *const c_char) -> Result<bool, std::str::Utf8Error> {
        let c_str = unsafe { CStr::from_ptr(str_ptr) };
        let string = c_str.to_str()?;

        let mut names = self.names.lock().unwrap();
        if let Some(weak_ptr) = names.get(string) {
            if let Some(arc_ptr) = weak_ptr.upgrade() {
                if Arc::ptr_eq(&arc_ptr, unsafe { &*(str_ptr as *const str as *const Arc<str>) }) {
                    // Decrease the ref count by dropping
                    drop(arc_ptr);

                    // If after drop, the weak pointer is now invalid, remove from hashmap
                    if weak_ptr.upgrade().is_none() {
                        names.remove(string);
                    }

                    let mut string_size = self.string_size.lock().unwrap();
                    *string_size -= string.len();

                    return Ok(true);
                }
                else{
                    return Ok(false);
                }
            }
        }
        Ok(false)
    }

    /// Returns the number of strings in the store.
    pub fn get_string_count_for_testing(&self) -> usize {
        let names = self.names.lock().unwrap();
        names.len()
    }

    /// Returns the size of strings in the store
    pub fn get_string_size(&self) -> usize {
        let string_size = self.string_size.lock().unwrap();
        *string_size
    }

    /// Returns true if the strings table is empty.
    pub fn is_empty(&self) -> bool {
        let names = self.names.lock().unwrap();
        names.is_empty()
    }
}

impl Drop for StringsStorage {
    fn drop(&mut self) {
        let mut names = self.names.lock().unwrap();
        names.clear();
    }
}

impl Default for StringsStorage {
    fn default() -> Self {
        Self::new()
    }
}