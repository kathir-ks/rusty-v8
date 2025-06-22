pub mod base {
    pub mod platform {
        pub struct ConditionVariable {}

        impl ConditionVariable {
            pub fn new() -> Self {
                ConditionVariable {}
            }

            pub fn notify_one(&self) {}
            pub fn notify_all(&self) {}
            pub fn wait(&self, _lock: &mut std::sync::MutexGuard<'_, ()>) {}
            // TODO(you): Add more methods as needed.
        }

        pub struct Mutex {}

        impl Mutex {
            pub fn new() -> Self {
                Mutex {}
            }

            pub fn lock(&self) -> Result<(), ()> {
                Ok(())
            }
        }

        // Implement a mock MutexGuard
        pub struct MutexGuard {}
    }

    pub struct TimeDelta {}
}

pub mod internal {
    pub struct Isolate {}

    impl Isolate {
        pub fn new() -> Self {
            Isolate {}
        }
    }

    pub struct Context {}

    pub type Tagged<T> = T;

    pub mod detail {
        use std::cell::RefCell;
        use std::rc::Rc;

        pub trait WaiterQueueNodeTrait {
            fn notify(&mut self);
            fn is_same_isolate_for_async_cleanup(&self, isolate: &Isolate) -> bool;
            fn cleanup_matching_async_waiters(&mut self, matcher: &dyn Fn(&dyn WaiterQueueNodeTrait) -> bool);
            fn set_ready_for_async_cleanup(&mut self);
        }

        pub struct WaiterQueueNode {
            requester_: *mut Isolate,
            next_: RefCell<Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>>,
            prev_: RefCell<Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>>,
        }

        impl WaiterQueueNode {
            pub fn new(requester: *mut Isolate) -> Self {
                WaiterQueueNode {
                    requester_: requester,
                    next_: RefCell::new(None),
                    prev_: RefCell::new(None),
                }
            }

            pub fn enqueue(head: &mut Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>, new_tail: Rc<RefCell<dyn WaiterQueueNodeTrait>>) {
                match head {
                    None => {
                        *head = Some(new_tail.clone());
                        new_tail.borrow_mut().set_next(head.clone());
                        new_tail.borrow_mut().set_prev(head.clone());
                    }
                    Some(head_rc) => {
                        let mut head_borrowed = head_rc.borrow_mut();
                        let old_tail = head_borrowed.prev().clone();

                        new_tail.borrow_mut().set_next(Some(head_rc.clone()));
                        new_tail.borrow_mut().set_prev(old_tail.clone());

                        if let Some(old_tail_rc) = old_tail {
                            let mut old_tail_borrowed = old_tail_rc.borrow_mut();
                            old_tail_borrowed.set_next(Some(new_tail.clone()));
                        }

                        head_borrowed.set_prev(Some(new_tail.clone()));
                    }
                }
            }

            pub fn dequeue_matching(
                head: &mut Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>,
                matcher: &dyn Fn(&dyn WaiterQueueNodeTrait) -> bool,
            ) -> Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>> {
                if head.is_none() {
                    return None;
                }

                let mut current = head.clone();
                let initial_head = head.clone();

                while let Some(current_rc) = current {
                    let current_borrowed = current_rc.borrow();
                    if matcher(&*current_borrowed) {
                        let next = current_borrowed.next().clone();
                        let prev = current_borrowed.prev().clone();

                        if let Some(prev_rc) = &prev {
                            prev_rc.borrow_mut().set_next(next.clone());
                        }

                        if let Some(next_rc) = &next {
                            next_rc.borrow_mut().set_prev(prev.clone());
                        }

                        // Update head if necessary
                        if let Some(initial_head_rc) = &initial_head {
                            if Rc::ptr_eq(&current_rc, initial_head_rc) {
                                if let Some(next_rc) = next.clone() {
                                    *head = Some(next_rc.clone());
                                    if Rc::ptr_eq(&next_rc, &current_rc){
                                        *head = None;
                                    }
                                } else {
                                    *head = None;
                                }
                            }
                        }
                        return Some(current_rc.clone());
                    }

                    current = current_borrowed.next().clone();

                    if let Some(initial_head_rc) = &initial_head {
                        if let Some(curr) = &current {
                            if Rc::ptr_eq(curr, initial_head_rc) {
                                break; // Prevent infinite loop in circular list.
                            }
                        }
                    }
                }

                None
            }

            pub fn dequeue_all_matching_for_async_cleanup(
                head: &mut Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>,
                matcher: &dyn Fn(&dyn WaiterQueueNodeTrait) -> bool,
            ) {
                while let Some(node) = Self::dequeue_matching(head, matcher) {
                    // Dropping the node will deallocate it.
                    drop(node);
                }
            }

            pub fn dequeue(head: &mut Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>>) -> Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>> {
                if head.is_none() {
                    return None;
                }

                let head_rc = head.take().unwrap();
                let mut head_borrowed = head_rc.borrow_mut();
                let next = head_borrowed.next().clone();
                let prev = head_borrowed.prev().clone();

                if let Some(prev_rc) = &prev {
                    prev_rc.borrow_mut().set_next(next.clone());
                }

                if let Some(next_rc) = &next {
                    next_rc.borrow_mut().set_prev(prev.clone());
                }

                if let Some(next_rc) = next {
                    *head = Some(next_rc.clone());
                } else {
                    *head = None;
                }

                Some(head_rc)
            }

            pub fn split(head: &mut Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>, count: u32) -> Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>> {
                if head.is_none() || count == 0 {
                    return None;
                }

                let mut current = head.clone();
                let mut i = 1;

                while let Some(current_rc) = current {
                    if i == count {
                        let next = current_rc.borrow().next().clone();

                        // Disconnect the list
                        current_rc.borrow_mut().set_next(None);

                        // Update head
                        *head = next.clone();

                        if let Some(head_rc) = head {
                            head_rc.borrow_mut().set_prev(None);
                        }

                        return next;
                    }

                    current = current_rc.borrow().next().clone();
                    i += 1;

                    if let Some(head_rc) = head.clone() {
                        if let Some(curr) = &current {
                            if Rc::ptr_eq(curr, &head_rc) {
                                break; // Prevent infinite loop in circular list.
                            }
                        }
                    }
                }

                None // If we reach the end of the list before reaching count
            }

            pub fn length_from_head(head: Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>) -> i32 {
                let mut length = 0;
                let mut current = head.clone();
                let initial_head = head.clone();

                while let Some(current_rc) = current {
                    length += 1;
                    current = current_rc.borrow().next().clone();

                    if let Some(initial_head_rc) = &initial_head {
                        if let Some(curr) = &current {
                            if Rc::ptr_eq(curr, initial_head_rc) {
                                break; // Prevent infinite loop in circular list.
                            }
                        }
                    }
                }

                length
            }

            pub fn notify_all_in_list(&self) -> u32 {
                // needs access to WaiterQueueNode fields next_ and prev_
                // TODO: Implement this correctly, this stub avoids compilation errors
                0
            }

            fn set_not_in_list_for_verification(&mut self) {
                //TODO: Implement this correctly
            }

            fn dequeue_unchecked(&mut self, _head: &mut Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>>) {
                //TODO: Implement this correctly
            }
        
            fn verify_not_in_list(&self) {
                //TODO: Implement this correctly
            }

            fn set_next(&self, next: Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>) {
                *self.next_.borrow_mut() = next;
            }

            fn next(&self) -> &RefCell<Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>> {
                &self.next_
            }

            fn set_prev(&self, prev: Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>) {
                *self.prev_.borrow_mut() = prev;
            }

            fn prev(&self) -> &RefCell<Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>> {
                &self.prev_
            }
        }
    }
}