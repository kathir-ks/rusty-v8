// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::marker::PhantomData;

pub trait ThreadedListElement<T> {
    fn next(&mut self) -> &mut Option<Box<T>>;
    fn next_const(&self) -> &Option<Box<T>>;
}

pub struct ThreadedListTraits<T, U: ThreadedListElement<T>> {
    _phantom: PhantomData<(T, U)>,
}

impl<T, U: ThreadedListElement<T>> ThreadedListTraits<T, U> {
    pub fn next(t: &mut U) -> &mut Option<Box<T>> {
        t.next()
    }
    pub fn next_const(t: &U) -> &Option<Box<T>> {
        t.next_const()
    }
}

pub struct ThreadedListBase<T, U: ThreadedListElement<T>, TLTraits = ThreadedListTraits<T,U>, const UNSAFE_INSERTION: bool = false>
where TLTraits: 'static
{
    head: Option<Box<T>>,
    tail: *mut Option<Box<T>>,
    _phantom: PhantomData<(U, TLTraits)>,
}

impl<T, U: ThreadedListElement<T>, TLTraits: 'static, const UNSAFE_INSERTION: bool> ThreadedListBase<T, U, TLTraits, UNSAFE_INSERTION>
{
    pub fn new() -> Self {
        ThreadedListBase {
            head: None,
            tail: std::ptr::null_mut(),
            _phantom: PhantomData,
        }
    }

    pub fn add(&mut self, v: T) {
        self.ensure_valid_tail();
        let mut boxed_v = Box::new(v);
        let next_ptr = U::next(&mut boxed_v);

        assert!(next_ptr.is_none());

        if self.tail.is_null() {
            self.head = Some(boxed_v);
            self.tail = U::next(self.head.as_mut().unwrap());

        } else {
             unsafe {
                 *self.tail = Some(boxed_v);
                 self.tail = U::next((*self.tail).as_mut().unwrap());
             }
        }
        
         assert!(unsafe { (*self.tail).is_none() });
    }

    pub fn add_front(&mut self, v: T) {
        let mut boxed_v = Box::new(v);
        let next = U::next(&mut boxed_v);

        assert!(next.is_none());

        *next = self.head.take();
        
        if self.head.is_none() {
            self.tail = U::next(self.head.as_mut().unwrap());
        }

        self.head = Some(boxed_v);
    }

    pub fn add_after(after_this: &mut T, v: T)
    where
        [(); UNSAFE_INSERTION as usize]:,
    {
        let mut boxed_v = Box::new(v);
        let next_v = U::next(&mut boxed_v);
        assert!(next_v.is_none());

        let after_this_element: &mut U = unsafe { std::mem::transmute(after_this) };

        let mut next_after_this = U::next(after_this_element).take();

        *next_v = next_after_this;
        *U::next(after_this_element) = Some(boxed_v);
    }

    pub fn drop_head(&mut self) {
        assert!(self.head.is_some());

        let mut old_head = self.head.take().unwrap();
        self.head = U::next(&mut old_head).take();

        if self.head.is_none() {
            self.tail = std::ptr::null_mut();
        }
        *U::next(&mut old_head) = None;
    }

    pub fn contains(&self, v: &T) -> bool
    where T: PartialEq {
        for item in self.iter() {
            if *item == v {
                return true;
            }
        }
        false
    }

    pub fn append(&mut self, mut list: ThreadedListBase<T, U, TLTraits, UNSAFE_INSERTION>) {
        if list.is_empty() {
            return;
        }

        self.ensure_valid_tail();

        unsafe {
            if self.tail.is_null() {
                self.head = list.head.take();
                self.tail = list.tail;
            } else {
                *self.tail = list.head.take();
                self.tail = list.tail;
            }
        }
       
        list.clear();
    }

    pub fn prepend(&mut self, mut list: ThreadedListBase<T, U, TLTraits, UNSAFE_INSERTION>) {
        if list.head.is_none() {
            return;
        }

        self.ensure_valid_tail();

        let mut new_head = list.head.take();

        let list_tail = list.tail;
        unsafe {
            if self.tail.is_null() {
                self.tail = list_tail;
            }
            *list_tail = self.head.take();
        }

        self.head = new_head;
        list.clear();
    }

