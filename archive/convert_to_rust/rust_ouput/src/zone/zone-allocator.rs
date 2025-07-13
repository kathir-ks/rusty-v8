// Converted from V8 C++ source files:
// Header: zone-allocator.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod zone_allocator {
    use std::marker::PhantomData;
    use std::ptr::null_mut;
    use crate::zone::zone::Zone;
    use crate::zone::zone::ZoneObject;
    use std::mem::size_of;
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr::NonNull;
    use std::mem;

    #[derive(Debug)]
    pub enum ZoneAllocatorError {
        AllocationError,
        DeallocationError,
        CompressionNotSupported,
    }

    pub struct ZoneAllocator<T> {
        zone_: *mut Zone,
        _phantom: PhantomData<T>,
    }

    impl<T> ZoneAllocator<T> {
        pub fn new(zone: *mut Zone) -> Self {
            unsafe {
                if size_of::<*mut T>() > 0 && (*zone).supports_compression() == false {
                }
            }

            ZoneAllocator {
                zone_: zone,
                _phantom: PhantomData,
            }
        }

        pub fn allocate(&self, length: usize) -> Result<*mut T, ZoneAllocatorError> {
            unsafe {
                if self.zone_.is_null() {
                    return Err(ZoneAllocatorError::AllocationError);
                }
                let size = length * size_of::<T>();
                 let layout = Layout::array::<T>(length).map_err(|_| ZoneAllocatorError::AllocationError)?;

                let ptr = (*self.zone_).allocate_aligned(size, mem::align_of::<T>());

                if ptr.is_null() {
                    Err(ZoneAllocatorError::AllocationError)
                } else {
                    Ok(ptr as *mut T)
                }
            }
        }

        pub fn deallocate(&self, p: *mut T, length: usize) -> Result<(), ZoneAllocatorError> {
             unsafe {
                if self.zone_.is_null() {
                    return Err(ZoneAllocatorError::DeallocationError);
                }
                if p.is_null() {
                    return Ok(());
                }

                let size = length * size_of::<T>();
                 let layout = Layout::array::<T>(length).map_err(|_| ZoneAllocatorError::DeallocationError)?;
                (*self.zone_).free(p as *mut std::ffi::c_void, size);
                Ok(())
             }
        }

        pub fn zone(&self) -> *mut Zone {
            self.zone_
        }
    }

    impl<T> PartialEq for ZoneAllocator<T> {
        fn eq(&self, other: &Self) -> bool {
            self.zone_ == other.zone_
        }
    }

    impl<T> Eq for ZoneAllocator<T> {}

    impl<T> Clone for ZoneAllocator<T> {
        fn clone(&self) -> Self {
            ZoneAllocator {
                zone_: self.zone_,
                _phantom: PhantomData,
            }
        }
    }

    impl<T, U> From<&ZoneAllocator<U>> for ZoneAllocator<T> {
        fn from(other: &ZoneAllocator<U>) -> Self {
            ZoneAllocator::new(other.zone_)
        }
    }

    struct FreeBlock {
        next: *mut FreeBlock,
        size: usize,
    }

    pub struct RecyclingZoneAllocator<T> {
        zone_allocator: ZoneAllocator<T>,
        free_list_: *mut FreeBlock,
        _phantom: PhantomData<T>,
    }

    impl<T> RecyclingZoneAllocator<T> {
        pub fn new(zone: *mut Zone) -> Self {
            RecyclingZoneAllocator {
                zone_allocator: ZoneAllocator::new(zone),
                free_list_: null_mut(),
                _phantom: PhantomData,
            }
        }

        pub fn allocate(&mut self, n: usize) -> Result<*mut T, ZoneAllocatorError> {
            unsafe {
                if !self.free_list_.is_null() && (*self.free_list_).size >= n {
                    let return_val = self.free_list_ as *mut T;
                    self.free_list_ = (*self.free_list_).next;
                    return Ok(return_val);
                }
            }
            self.zone_allocator.allocate(n)
        }

        pub fn deallocate(&mut self, p: *mut T, n: usize) -> Result<(), ZoneAllocatorError> {
            if size_of::<T>() * n < size_of::<FreeBlock>() {
                return Ok(());
            }

            unsafe {
                if self.free_list_.is_null() || (*self.free_list_).size <= n {
                    if size_of::<T>() * n < size_of::<FreeBlock>() {
                        return Ok(());
                    }
                    let new_free_block = p as *mut FreeBlock;
                    (*new_free_block).size = n;
                    (*new_free_block).next = self.free_list_;
                    self.free_list_ = new_free_block;
                }
            }
            Ok(())
        }
    }

    impl<T, U> From<&RecyclingZoneAllocator<U>> for RecyclingZoneAllocator<T> {
        fn from(other: &RecyclingZoneAllocator<U>) -> Self {
            RecyclingZoneAllocator::new(other.zone_allocator.zone())
        }
    }
}
