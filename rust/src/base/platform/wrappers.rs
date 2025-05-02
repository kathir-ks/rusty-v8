// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod wrappers {

    /// Opens a file with the given filename and mode.
    ///
    /// On Starboard platforms, this function always returns `None`.
    ///
    /// # Safety
    ///
    /// The `filename` and `mode` parameters must be valid C strings.
    pub unsafe fn fopen(filename: *const i8, mode: *const i8) -> Option<*mut libc::FILE> {
        #[cfg(target_os = "starboard")]
        {
            None
        }
        #[cfg(not(target_os = "starboard"))]
        {
            if filename.is_null() || mode.is_null() {
                return None; // Or handle the error as appropriate
            }

            let file = libc::fopen(filename, mode);
            if file.is_null() {
                None
            } else {
                Some(file)
            }
        }
    }

    /// Closes the given file stream.
    ///
    /// On Starboard platforms, this function always returns -1.
    ///
    /// # Safety
    ///
    /// The `stream` parameter must be a valid file stream.
    pub unsafe fn fclose(stream: *mut libc::FILE) -> Result<i32, i32> {
        #[cfg(target_os = "starboard")]
        {
            Err(-1)
        }
        #[cfg(not(target_os = "starboard"))]
        {
            if stream.is_null() {
                return Err(libc::EINVAL); // Indicate invalid argument
            }

            let result = libc::fclose(stream);
            if result == 0 {
                Ok(result)
            } else {
                Err(result)
            }
        }
    }
}