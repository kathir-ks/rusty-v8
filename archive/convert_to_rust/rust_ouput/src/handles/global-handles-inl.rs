// Converted from V8 C++ source files:
// Header: global-handles-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod global_handles_inl {
  use crate::handles::global_handles::*;
  use crate::handles::handles_inl::*;
  use crate::objects::heap_object_inl::*;
  use crate::objects::tagged::Tagged;
  use std::marker::PhantomData;
  use std::mem::size_of;

  impl GlobalHandles {
    pub fn create<T>(value: Tagged<T>) -> IndirectHandle<T> {
      // where T: Object { //Cannot add trait bound here, it breaks compilation
      // static_assert(is_subtype_v<T, Object>, "static type violation"); //Rust has trait bounds instead
      // The compiler should only pick this method if T is not Object.
      // static_assert(!std::is_same<Object, T>::value, "compiler error"); //Handled by Rust's type system
      let obj: Tagged<Object> = unsafe { std::mem::transmute_copy(&value) };
      let indirect_handle: IndirectHandle<Object> = GlobalHandles::create_object(obj);
      unsafe { std::mem::transmute_copy(&indirect_handle) } // Convert to IndirectHandle<T>
    }
  }

  impl<T> GlobalHandleVector<T> {
    pub fn pop(&mut self) -> Tagged<T> {
      let obj: Tagged<Object> =
        unsafe { std::mem::transmute_copy(&self.locations_.last().unwrap()) }; // Assuming locations_ is a Vec<Address>
      self.locations_.pop();
      unsafe { std::mem::transmute_copy(&obj) }
    }

    pub fn new_local_heap(local_heap: *mut LocalHeap) -> Self {
      let heap: *mut Heap = unsafe { (*local_heap).as_heap() };
      GlobalHandleVector::new_heap(heap)
    }

    pub fn new_heap(heap: *mut Heap) -> Self {
      GlobalHandleVector {
        locations_: Vec::new(), // Using a Vec<Address> as a replacement for StrongRootAllocator
        _phantom: PhantomData,
        heap_ptr: heap,
      }
    }
  }
}