    pub fn truncate_at(
        &mut self,
        rem: &mut ThreadedListBase<T, U, TLTraits, UNSAFE_INSERTION>,
        v: &mut T,
    ) where T: PartialEq
    {
        assert!(rem.is_empty());

        let mut last: *mut T = std::ptr::null_mut();
        let mut current = self.head.as_mut();

        while let Some(node) = current {
            if node.as_ref() == v {
                break;
            }
            last = node.as_mut() as *mut Box<T> as *mut T;
            let node_element: &mut U = unsafe { std::mem::transmute(node.as_mut()) };
            current = U::next(node_element).as_mut();
        }
       
        assert_eq!(unsafe { &mut *last }, v);
       

        rem.head = Some(unsafe { Box::from_raw(v) });
        rem.tail = self.tail;

        if last.is_null() {
            assert_eq!(self.head.as_ref().unwrap().as_ref(), v);
            self.clear();
        } else {
            let last_element: &mut U = unsafe { std::mem::transmute(&mut *last) };
            self.tail = U::next(last_element);
            unsafe { *self.tail = None; }
        }
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = std::ptr::null_mut();
    }

    pub fn remove(&mut self, v: &T) -> bool
    where T: PartialEq
    {
        let mut current = self.first_mut();
        
        if let Some(head) = &mut current {
            if *head == v {
                self.drop_head();
                return true;
            }
        }
        

        self.ensure_valid_tail();

        let mut current_node: *mut Box<T> = match &mut self.head {
            Some(node) => node.as_mut() as *mut Box<T>,
            None => std::ptr::null_mut(),
        };
        
        while !current_node.is_null() {
            let next_node_ptr = unsafe { U::next(&mut *current_node) };

            if next_node_ptr.is_none() {
                 current_node = std::ptr::null_mut();
            } else {
                let next_node = unsafe { next_node_ptr.as_mut().unwrap().as_ref() };
                if next_node == v {
                    let next_node_mut = unsafe { next_node_ptr.as_mut().unwrap() };

                    let next_next_node = U::next(next_node_mut).take();
                    unsafe { *U::next(&mut *current_node) = next_next_node; }
                    *U::next(next_node_mut) = None;
                   
                    if U::next(next_node_mut) as *mut _ == self.tail as *mut _ {
                       let current_node_element: &mut U = unsafe { std::mem::transmute(&mut *current_node) };
                        self.tail = U::next(current_node_element);
                    }
                    return true;
                } else {
                    let next_node_element: &mut U = unsafe { std::mem::transmute(next_node_ptr.as_mut().unwrap().as_mut()) };
                    current_node = unsafe { U::next(next_node_element).as_mut().map(|x| x as *mut Box<T>).unwrap_or(std::ptr::null_mut()) };
                }
            }
        }
       
        false
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn first(&self) -> Option<&T> {
        self.head.as_ref().map(|boxed_t| boxed_t.as_ref())
    }

    fn first_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|boxed_t| boxed_t.as_mut())
    }

    pub fn len(&self) -> usize {
        let mut result = 0;
        for _ in self.iter() {
            result += 1;
        }
        result
    }

    fn at(&self, i: usize) -> Option<&T> {
        let mut it = self.iter();
        for _ in 0..i {
            it.next();
        }
        it.next()
    }

    pub fn verify(&self) -> bool {
        let mut last = self.first();

        if last.is_none() {
            assert!(self.tail.is_null());
        } else {
            let mut last_ptr: *const Box<T> = self.head.as_ref().unwrap().as_ref();
            let mut next = unsafe { U::next_const(&(*last_ptr)) };

            while !next.is_none() {
                 last_ptr = next.as_ref().unwrap().as_ref();
                 next = unsafe { U::next_const(&(*last_ptr)) };
            }
           
           unsafe {
                let tail_ptr: *mut Option<Box<T>> = self.tail;
                if !tail_ptr.is_null() {
                     let last_element : *const T = last.unwrap() as *const T;
                     let last_trait_element: *const U = last_element as *const U;
                    assert_eq!(U::next_const(&(*last_trait_element)) as *const _, self.tail as *const _);
                }
               
           }
            
        }
        true
    }

    fn ensure_valid_tail(&mut self)
    where
        [(); UNSAFE_INSERTION as usize]:,
    {
        if !UNSAFE_INSERTION {
            unsafe { assert!((*self.tail).is_none()); }
            return;
        }

        if self.tail.is_null() {
            return;
        }

        unsafe {
             if (*self.tail).is_none() {
                return;
             }
        }

        let mut last_ptr: *mut Box<T>;
        unsafe {
             last_ptr = (*self.tail).as_mut().map(|x| x as *mut Box<T>).unwrap_or(std::ptr::null_mut());
             if last_ptr.is_null() {
                return;
             }
        }

        unsafe {
            let mut next = U::next(&mut *last_ptr);
            while !next.is_none() {
                last_ptr = next.as_mut().map(|x| x as *mut Box<T>).unwrap_or(std::ptr::null_mut());
                next = U::next(&mut *last_ptr);
            }

            let last_element: &mut U = std::mem::transmute(&mut *last_ptr);
            self.tail = U::next(last_element);
        }
    }

    pub fn iter(&self) -> Iter<'_, T, U, TLTraits, UNSAFE_INSERTION> {
        Iter {
            list: self,
            current: match &self.head {
                Some(head) => Some(head.as_ref()),
                None => None,
            },
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T, U, TLTraits, UNSAFE_INSERTION> {
        IterMut {
            list: self,
            current: match &mut self.head {
                Some(head) => Some(head.as_mut()),
                None => None,
            },
        }
    }
}

