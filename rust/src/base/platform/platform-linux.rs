// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod base {
    use std::{
        ffi::CStr,
        fs::File,
        io::{BufRead, BufReader},
        os::raw::c_char,
        path::PathBuf,
    };

    /// Represents a memory region, as parsed from /proc/PID/maps.
    /// Visible for testing.
    #[derive(Debug, Default, PartialEq)]
    pub struct MemoryRegion {
        pub start: usize,
        pub end: usize,
        pub permissions: [char; 5],
        pub offset: usize, //off_t in C++, but usize can hold large offsets.
        pub dev: usize,    //dev_t in C++, using usize to hold the value.
        pub inode: usize,  //ino_t in C++, using usize to hold the value.
        pub pathname: String,
    }

    impl MemoryRegion {
        /// Parses a line from /proc/PID/maps into a `MemoryRegion`.
        ///
        /// The `line` must not contain the trailing newline character.
        pub fn from_maps_line(line: &str) -> Option<MemoryRegion> {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() < 5 {
                return None; // Invalid format
            }

            // Parse address range
            let address_range: Vec<&str> = parts[0].split('-').collect();
            if address_range.len() != 2 {
                return None; // Invalid address range format
            }
            let start = usize::from_str_radix(address_range[0], 16).ok()?;
            let end = usize::from_str_radix(address_range[1], 16).ok()?;

            // Parse permissions
            if parts[1].len() != 4 {
                return None; // Invalid permissions format
            }
            let permissions_str = parts[1];
            let mut permissions = ['-'; 5];
            for (i, c) in permissions_str.chars().enumerate() {
                permissions[i] = c;
            }
            permissions[4] = '\0';

            // Parse offset
            let offset = usize::from_str_radix(parts[2], 16).ok()?;

            // Parse device
            let dev_and_inode = parts[3];
            let mut dev: usize = 0;
            let mut inode: usize = 0;
            if dev_and_inode.len() > 0 {
                // Check if the string contains the ':'
                let parts2: Vec<&str> = dev_and_inode.split(':').collect();
                if parts2.len() == 2 {
                    dev = usize::from_str_radix(parts2[0], 16).ok()?;
                    inode = usize::from_str_radix(parts2[1], 16).ok()?;
                }
            }

            // Parse pathname (optional)
            let pathname = if parts.len() > 5 {
                parts[5..].join(" ")
            } else {
                String::new()
            };

            Some(MemoryRegion {
                start,
                end,
                permissions,
                offset,
                dev,
                inode,
                pathname,
            })
        }
    }

    /// Represents the address and name of a shared library.
    #[derive(Debug, PartialEq)]
    pub struct SharedLibraryAddress {
        pub address: usize,
        pub name: String,
    }

    /// Retrieves the addresses of shared libraries loaded in the current process.
    ///
    /// Uses the `/proc/self/maps` file to determine the loaded libraries.
    ///
    /// The `file` parameter is for testing, to pass a fake `/proc/self/maps` file.
    pub fn get_shared_library_addresses(file: File) -> Vec<SharedLibraryAddress> {
        let reader = BufReader::new(file);
        let mut shared_libraries = Vec::new();

        for line_result in reader.lines() {
            if let Ok(line) = line_result {
                if let Some(region) = MemoryRegion::from_maps_line(&line) {
                    // Assuming shared libraries are marked with 'x' (executable) permission.
                    if region.permissions[0] == 'r' && region.permissions[1] == 'w' && region.permissions[2] == 'x' {
                        if !region.pathname.is_empty() {
                            // Check if the path exists and is a file
                            let path = PathBuf::from(region.pathname.clone());
                            if path.exists() && path.is_file() {
                                shared_libraries.push(SharedLibraryAddress {
                                    address: region.start,
                                    name: region.pathname.clone(),
                                });
                            }

                        }
                    }
                }
            }
        }

        shared_libraries
    }
}