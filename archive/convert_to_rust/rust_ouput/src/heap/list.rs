// Converted from V8 C++ source files:
// Header: list.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod heap {

    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct List<T> {
        front: Option<Rc<RefCell<T>>>,
        back: Option<Rc<RefCell<T>>>,
        size: usize,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List {
                front: None,
                back: None,
                size: 0,
            }
        }

        pub fn push_back(&mut self, element: Rc<RefCell<T>>)
            where T: ListNodeTrait<T>
        {
            assert!(element.borrow().list_node_next().is_none());
            assert!(element.borrow().list_node_prev().is_none());

            if let Some(back) = &self.back {
                assert!(self.front.is_some());
                self.insert_after(element.clone(), back.clone());
            } else {
                self.add_first_element(element.clone());
            }
            self.size += 1;
        }

        pub fn push_front(&mut self, element: Rc<RefCell<T>>)
            where T: ListNodeTrait<T>
        {
            assert!(element.borrow().list_node_next().is_none());
            assert!(element.borrow().list_node_prev().is_none());

            if let Some(front) = &self.front {
                assert!(self.back.is_some());
                self.insert_before(element.clone(), front.clone());
            } else {
                self.add_first_element(element.clone());
            }
            self.size += 1;
        }

        pub fn remove(&mut self, element: Rc<RefCell<T>>>)
            where T: ListNodeTrait<T>
        {
            assert!(self.contains(&element));

            if let Some(back) = &self.back {
                if Rc::ptr_eq(back, &element) {
                    self.back = element.borrow().list_node_prev().clone();
                }
            }

            if let Some(front) = &self.front {
                if Rc::ptr_eq(front, &element) {
                    self.front = element.borrow().list_node_next().clone();
                }
            }

            let next = element.borrow().list_node_next().clone();
            let prev = element.borrow().list_node_prev().clone();

            if let Some(next) = &next {
                next.borrow_mut().set_list_node_prev(prev.clone());
            }
            if let Some(prev) = &prev {
                prev.borrow_mut().set_list_node_next(next.clone());
            }

            element.borrow_mut().set_list_node_prev(None);
            element.borrow_mut().set_list_node_next(None);

            self.size -= 1;
        }

        pub fn contains(&self, element: &Rc<RefCell<T>>>) -> bool
            where T: ListNodeTrait<T>
        {
            let mut it = self.front.clone();
            while let Some(node) = it {
                if Rc::ptr_eq(&node, element) {
                    return true;
                }
                it = node.borrow().list_node_next().clone();
            }
            false
        }

        pub fn is_empty(&self) -> bool {
            assert_eq!(self.size == 0, self.front.is_none());
            assert_eq!(self.size == 0, self.back.is_none());
            self.size == 0
        }

        pub fn front(&self) -> Option<Rc<RefCell<T>>> {
            self.front.clone()
        }

        pub fn back(&self) -> Option<Rc<RefCell<T>>> {
            self.back.clone()
        }

        pub fn size(&self) -> usize {
            self.size
        }

        fn add_first_element(&mut self, element: Rc<RefCell<T>>>
            where T: ListNodeTrait<T>
        {
            assert!(self.back.is_none());
            assert!(self.front.is_none());
            assert!(element.borrow().list_node_next().is_none());
            assert!(element.borrow().list_node_prev().is_none());

            element.borrow_mut().set_list_node_prev(None);
            element.borrow_mut().set_list_node_next(None);
            self.front = Some(element.clone());
            self.back = Some(element);
        }

        fn insert_after(&mut self, element: Rc<RefCell<T>>, other: Rc<RefCell<T>>>)
            where T: ListNodeTrait<T>
        {
            let other_next = other.borrow().list_node_next().clone();
            element.borrow_mut().set_list_node_next(other_next.clone());
            element.borrow_mut().set_list_node_prev(Some(other.clone()));
            other.borrow_mut().set_list_node_next(Some(element.clone()));

            if let Some(other_next) = &other_next {
                other_next.borrow_mut().set_list_node_prev(Some(element.clone()));
            } else {
                self.back = Some(element.clone());
            }
        }

        fn insert_before(&mut self, element: Rc<RefCell<T>>, other: Rc<RefCell<T>>>)
            where T: ListNodeTrait<T>
        {
            let other_prev = other.borrow().list_node_prev().clone();
            element.borrow_mut().set_list_node_next(Some(other.clone()));
            element.borrow_mut().set_list_node_prev(other_prev.clone());
            other.borrow_mut().set_list_node_prev(Some(element.clone()));

            if let Some(other_prev) = &other_prev {
                other_prev.borrow_mut().set_list_node_next(Some(element.clone()));
            } else {
                self.front = Some(element.clone());
            }
        }
    }

    pub trait ListNodeTrait<T> {
        fn list_node_next(&self) -> &Option<Rc<RefCell<T>>>;
        fn list_node_prev(&self) -> &Option<Rc<RefCell<T>>>;
        fn set_list_node_next(&mut self, next: Option<Rc<RefCell<T>>>);
        fn set_list_node_prev(&mut self, prev: Option<Rc<RefCell<T>>>);
    }

    pub struct ListNode<T> {
        next_: Option<Rc<RefCell<T>>>,
        prev_: Option<Rc<RefCell<T>>>,
    }

    impl<T> ListNode<T> {
        pub fn new() -> Self {
            let mut node = ListNode {
                next_: None,
                prev_: None,
            };
            node.initialize();
            node
        }

        pub fn next(&self) -> &Option<Rc<RefCell<T>>> {
            &self.next_
        }

        pub fn prev(&self) -> &Option<Rc<RefCell<T>>> {
            &self.prev_
        }

        fn initialize(&mut self) {
            self.next_ = None;
            self.prev_ = None;
        }
    }
}
