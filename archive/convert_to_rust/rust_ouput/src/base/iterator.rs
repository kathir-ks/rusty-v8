// Converted from V8 C++ source files:
// Header: iterator.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    use std::iter::Rev;
    use std::{
        iter,
        marker::PhantomData,
        ops::{Deref, DerefMut},
        ptr,
    };

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
    pub struct iterator<Category, Type, Diff = isize, Pointer = *mut Type, Reference = Type> {
        _category: std::marker::PhantomData<Category>,
        _type: std::marker::PhantomData<Type>,
        _diff: std::marker::PhantomData<Diff>,
        _pointer: std::marker::PhantomData<Pointer>,
        _reference: std::marker::PhantomData<Reference>,
    }

    impl<Category, Type, Diff, Pointer, Reference> iterator<Category, Type, Diff, Pointer, Reference> {
        pub fn new() -> Self {
            iterator {
                _category: std::marker::PhantomData,
                _type: std::marker::PhantomData,
                _diff: std::marker::PhantomData,
                _pointer: std::marker::PhantomData,
                _reference: std::marker::PhantomData,
            }
        }
    }

    pub struct iterator_range<ForwardIterator>
    where
        ForwardIterator: Iterator + Clone,
    {
        begin_: ForwardIterator,
        end_: ForwardIterator,
        _phantom: PhantomData<ForwardIterator>,
    }

    impl<ForwardIterator> iterator_range<ForwardIterator>
    where
        ForwardIterator: Iterator + Clone,
    {
        pub fn new(begin: ForwardIterator, end: ForwardIterator) -> Self {
            iterator_range {
                begin_: begin,
                end_: end,
                _phantom: PhantomData,
            }
        }

        pub fn begin(&self) -> ForwardIterator {
            self.begin_.clone()
        }

        pub fn end(&self) -> ForwardIterator {
            self.end_.clone()
        }

        pub fn cbegin(&self) -> ForwardIterator {
            self.begin_.clone()
        }

        pub fn cend(&self) -> ForwardIterator {
            self.end_.clone()
        }

        pub fn rbegin(&self) -> Rev<ForwardIterator>
        where
            ForwardIterator: DoubleEndedIterator,
        {
            self.end_.clone().rev()
        }

        pub fn rend(&self) -> Rev<ForwardIterator>
        where
            ForwardIterator: DoubleEndedIterator,
        {
            self.begin_.clone().rev()
        }

        pub fn empty(&self) -> bool
        where
            ForwardIterator: PartialEq,
        {
            self.cbegin() == self.cend()
        }

        // Random Access iterators only.
        // Implementing operator[] requires knowing the output type of the iterator.
        // Given the limitations and the available context, this is not implemented.
        // Implement size when ForwardIterator implements ExactSizeIterator
        pub fn size(&self) -> usize
        where
            ForwardIterator: ExactSizeIterator,
        {
            self.cend().len()
        }
    }

    pub fn make_iterator_range<ForwardIterator>(
        begin: ForwardIterator,
        end: ForwardIterator,
    ) -> iterator_range<ForwardIterator>
    where
        ForwardIterator: Iterator + Clone,
    {
        iterator_range::new(begin, end)
    }

    pub struct DerefPtrIterator<'a, T> {
        ptr: *const *mut T,
        _phantom: PhantomData<&'a T>,
    }

    impl<'a, T> DerefPtrIterator<'a, T> {
        pub fn new(ptr: *const *mut T) -> Self {
            DerefPtrIterator {
                ptr,
                _phantom: PhantomData,
            }
        }
    }

    impl<'a, T> Iterator for DerefPtrIterator<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.ptr.is_null() {
                return None;
            }
            unsafe {
                let value = **self.ptr;
                if value.is_null() {
                    return None;
                }
                self.ptr = self.ptr.offset(1);
                Some(&mut *value)
            }
        }
    }

    impl<'a, T> DoubleEndedIterator for DerefPtrIterator<'a, T> {
        fn next_back(&mut self) -> Option<Self::Item> {
            // This is a placeholder. A real implementation would require keeping track of the
            // beginning of the iterator and decrementing from the end.
            None
        }
    }

    impl<'a, T> Deref for DerefPtrIterator<'a, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &**self.ptr }
        }
    }

    impl<'a, T> DerefMut for DerefPtrIterator<'a, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { &mut **self.ptr }
        }
    }

    impl<'a, T> PartialEq for DerefPtrIterator<'a, T> {
        fn eq(&self, other: &Self) -> bool {
            self.ptr == other.ptr
        }
    }

    impl<'a, T> Eq for DerefPtrIterator<'a, T> {}

    pub fn Reversed<T>(t: &T) -> iterator_range<Rev<std::slice::Iter<T::Item>>>
    where
        T: std::ops::Deref,
        T::Target: AsRef<[T::Item]>,
    {
        let slice = t.deref().as_ref();
        let rev_iter = slice.iter().rev();
        let begin = rev_iter.clone();
        let end = rev_iter;
        make_iterator_range(begin, end)
    }

    pub fn Reversed<T>(t: &iterator_range<T>) -> iterator_range<Rev<T>>
    where
        T: Iterator + DoubleEndedIterator + Clone,
    {
        let begin = t.begin().rev();
        let end = t.end().rev();
        make_iterator_range(begin, end)
    }

    pub fn IterateWithoutLast<T>(t: &T) -> iterator_range<std::slice::Iter<T::Item>>
    where
        T: std::ops::Deref,
        T::Target: AsRef<[T::Item]>,
    {
        let slice = t.deref().as_ref();
        assert!(!slice.is_empty());
        let begin = slice.iter();
        let end = slice[..slice.len() - 1].iter();
        make_iterator_range(begin, end)
    }

    pub fn IterateWithoutLast<T>(t: &iterator_range<T>) -> iterator_range<T>
    where
        T: Iterator + Clone,
    {
        let mut range_copy = iterator_range::new(t.begin(), t.end());
        IterateWithoutLast(&range_copy)
    }

    pub fn IterateWithoutFirst<T>(t: &T) -> iterator_range<std::slice::Iter<T::Item>>
    where
        T: std::ops::Deref,
        T::Target: AsRef<[T::Item]>,
    {
        let slice = t.deref().as_ref();
        assert!(!slice.is_empty());
        let begin = slice[1..].iter();
        let end = slice.iter().skip(slice.len());
        make_iterator_range(begin, end)
    }

    pub fn IterateWithoutFirst<T>(t: &iterator_range<T>) -> iterator_range<T>
    where
        T: Iterator + Clone,
    {
        let mut range_copy = iterator_range::new(t.begin(), t.end());
        IterateWithoutFirst(&range_copy)
    }

    pub struct TupleIterator<Iterators> {
        its_: Iterators,
    }

    impl<Iterators> TupleIterator<Iterators> {
        pub fn new(its_: Iterators) -> Self {
            TupleIterator { its_ }
        }
    }

    impl<I1, I2, T1, T2> Iterator for TupleIterator<(I1, I2)>
    where
        I1: Iterator<Item = T1>,
        I2: Iterator<Item = T2>,
    {
        type Item = (T1, T2);

        fn next(&mut self) -> Option<Self::Item> {
            match (self.its_.0.next(), self.its_.1.next()) {
                (Some(v1), Some(v2)) => Some((v1, v2)),
                _ => None,
            }
        }
    }

    impl<I1, I2, T1, T2> PartialEq for TupleIterator<(I1, I2)>
    where
        I1: Iterator<Item = T1> + PartialEq,
        I2: Iterator<Item = T2> + PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            true //Placeholder. Implement only if needed.
        }
    }

    pub fn zip<C1, C2>(c1: C1, c2: C2) -> iterator_range<TupleIterator<(C1::IntoIter, C2::IntoIter)>>
    where
        C1: IntoIterator,
        C2: IntoIterator,
    {
        let tuple_it = TupleIterator::new((c1.into_iter(), c2.into_iter()));
        make_iterator_range(tuple_it, TupleIterator::new((
            std::iter::empty().into_iter(),
            std::iter::empty().into_iter(),
        )))
    }
}
