// Copyright 2009 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod version {
    use std::ffi::CString;
    use std::os::raw::c_char;

    use std::hash::{Hash, Hasher};

    pub struct Version {
        major: i32,
        minor: i32,
        build: i32,
        patch: i32,
        embedder: CString,
        candidate: bool,
        soname: CString,
        version_string: CString,
    }

    impl Version {
        /// Returns the major version component.
        pub fn get_major(&self) -> i32 {
            self.major
        }

        /// Returns the minor version component.
        pub fn get_minor(&self) -> i32 {
            self.minor
        }

        /// Returns the build version component.
        pub fn get_build(&self) -> i32 {
            self.build
        }

        /// Returns the patch version component.
        pub fn get_patch(&self) -> i32 {
            self.patch
        }

        /// Returns the embedder string.
        pub fn get_embedder(&self) -> &CStr {
            self.embedder.as_c_str()
        }

        /// Returns true if this is a candidate version.
        pub fn is_candidate(&self) -> bool {
            self.candidate
        }

        /// Calculates a hash of the version components.
        pub fn hash(&self) -> u32 {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            self.major.hash(&mut hasher);
            self.minor.hash(&mut hasher);
            self.build.hash(&mut hasher);
            self.patch.hash(&mut hasher);
            hasher.finish() as u32
        }

        /// Calculates the V8 version string.
        // TODO: Implement GetString using Rust's String type instead of base::Vector<char>.
        pub fn get_string(&self) -> String {
          format!("{}.{}.{}.{}", self.major, self.minor, self.build, self.patch)
        }

        /// Calculates the SONAME for the V8 shared library.
        // TODO: Implement GetSONAME using Rust's String type instead of base::Vector<char>.
        pub fn get_soname(&self) -> String {
          format!("libv8.so.{}.{}.{}", self.major, self.minor, self.build)
        }

        /// Returns the version string.
        pub fn get_version(&self) -> &CStr {
            self.version_string.as_c_str()
        }
    }

    use std::ffi::CStr;

    lazy_static::lazy_static! {
        static ref VERSION: Version = {
            // Default initialization.  These values will likely be overwritten by SetVersion().
            Version {
                major: 0,
                minor: 0,
                build: 0,
                patch: 0,
                embedder: CString::new("").unwrap(),
                candidate: false,
                soname: CString::new("").unwrap(),
                version_string: CString::new("").unwrap(),
            }
        };
    }

    pub fn get_version_static() -> &'static Version {
        &VERSION
    }

    // NOTE: this function mimics the friend relationship in C++ by allowing
    // test code to set the static version.  This is generally not good
    // Rust style.
    #[cfg(test)]
    pub fn set_version(
        major: i32,
        minor: i32,
        build: i32,
        patch: i32,
        embedder: &str,
        candidate: bool,
        soname: &str,
    ) {
        use std::sync::Once;
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            let mut version = unsafe {
                std::ptr::addr_of_mut!(*VERSION) as *mut Version
            };

            unsafe {
                (*version).major = major;
                (*version).minor = minor;
                (*version).build = build;
                (*version).patch = patch;
                (*version).embedder = CString::new(embedder).unwrap();
                (*version).candidate = candidate;
                (*version).soname = CString::new(soname).unwrap();
                (*version).version_string = CString::new(format!("{}.{}.{}.{}", major, minor, build, patch)).unwrap();

            }
        });

    }
}