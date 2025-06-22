// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod base {
    use std::iter::{Rev};
    use std::ops::{Index, Deref};
    use std::slice;

    /// Equivalent of C++'s `std::iterator`.  Rust's built-in `Iterator`
    /// trait serves a similar purpose, but this struct might be useful
    /// for cases where direct compatibility with C++ iterator categories
    /// is desired.  It's mostly a marker type.
    pub struct Iterator<Category, T, Diff = isize, Pointer = *mut T, Reference = T> {
        _category: std::marker::PhantomData<Category>,
        _type: std::marker::PhantomData<T>,
        _diff: std::marker::PhantomData<Diff>,
        _pointer: std::marker::PhantomData<Pointer>,
        _reference: std::marker::PhantomData<Reference>,
    }

    /// Simulate iterator tags from C++
    pub struct BidirectionalIteratorTag;
    pub struct RandomAccessIteratorTag;


    /// A range defined by a pair of iterators.  Similar to C++'s
    /// `base::iterator_range`.
    pub struct IteratorRange<I> {
        begin: I,
        end: I,
    }

    impl<I> IteratorRange<I> {
        pub fn new(begin: I, end: I) -> Self {
            IteratorRange { begin, end }
        }
    }

    impl<I> IteratorRange<I>
    where
        I: Copy,
    {
        pub fn begin(&self) -> I {
            self.begin
        }
        pub fn end(&self) -> I {
            self.end
        }
        pub fn cbegin(&self) -> I {
            self.begin
        }
        pub fn cend(&self) -> I {
            self.end
        }
    }

    impl<'a, T> IteratorRange<&'a [T]> {
        pub fn rbegin(&self) -> Rev<slice::Iter<'a, T>> {
            self.end().iter().rev()
        }

        pub fn rend(&self) -> Rev<slice::Iter<'a, T>> {
            self.begin().iter().rev()
        }
    }

    impl<I: PartialEq> IteratorRange<I> {
        pub fn empty(&self) -> bool {
            self.cbegin() == self.cend()
        }
    }
    
    impl<I: std::ops::Sub<Output = I> + Copy> IteratorRange<I> {
        pub fn size(&self) -> I {
            self.cend() - self.cbegin()
        }
    }
    
    impl<I: Index<usize>> IteratorRange<I> {
      pub fn get(&self, n: usize) -> &I::Output {
        &self.begin()[n]
      }
    }

    /// Creates an `IteratorRange` from a begin and end iterator.
    pub fn make_iterator_range<I>(begin: I, end: I) -> IteratorRange<I> {
        IteratorRange::new(begin, end)
    }


    /// Iterator that dereferences a pointer to a pointer.  Equivalent
    /// to C++'s `DerefPtrIterator`.
    pub struct DerefPtrIterator<'a, T> {
        ptr: *const *const T,
        _phantom: std::marker::PhantomData<&'a T>,
    }

    impl<'a, T> DerefPtrIterator<'a, T> {
        pub fn new(ptr: *const *const T) -> Self {
            DerefPtrIterator {
                ptr,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<'a, T> Deref for DerefPtrIterator<'a, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { **self.ptr }
        }
    }

    impl<'a, T> DerefPtrIterator<'a, T> {
        pub fn increment(&mut self) -> &mut Self{
            unsafe {
              self.ptr = self.ptr.offset(1);
            }
            self
        }

        pub fn decrement(&mut self) -> &mut Self {
          unsafe {
            self.ptr = self.ptr.offset(-1);
          }
          self
        }
    }

    impl<'a, T> PartialEq for DerefPtrIterator<'a, T> {
        fn eq(&self, other: &Self) -> bool {
            self.ptr == other.ptr
        }
    }
    impl<'a, T> Eq for DerefPtrIterator<'a, T> {}

    /// Creates a reversed range.  Equivalent to C++'s `Reversed`.
    pub fn reversed<T>(t: &T) -> IteratorRange<std::slice::Iter<T::Item>>
    where
        T: std::ops::Deref,
        <T as std::ops::Deref>::Target: std::iter::IntoIterator,
    {
        let slice: &[T::Item] = t.deref().into_iter().collect::<Vec<_>>().as_slice();
        let iter = slice.iter();
        make_iterator_range(iter.clone(), iter)
    }

    pub fn reversed_range<T>(t: &IteratorRange<T>) -> IteratorRange<T> where T: Copy
    {
        make_iterator_range(t.end(), t.begin())
    }

    /// Creates a range that excludes the last element.  Equivalent to
    /// C++'s `IterateWithoutLast`.
    pub fn iterate_without_last<T>(t: &mut [T]) -> IteratorRange<*mut T> {
        assert!(!t.is_empty());
        let begin = t.as_mut_ptr();
        let end = unsafe { t.as_mut_ptr().add(t.len() - 1) };
        make_iterator_range(begin, end)
    }

    pub fn iterate_without_last_range<T>(t: &mut IteratorRange<*mut T>) -> IteratorRange<*mut T> {
        iterate_without_last(unsafe { std::slice::from_raw_parts_mut(t.begin, (t.end as usize - t.begin as usize)/std::mem::size_of::<*mut T>() + 1)})
    }

    /// Creates a range that excludes the first element.  Equivalent to
    /// C++'s `IterateWithoutFirst`.
    pub fn iterate_without_first<T>(t: &mut [T]) -> IteratorRange<*mut T> {
        assert!(!t.is_empty());
        let begin = unsafe { t.as_mut_ptr().add(1) };
        let end = unsafe { t.as_mut_ptr().add(t.len()) };
        make_iterator_range(begin, end)
    }

    pub fn iterate_without_first_range<T>(t: &mut IteratorRange<*mut T>) -> IteratorRange<*mut T> {
        iterate_without_first(unsafe { std::slice::from_raw_parts_mut(t.begin, (t.end as usize - t.begin as usize)/std::mem::size_of::<*mut T>() + 1)})
    }

    /// An iterator that wraps multiple iterators.  Used by the `zip`
    /// function.  Equivalent to C++'s `TupleIterator`.
    pub struct TupleIterator<Iterators> {
        iters: Iterators,
    }

    impl<Iterators> TupleIterator<Iterators> {
        pub fn new(iters: Iterators) -> Self {
            TupleIterator { iters }
        }
    }

    //This is a placeholder for a more complex implementation that allows comparison.
    impl<A,B> PartialEq for TupleIterator<(A,B)> where A: PartialEq<A>, B: PartialEq<B> {
      fn eq(&self, other: &Self) -> bool {
          true
      }
    }

    /// Creates an iterator range from multiple containers.  Can be used
    /// to iterate over multiple containers at once.  Equivalent to
    /// C++'s `zip`.
    pub fn zip<A, B>(a: A, b: B) -> IteratorRange<(A::IntoIter, B::IntoIter)>
    where
        A: IntoIterator,
        B: IntoIterator,
    {
        let a_iter = a.into_iter();
        let b_iter = b.into_iter();

        make_iterator_range((a_iter, b_iter), (a_iter, b_iter)) // Placeholders
    }

}