// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod compiler {
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A generic stack implemented with a singly-linked list, which results in an
    /// O(1) copy operation. It can be used to model immutable lists like those in
    /// functional languages. Compared to typical functional lists, this also caches
    /// the length of the list in each node.
    /// Note: The underlying implementation is mutable, so if you want to use this as
    /// an immutable list, make sure to create a copy by passing it by value and
    /// operate on the copy.
    #[derive(Clone)]
    pub struct FunctionalList<A> {
        elements_: Option<Rc<Cons<A>>>,
    }

    struct Cons<A> {
        top: A,
        rest: Option<Rc<Cons<A>>>,
        size: usize,
    }

    impl<A> FunctionalList<A> {
        pub fn new() -> Self {
            FunctionalList { elements_: None }
        }

        pub fn trivially_equals(&self, other: &Self) -> bool {
            match (&self.elements_, &other.elements_) {
                (Some(s), Some(o)) => Rc::ptr_eq(s, o),
                (None, None) => true,
                _ => false,
            }
        }

        pub fn front(&self) -> &A {
            match &self.elements_ {
                Some(cons) => &cons.top,
                None => panic!("List is empty"),
            }
        }

        pub fn rest(&self) -> Self {
            match &self.elements_ {
                Some(cons) => FunctionalList {
                    elements_: cons.rest.clone(),
                },
                None => FunctionalList::new(),
            }
        }

        pub fn drop_front(&mut self) {
            match &self.elements_.take() {
                Some(cons) => {
                    self.elements_ = cons.rest.clone();
                }
                None => panic!("List is empty"),
            }
        }

        pub fn push_front(&mut self, a: A) {
            self.elements_ = Some(Rc::new(Cons {
                top: a,
                rest: self.elements_.clone(),
                size: self.size() + 1,
            }));
        }

        pub fn push_front_with_hint(&mut self, a: A, hint: FunctionalList<A>)
        where
            A: PartialEq,
        {
            if hint.size() == self.size() + 1
                && hint.front() == &a
                && hint.rest() == *self
            {
                *self = hint;
            } else {
                self.push_front(a);
            }
        }

        // Drop elements until the current stack is equal to the tail shared with
        // {other}. The shared tail must not only be equal, but also refer to the
        // same memory.
        pub fn reset_to_common_ancestor(&mut self, mut other: FunctionalList<A>)
        where A: PartialEq + Clone
        {
            while other.size() > self.size() {
                other.drop_front();
            }
            while other.size() < self.size() {
                self.drop_front();
            }
            while !self.trivially_equals(&other) {
                self.drop_front();
                other.drop_front();
            }
        }

        pub fn size(&self) -> usize {
            match &self.elements_ {
                Some(cons) => cons.size,
                None => 0,
            }
        }

        pub fn clear(&mut self) {
            self.elements_ = None;
        }

        pub fn iter(&self) -> FunctionalListIterator<A> {
            FunctionalListIterator {
                current_: self.elements_.clone(),
            }
        }
    }

    impl<A: PartialEq> PartialEq for FunctionalList<A> {
        fn eq(&self, other: &Self) -> bool {
            if self.size() != other.size() {
                return false;
            }
            let mut it = self.iter();
            let mut other_it = other.iter();
            loop {
                match (it.next(), other_it.next()) {
                    (Some(val), Some(other_val)) => {
                        if val != other_val {
                            return false;
                        }
                    }
                    (None, None) => return true,
                    _ => return false
                }
            }
        }
    }

    impl<A: PartialEq> Eq for FunctionalList<A> {}

    impl<A> Default for FunctionalList<A> {
        fn default() -> Self {
            Self::new()
        }
    }

    pub struct FunctionalListIterator<A> {
        current_: Option<Rc<Cons<A>>>,
    }

    impl<'a, A> Iterator for FunctionalListIterator<A> {
        type Item = &'a A;

        fn next(&mut self) -> Option<Self::Item> {
            match self.current_.take() {
                Some(cons) => {
                    self.current_ = cons.rest.clone();
                    Some(&cons.top)
                }
                None => None,
            }
        }
    }
}