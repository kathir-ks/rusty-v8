// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod zone_utils {
    use std::marker::Copy;
    use std::mem;
    use std::vec::Vec;

    use crate::base::vector::Vector;
    use crate::zone::zone::Zone;

    /// Clones a `Vector<T>` into a new `Vector<T>` allocated in the given `Zone`.
    ///
    /// If `T` is trivially copyable, `memcpy` is used for efficiency. Otherwise,
    /// the elements are copied using `std::copy`.
    pub fn clone_vector<T: Copy>(zone: &Zone, other: &Vector<T>) -> Vector<T> {
        let length = other.len();
        if length == 0 {
            return Vector::empty();
        }

        let data = zone.allocate_array::<T>(length);
        if mem::needs_drop::<T>() {
            // T is not trivially copyable, using copy_from_slice is safe because zone allocated data is zero initialized
            data.copy_from_slice(other.as_slice());
        } else {
            // T is trivially copyable, use memcpy
            unsafe {
                std::ptr::copy_nonoverlapping(other.as_ptr(), data.as_mut_ptr(), length);
            }
        }
        Vector::from_raw_parts(data.as_mut_ptr(), length, length)
    }
}

pub mod base {
    pub mod vector {
        #[derive(Debug)]
        pub struct Vector<T> {
            data: *mut T,
            len: usize,
            capacity: usize,
        }

        impl<T> Vector<T> {
            pub fn empty() -> Self {
                Vector {
                    data: std::ptr::null_mut(),
                    len: 0,
                    capacity: 0,
                }
            }

            pub fn from_raw_parts(data: *mut T, len: usize, capacity: usize) -> Self {
                Vector { data, len, capacity }
            }

            pub fn as_slice(&self) -> &[T] {
                unsafe { std::slice::from_raw_parts(self.data, self.len) }
            }

            pub fn as_ptr(&self) -> *const T {
                self.data as *const T
            }

            pub fn as_mut_ptr(&self) -> *mut T {
                self.data
            }

            pub fn len(&self) -> usize {
                self.len
            }

            pub fn capacity(&self) -> usize {
                self.capacity
            }

        }

        impl<T: Copy> Vector<T> {
            pub fn to_vec(&self) -> Vec<T> {
                self.as_slice().to_vec()
            }
        }
    }
}

pub mod zone {
    pub mod zone {
        use std::alloc::{alloc, dealloc, Layout};
        use std::ptr::NonNull;

        pub struct Zone {
            // This is a simplified zone implementation.  A real zone implementation
            // would likely have more complex memory management.
            allocated: Vec<NonNull<u8>>,
        }

        impl Zone {
            pub fn new() -> Self {
                Zone {
                    allocated: Vec::new(),
                }
            }

            pub fn allocate_array<T>(&self, count: usize) -> Vec<T> {
                let layout = Layout::array::<T>(count).unwrap();
                unsafe {
                    let ptr = alloc(layout) as *mut T;
                    std::slice::from_raw_parts_mut(ptr, count).to_vec()
                }
            }
        }
    }
}