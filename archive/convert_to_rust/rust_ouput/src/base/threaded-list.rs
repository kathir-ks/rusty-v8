// Converted from V8 C++ source files:
// Header: threaded-list.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    use std::ptr::null_mut;
    use std::marker::PhantomData;

    pub struct ThreadedListTraits<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> ThreadedListTraits<T> {
        pub fn next(t: *mut T) -> *mut *mut T {
            unsafe { (*t).next_ptr() }
        }
        pub fn start(t: *mut *mut T) -> *mut *mut T {
            t
        }
        pub fn start_const(t: *const *mut T) -> *const *mut T {
            t
        }
    }

    pub trait ThreadedListEntry<T> {
        fn next_ptr(&mut self) -> *mut *mut T;
    }

    pub struct ThreadedListBase<T, BaseClass, TLTraits = ThreadedListTraits<T>, const K_SUPPORTS_UNSAFE_INSERTION: bool = false>
    where
        TLTraits: 'static,
    {
        head_: *mut T,
        tail_: *mut *mut T,
        _base: PhantomData<BaseClass>,
        _traits: PhantomData<TLTraits>,
    }

    impl<T, BaseClass, TLTraits, const K_SUPPORTS_UNSAFE_INSERTION: bool> ThreadedListBase<T, BaseClass, TLTraits, K_SUPPORTS_UNSAFE_INSERTION>
    where
        TLTraits: 'static,
        T: ThreadedListEntry<T>,
    {
        pub fn new() -> Self {
            Self {
                head_: null_mut(),
                tail_: &mut null_mut() as *mut *mut T,
                _base: PhantomData,
                _traits: PhantomData,
            }
        }

        pub fn add(&mut self, v: *mut T) {
            self.ensure_valid_tail();
            assert!((*self.tail_).is_null());
            assert!(unsafe { (*ThreadedListTraits::<T>::next(v)).is_null() });
            unsafe {
                *self.tail_ = v;
                self.tail_ = ThreadedListTraits::<T>::next(v);
            }
            assert!((*self.tail_).is_null());
        }

        pub fn add_front(&mut self, v: *mut T) {
            assert!(unsafe { (*ThreadedListTraits::<T>::next(v)).is_null() });
            assert!(!v.is_null());
            let next: *mut *mut T = ThreadedListTraits::<T>::next(v);

            unsafe {
                *next = self.head_;
                if self.head_.is_null() {
                    self.tail_ = next;
                }
                self.head_ = v;
            }
        }

        pub fn add_after(after_this: *mut T, v: *mut T)
        where
            [(); K_SUPPORTS_UNSAFE_INSERTION as usize]:,
        {
            assert!(unsafe { (*ThreadedListTraits::<T>::next(v)).is_null() });
            unsafe {
                *ThreadedListTraits::<T>::next(v) = *ThreadedListTraits::<T>::next(after_this);
                *ThreadedListTraits::<T>::next(after_this) = v;
            }
        }

        pub fn drop_head(&mut self) {
            assert!(!self.head_.is_null());

            let old_head: *mut T = self.head_;
            unsafe {
                self.head_ = *ThreadedListTraits::<T>::next(self.head_);
                if self.head_.is_null() {
                    self.tail_ = &mut self.head_ as *mut *mut T;
                }
                *ThreadedListTraits::<T>::next(old_head) = null_mut();
            }
        }

        pub fn contains(&self, v: *mut T) -> bool {
            let mut it: Iterator<T, TLTraits> = self.begin();
            let end: Iterator<T, TLTraits> = self.end();
            while it != end {
                if *it == v {
                    return true;
                }
                it.increment();
            }
            false
        }

        pub fn append(&mut self, mut list: ThreadedListBase<T, BaseClass, TLTraits, K_SUPPORTS_UNSAFE_INSERTION>) {
            if list.is_empty() {
                return;
            }

            self.ensure_valid_tail();
            unsafe {
                *self.tail_ = list.head_;
                self.tail_ = list.tail_;
            }
            list.clear();
        }

        pub fn prepend(&mut self, mut list: ThreadedListBase<T, BaseClass, TLTraits, K_SUPPORTS_UNSAFE_INSERTION>) {
            if list.head_.is_null() {
                return;
            }

            self.ensure_valid_tail();
            let new_head: *mut T = list.head_;
            unsafe {
                *list.tail_ = self.head_;
            }
            if self.head_.is_null() {
                self.tail_ = list.tail_;
            }
            self.head_ = new_head;
            list.clear();
        }

        pub fn truncate_at(&mut self, rem: &mut ThreadedListBase<T, BaseClass, TLTraits, K_SUPPORTS_UNSAFE_INSERTION>, v: *mut T) {
            assert!(!v.is_null());
            assert!(rem.is_empty());
            let mut it: Iterator<T, TLTraits> = self.begin();
            let end: Iterator<T, TLTraits> = self.end();

            let mut last: *mut T = null_mut();
            while it != end {
                if *it == v {
                    break;
                }
                last = *it;
                it.increment();
            }
            assert_eq!(v, *it);

            rem.head_ = v;
            rem.tail_ = self.tail_;

            if last.is_null() {
                assert_eq!(self.head_, v);
                self.clear();
            } else {
                unsafe {
                    self.tail_ = ThreadedListTraits::<T>::next(last);
                    *self.tail_ = null_mut();
                }
            }
        }

        pub fn clear(&mut self) {
            self.head_ = null_mut();
            self.tail_ = &mut self.head_ as *mut *mut T;
        }

        pub fn remove(&mut self, v: *mut T) -> bool {
            let mut current: *mut T = self.first();
            if current == v {
                self.drop_head();
                return true;
            }

            self.ensure_valid_tail();
            while !current.is_null() {
                unsafe {
                    let next: *mut T = *ThreadedListTraits::<T>::next(current);
                    if next == v {
                        *ThreadedListTraits::<T>::next(current) = *ThreadedListTraits::<T>::next(next);
                        *ThreadedListTraits::<T>::next(next) = null_mut();

                        if ThreadedListTraits::<T>::next(next) == self.tail_ {
                            self.tail_ = ThreadedListTraits::<T>::next(current);
                        }
                        return true;
                    }
                    current = next;
                }
            }
            false
        }

        pub fn begin(&self) -> Iterator<T, TLTraits> {
            Iterator::new(ThreadedListTraits::<T>::start(unsafe { &self.head_ as *const *mut T as *mut *mut T }))
        }

        pub fn end(&self) -> Iterator<T, TLTraits> {
            self.ensure_valid_tail();
            Iterator::new(self.tail_)
        }

        pub fn rewind(&mut self, reset_point: Iterator<T, TLTraits>) {
            self.tail_ = reset_point.entry_;
            unsafe { *self.tail_ = null_mut() };
        }

        pub fn move_tail(&mut self, from_list: &mut ThreadedListBase<T, BaseClass, TLTraits, K_SUPPORTS_UNSAFE_INSERTION>, from_location: Iterator<T, TLTraits>) {
            if from_list.end() != from_location {
                assert!((*self.tail_).is_null());
                unsafe {
                    *self.tail_ = *from_location.entry_;
                    self.tail_ = from_list.tail_;
                }
                from_list.rewind(from_location);
            }
        }

        pub fn remove_at(&mut self, mut it: Iterator<T, TLTraits>) -> Iterator<T, TLTraits> {
            unsafe {
                if *it.entry_ == self.head_ {
                    self.drop_head();
                    return self.begin();
                } else if self.tail_ == ThreadedListTraits::<T>::next(*it.entry_) {
                    self.tail_ = it.entry_;
                    *it.entry_ = null_mut();
                    return self.end();
                } else {
                    let old_entry: *mut T = *it.entry_;
                    *it.entry_ = *ThreadedListTraits::<T>::next(*it.entry_);
                    *ThreadedListTraits::<T>::next(old_entry) = null_mut();
                    return Iterator::new(it.entry_);
                }
            }
        }

        pub fn is_empty(&self) -> bool {
            self.head_.is_null()
        }

        pub fn first(&self) -> *mut T {
            self.head_
        }

        pub fn length_for_test(&self) -> i32 {
            let mut result: i32 = 0;
            let mut t: Iterator<T, TLTraits> = self.begin();
            let end: Iterator<T, TLTraits> = self.end();
            while t != end {
                result += 1;
                t.increment();
            }
            result
        }

        pub fn at_for_test(&self, i: i32) -> *mut T {
            let mut t: Iterator<T, TLTraits> = self.begin();
            let end: Iterator<T, TLTraits> = self.end();
            let mut counter = 0;

            while t != end {
              if counter == i {
                return *t;
              }
              t.increment();
              counter += 1;
            }
            null_mut()
        }

        pub fn verify(&self) -> bool {
            let mut last: *mut T = self.first();
            if last.is_null() {
                assert_eq!(&(self.head_ as *mut T), self.tail_ as *mut *mut T);
            } else {
                unsafe {
                    while !(*ThreadedListTraits::<T>::next(last)).is_null() {
                        last = *ThreadedListTraits::<T>::next(last);
                    }
                    assert_eq!(ThreadedListTraits::<T>::next(last), self.tail_);
                }
            }
            true
        }

        #[inline]
        pub fn ensure_valid_tail(&self)
        where
            [(); K_SUPPORTS_UNSAFE_INSERTION as usize]:,
        {
            if !K_SUPPORTS_UNSAFE_INSERTION {
                unsafe {
                  assert!((*self.tail_).is_null());
                }
                return;
            }

            unsafe {
              if (*self.tail_).is_null() {
                  return;
              }

              let mut last: *mut T = *self.tail_;
              if !last.is_null() {
                  while !(*ThreadedListTraits::<T>::next(last)).is_null() {
                      last = *ThreadedListTraits::<T>::next(last);
                  }
                  self.tail_ = ThreadedListTraits::<T>::next(last);
              }
            }
        }
    }

    impl<T, BaseClass, TLTraits, const K_SUPPORTS_UNSAFE_INSERTION: bool> Drop for ThreadedListBase<T, BaseClass, TLTraits, K_SUPPORTS_UNSAFE_INSERTION>
    where
        TLTraits: 'static,
        T: ThreadedListEntry<T>,
    {
        fn drop(&mut self) {
            self.clear();
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Iterator<T, TLTraits>
    where
      TLTraits: 'static,
    {
        entry_: *mut *mut T,
        _traits: PhantomData<TLTraits>,
    }

    impl<T, TLTraits> Iterator<T, TLTraits>
    where
        TLTraits: 'static,
    {
        pub fn new(entry: *mut *mut T) -> Self {
            Self { entry_: entry, _traits: PhantomData }
        }

        pub fn increment(&mut self) -> &mut Self {
            unsafe {
                self.entry_ = ThreadedListTraits::<T>::next(*self.entry_);
            }
            self
        }

        pub fn equals(&self, other: &Self) -> bool {
            self.entry_ == other.entry_
        }

        pub fn deref(&self) -> *mut T {
            unsafe { *self.entry_ }
        }

        pub fn insert_before(&mut self, value: *mut T) {
            unsafe {
                let old_entry_value: *mut T = *self.entry_;
                *self.entry_ = value;
                self.entry_ = ThreadedListTraits::<T>::next(value);
                *self.entry_ = old_entry_value;
            }
        }

        pub fn is_null(&self) -> bool {
            self.entry_.is_null()
        }
    }

    impl<T, TLTraits> PartialEq for Iterator<T, TLTraits>
    where
        TLTraits: 'static,
    {
        fn eq(&self, other: &Self) -> bool {
            self.entry_ == other.entry_
        }
    }

    impl<T, TLTraits> Eq for Iterator<T, TLTraits>
    where
        TLTraits: 'static,
    {}

    impl<T, TLTraits> std::ops::Deref for Iterator<T, TLTraits>
    where
        TLTraits: 'static,
    {
        type Target = *mut T;

        fn deref(&self) -> &Self::Target {
            unsafe { &(*self.entry_) }
        }
    }

    pub struct ConstIterator<T, TLTraits>
    where
        TLTraits: 'static,
    {
        entry_: *const *mut T,
        _traits: PhantomData<TLTraits>,
    }

    impl<T, TLTraits> ConstIterator<T, TLTraits>
    where
        TLTraits: 'static,
    {
        pub fn new(entry: *const *mut T) -> Self {
            Self { entry_: entry, _traits: PhantomData }
        }

        pub fn from_iterator(iterator: &Iterator<T, TLTraits>) -> Self {
            Self { entry_: iterator.entry_ as *const *mut T, _traits: PhantomData }
        }

        pub fn increment(&mut self) -> &mut Self {
            unsafe {
                self.entry_ = ThreadedListTraits::<T>::start_const(ThreadedListTraits::<T>::next(*(self.entry_ as *mut *mut T)) as *const *mut T);
            }
            self
        }

        pub fn equals(&self, other: &Self) -> bool {
            self.entry_ == other.entry_
        }

        pub fn deref(&self) -> *const T {
            unsafe { *self.entry_ }
        }
    }

    impl<T, TLTraits> PartialEq for ConstIterator<T, TLTraits>
    where
        TLTraits: 'static,
    {
        fn eq(&self, other: &Self) -> bool {
            self.entry_ == other.entry_
        }
    }

    impl<T, TLTraits> Eq for ConstIterator<T, TLTraits>
    where
        TLTraits: 'static,
    {}

    impl<T, TLTraits> std::ops::Deref for ConstIterator<T, TLTraits>
    where
        TLTraits: 'static,
    {
        type Target = *const T;

        fn deref(&self) -> &Self::Target {
            unsafe { &(*self.entry_) }
        }
    }

    pub struct EmptyBase {}

    const K_UNSAFE_INSERTION: bool = true;

    pub type ThreadedList<T, TLTraits = ThreadedListTraits<T>> = ThreadedListBase<T, EmptyBase, TLTraits, false>;

    pub type ThreadedListWithUnsafeInsertions<T, TLTraits = ThreadedListTraits<T>> =
        ThreadedListBase<T, EmptyBase, TLTraits, K_UNSAFE_INSERTION>;
}