impl<T, U: ThreadedListElement<T>, TLTraits: 'static, const UNSAFE_INSERTION: bool> Drop for ThreadedListBase<T, U, TLTraits, UNSAFE_INSERTION> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            let node_element: &mut U = unsafe { std::mem::transmute(node.as_mut()) };
            current = U::next(node_element).take();
        }
    }
}

impl<T, U: ThreadedListElement<T>, TLTraits: 'static, const UNSAFE_INSERTION: bool> Default for ThreadedListBase<T, U, TLTraits, UNSAFE_INSERTION> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Iter<'a, T, U: ThreadedListElement<T>, TLTraits, const UNSAFE_INSERTION: bool> {
    list: &'a ThreadedListBase<T, U, TLTraits, UNSAFE_INSERTION>,
    current: Option<&'a T>,
}

impl<'a, T, U: ThreadedListElement<T>, TLTraits, const UNSAFE_INSERTION: bool> Iterator
    for Iter<'a, T, U, TLTraits, UNSAFE_INSERTION>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(current_ref) => {
                let current_element: &U = unsafe { std::mem::transmute(current_ref) };
                let next_ptr = ThreadedListTraits::<T,U>::next_const(current_element);
                self.current = match next_ptr {
                    Some(next_box) => Some(next_box.as_ref()),
                    None => None,
                };
                Some(current_ref)
            }
            None => None,
        }
    }
}

pub struct IterMut<'a, T, U: ThreadedListElement<T>, TLTraits, const UNSAFE_INSERTION: bool> {
    list: &'a mut ThreadedListBase<T, U, TLTraits, UNSAFE_INSERTION>,
    current: Option<&'a mut T>,
}

impl<'a, T, U: ThreadedListElement<T>, TLTraits, const UNSAFE_INSERTION: bool> Iterator
    for IterMut<'a, T, U, TLTraits, UNSAFE_INSERTION>
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            Some(current_ref) => {
                let current_element: &mut U = unsafe { std::mem::transmute(current_ref) };
                let next_ptr = ThreadedListTraits::<T, U>::next(current_element);
                self.current = match next_ptr.as_mut() {
                    Some(next_box) => Some(next_box.as_mut()),
                    None => None,
                };
                Some(current_ref)
            }
            None => None,
        }
    }
}

pub struct EmptyBase {}

pub const UNSAFE_INSERTION: bool = true;

pub type ThreadedList<T, U: ThreadedListElement<T>, TLTraits = ThreadedListTraits<T,U>> = ThreadedListBase<T, U, TLTraits, false>;

pub type ThreadedListWithUnsafeInsertions<T, U: ThreadedListElement<T>, TLTraits = ThreadedListTraits<T,U>> =
    ThreadedListBase<T, U, TLTraits, UNSAFE_INSERTION>;