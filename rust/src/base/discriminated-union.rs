// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod base {
  use std::{
    any::Any,
    fmt::{Debug, Display},
    marker::PhantomData,
    mem::{align_of, size_of, MaybeUninit},
    ops::{Deref, DerefMut},
  };

  /// A variant-like discriminated union type, which takes a discriminating enum
  /// and a set of types. The enum must have as many elements as the number of
  /// types, with each enum value corresponding to one type in the set.
  ///
  /// Example usage:
  ///
  /// ```rust
  /// #[derive(Debug, PartialEq)]
  /// enum FooType {
  ///   Bar,
  ///   Baz,
  /// }
  /// #[derive(Debug, PartialEq)]
  /// struct Bar { value: i32 }
  /// #[derive(Debug, PartialEq)]
  /// struct Baz { value: String }
  ///
  /// // FooType::Bar and FooType::Baz match Bar and Baz, respectively.
  /// let union = DiscriminatedUnion::<FooType, (Bar, Baz)>::new(FooType::Bar, Bar { value: 10 });
  ///
  /// match union.tag() {
  ///   FooType::Bar => {
  ///     let bar = union.get::<Bar>();
  ///     assert_eq!(bar, &Bar { value: 10 });
  ///   }
  ///   FooType::Baz => {
  ///     // unreachable in this example
  ///   }
  /// }
  /// ```
  pub struct DiscriminatedUnion<TagEnum: Copy + PartialEq + 'static, Ts> {
    data_: AlignedStorage<Ts>,
    tag_: u8,
    _phantom: PhantomData<TagEnum>,
  }

  impl<TagEnum: Copy + PartialEq + 'static, Ts> DiscriminatedUnion<TagEnum, Ts> {
    pub fn new<T: 'static>(tag: TagEnum, data: T) -> Self
    where
      Ts: TypesTuple<T = T>,
    {
      let index = Ts::index_of::<T>().unwrap();
      let tag_u8 = index as u8;
      let mut data_: AlignedStorage<Ts> = AlignedStorage::new();
      unsafe {
        data_.write(data, index);
      }

      DiscriminatedUnion {
        data_,
        tag_: tag_u8,
        _phantom: PhantomData,
      }
    }

    pub fn from_type<T: 'static>(data: T) -> Self
    where
      Ts: TypesTuple<T = T>,
      TagEnum: From<usize>,
    {
      let index = Ts::index_of::<T>().unwrap();
      let tag: TagEnum = From::from(index);
      let tag_u8 = index as u8;
      let mut data_: AlignedStorage<Ts> = AlignedStorage::new();
      unsafe {
        data_.write(data, index);
      }
      DiscriminatedUnion {
        data_,
        tag_: tag_u8,
        _phantom: PhantomData,
      }
    }

    pub fn tag(&self) -> TagEnum
    where
      TagEnum: From<u8>,
    {
      From::from(self.tag_)
    }

    pub fn get<T: 'static>(&self) -> &T
    where
      Ts: TypesTuple<T = T>,
    {
      let index = Ts::index_of::<T>().unwrap();
      assert_eq!(self.tag() as u8, index as u8);
      unsafe { self.data_.read_ref::<T>(index) }
    }

    pub fn get_mut<T: 'static>(&mut self) -> &mut T
    where
      Ts: TypesTuple<T = T>,
    {
      let index = Ts::index_of::<T>().unwrap();
      assert_eq!(self.tag() as u8, index as u8);
      unsafe { self.data_.read_mut::<T>(index) }
    }
  }

  /// A helper struct to allocate aligned storage for the union.
  struct AlignedStorage<Ts> {
    data_: MaybeUninit<[u8; MAX_SIZE]>,
    _phantom: PhantomData<Ts>,
  }

  impl<Ts> AlignedStorage<Ts> {
    const fn new() -> Self {
      AlignedStorage {
        data_: MaybeUninit::uninit(),
        _phantom: PhantomData,
      }
    }

    unsafe fn write<T>(&mut self, value: T, index: usize)
    where
      Ts: TypesTuple<T = T>,
    {
      let ptr = self.data_.as_mut_ptr() as *mut u8;
      let aligned_ptr = ptr.add(0) as *mut T;
      aligned_ptr.write(value);
    }

    unsafe fn read_ref<T>(&self, index: usize) -> &T
    where
      Ts: TypesTuple<T = T>,
    {
      let ptr = self.data_.as_ptr() as *const u8;
      let aligned_ptr = ptr.add(0) as *const T;
      &*aligned_ptr
    }

    unsafe fn read_mut<T>(&mut self, index: usize) -> &mut T
    where
      Ts: TypesTuple<T = T>,
    {
      let ptr = self.data_.as_mut_ptr() as *mut u8;
      let aligned_ptr = ptr.add(0) as *mut T;
      &mut *aligned_ptr
    }
  }

  const MAX_SIZE: usize = {
    let mut max_size = 0;
    macro_rules! calculate_max_size {
        ($t:ty) => {
            {
                let size = size_of::<$t>();
                if size > max_size {
                    max_size = size;
                }
            }
        };
    }

    calculate_max_size!(i32);
    calculate_max_size!(f64);
    calculate_max_size!(String);
    max_size
  };

  //A trait for getting the index of a type in a tuple
  pub trait TypesTuple {
    type T: 'static;
    fn index_of<T: 'static>() -> Option<usize>;
  }

  impl TypesTuple for (i32, f64, String) {
    type T = Self;

    fn index_of<T: 'static>() -> Option<usize> {
      if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
        Some(0)
      } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>() {
        Some(1)
      } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<String>() {
        Some(2)
      } else {
        None
      }
    }
  }
}