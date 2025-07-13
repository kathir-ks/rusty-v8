// Converted from V8 C++ source files:
// Header: version.h
// Implementation: version.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub fn hash_combine(a: i32, b: i32, c: i32, d: i32) -> i64 {
        let mut result: i64 = a as i64;
        result = result.wrapping_mul(31).wrapping_add(b as i64);
        result = result.wrapping_mul(31).wrapping_add(c as i64);
        result = result.wrapping_mul(31).wrapping_add(d as i64);
        result
    }

    pub struct Vector<T> {
        data: Vec<T>,
    }

    impl<T> Vector<T> {
        pub fn new() -> Self {
            Vector { data: Vec::new() }
        }

        pub fn resize(&mut self, new_size: usize) {
            self.data.resize_with(new_size, || panic!("Vector::resize requires T to implement Default or provide a closure"));
        }
    }

    impl Vector<char> {
        pub fn push(&mut self, c: char) {
            self.data.push(c);
        }
    }

    pub fn snprintf(buffer: &mut Vector<char>, format: &str, args: &[&dyn std::fmt::Display]) {
        let mut formatted_string = String::new();
        let mut arg_index = 0;
        for part in format.split('%') {
            if arg_index == 0 {
                formatted_string.push_str(part);
            } else {
                if let Some(arg) = args.get(arg_index - 1) {
                    formatted_string.push_str(&arg.to_string());
                    formatted_string.push_str(&part[1..]);
                }
            }
            arg_index += 1;
        }
    
        buffer.data = formatted_string.chars().collect();
    }
    
    
}

pub mod internal {

    use super::base;

    pub struct Version {
    }

    impl Version {
        static MAJOR: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(3);
        static MINOR: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(14);
        static BUILD: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(15);
        static PATCH: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(16);
        static EMBEDDER: std::sync::atomic::AtomicPtr<std::ffi::c_char> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
        static CANDIDATE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        static SONAME: std::sync::atomic::AtomicPtr<std::ffi::c_char> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
        static VERSION_STRING: std::sync::atomic::AtomicPtr<std::ffi::c_char> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
    
        pub fn get_major() -> i32 {
            Version::MAJOR.load(std::sync::atomic::Ordering::Relaxed)
        }
    
        pub fn get_minor() -> i32 {
            Version::MINOR.load(std::sync::atomic::Ordering::Relaxed)
        }
    
        pub fn get_build() -> i32 {
            Version::BUILD.load(std::sync::atomic::Ordering::Relaxed)
        }
    
        pub fn get_patch() -> i32 {
            Version::PATCH.load(std::sync::atomic::Ordering::Relaxed)
        }
    
        pub fn get_embedder() -> &'static str {
            unsafe {
                let ptr = Version::EMBEDDER.load(std::sync::atomic::Ordering::Relaxed);
                if ptr.is_null() {
                    ""
                } else {
                    std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("")
                }
            }
        }
    
        pub fn is_candidate() -> bool {
            Version::CANDIDATE.load(std::sync::atomic::Ordering::Relaxed)
        }
    
        pub fn hash() -> u32 {
            let major = Version::get_major();
            let minor = Version::get_minor();
            let build = Version::get_build();
            let patch = Version::get_patch();
            base::hash_combine(major, minor, build, patch) as u32
        }

        pub fn get_string(str: &mut base::Vector<char>) {
            let candidate_str = if Version::is_candidate() { " (candidate)" } else { "" };
            let embedder_str = Version::get_embedder();
        
            if Version::get_patch() > 0 {
                base::snprintf(
                    str,
                    "%d.%d.%d.%d%s%s",
                    &[
                        &Version::get_major(),
                        &Version::get_minor(),
                        &Version::get_build(),
                        &Version::get_patch(),
                        &embedder_str,
                        &candidate_str,
                    ],
                );
            } else {
                base::snprintf(
                    str,
                    "%d.%d.%d%s%s",
                    &[
                        &Version::get_major(),
                        &Version::get_minor(),
                        &Version::get_build(),
                        &embedder_str,
                        &candidate_str,
                    ],
                );
            }
        }

        pub fn get_soname(str: &mut base::Vector<char>) {
            unsafe {
                let soname_ptr = Version::SONAME.load(std::sync::atomic::Ordering::Relaxed);
                if soname_ptr.is_null() || (*soname_ptr == 0) {
                    let candidate_str = if Version::is_candidate() { "-candidate" } else { "" };
                    let embedder_str = Version::get_embedder();
        
                    if Version::get_patch() > 0 {
                        base::snprintf(
                            str,
                            "libv8-%d.%d.%d.%d%s%s.so",
                            &[
                                &Version::get_major(),
                                &Version::get_minor(),
                                &Version::get_build(),
                                &Version::get_patch(),
                                &embedder_str,
                                &candidate_str,
                            ],
                        );
                    } else {
                        base::snprintf(
                            str,
                            "libv8-%d.%d.%d%s%s.so",
                            &[
                                &Version::get_major(),
                                &Version::get_minor(),
                                &Version::get_build(),
                                &embedder_str,
                                &candidate_str,
                            ],
                        );
                    }
                } else {
                    let soname_cstr = std::ffi::CStr::from_ptr(soname_ptr);
                    let soname_str = soname_cstr.to_str().unwrap_or("");
        
                    base::snprintf(str, "%s", &[&soname_str]);
                }
            }
        }

        pub fn get_version() -> &'static str {
            unsafe {
                let ptr = Version::VERSION_STRING.load(std::sync::atomic::Ordering::Relaxed);
                if ptr.is_null() {
                    ""
                } else {
                    std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("")
                }
            }
        }
    }
}
