// Converted from V8 C++ source files:
// Header: waiter-queue-node.h
// Implementation: waiter-queue-node.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/objects/waiter-queue-node.h

pub mod waiter_queue_node {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::{Mutex, MutexGuard, PoisonError};

    pub struct V8_NODISCARD {}

    pub trait WaiterQueueNodeTrait {
        fn notify(&mut self);
        fn is_same_isolate_for_async_cleanup(&self, isolate: *mut Isolate) -> bool;
        fn cleanup_matching_async_waiters(&mut self, matcher: &dyn Fn(&mut dyn WaiterQueueNodeTrait) -> bool);
        fn set_ready_for_async_cleanup(&mut self);
    }

    pub struct WaiterQueueNode {
        pub requester_: *mut Isolate,
        pub next_: Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>,
        pub prev_: Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>,
    }

    impl WaiterQueueNode {
        pub fn new(requester: *mut Isolate) -> Self {
            WaiterQueueNode {
                requester_: requester,
                next_: None,
                prev_: None,
            }
        }

        // Enqueues {new_tail}, mutating {head} to be the new head.
        pub fn enqueue(head: &mut Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>, new_tail: Rc<RefCell<dyn WaiterQueueNodeTrait>>) {
            if let Some(head_rc) = head {
                let mut head_borrowed = head_rc.borrow_mut();
                if head_borrowed.as_any().downcast_ref::<WaiterQueueNode>().unwrap().next_.is_none() {
                    // The queue contains exactly 1 node.
                    new_tail.borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().next_ = Some(new_tail.clone());
                    new_tail.borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().prev_ = Some(new_tail.clone());
                    head_borrowed.as_any().downcast_mut::<WaiterQueueNode>().unwrap().next_ = Some(new_tail.clone());
                    head_borrowed.as_any().downcast_mut::<WaiterQueueNode>().unwrap().prev_ = Some(new_tail.clone());

                    
                } else {
                    // The queue contains >1 nodes.
                    let current_tail = head_borrowed.as_any().downcast_mut::<WaiterQueueNode>().unwrap().prev_.clone().unwrap();
                    let mut current_tail_borrowed = current_tail.borrow_mut();
                
                    current_tail_borrowed.as_any().downcast_mut::<WaiterQueueNode>().unwrap().next_ = Some(new_tail.clone());
                    head_borrowed.as_any().downcast_mut::<WaiterQueueNode>().unwrap().prev_ = Some(new_tail.clone());
                    new_tail.borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().next_ = Some(head_rc.clone());
                    new_tail.borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().prev_ = Some(current_tail.clone());
                }
                
            } else {
                new_tail.borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().next_ = Some(new_tail.clone());
                new_tail.borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().prev_ = Some(new_tail.clone());
                *head = Some(new_tail.clone());
            }
        }

        // Dequeues the first waiter for which {matcher} returns true and returns it;
        // mutating {head} to be the new head.
        //
        // The queue lock must be held in the synchronization primitive that owns
        // this waiter queue when calling this method.
        pub fn dequeue_matching(head: &mut Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>, matcher: &dyn Fn(&mut dyn WaiterQueueNodeTrait) -> bool) -> Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>> {
            if head.is_none() {
                return None;
            }
            let original_head = head.clone().unwrap();
            let mut cur = head.clone().unwrap();
            loop {
                if matcher(&mut *cur.borrow_mut()) {
                    let next_node = cur.borrow().as_any().downcast_ref::<WaiterQueueNode>().unwrap().next_.clone();
                    let prev_node = cur.borrow().as_any().downcast_ref::<WaiterQueueNode>().unwrap().prev_.clone();
                   
                    if next_node.clone().unwrap().as_ptr() == cur.as_ptr(){
                         // The queue contains exactly 1 node.
                        *head = None;
                    } else {
                         // The queue contains >1 nodes.
                        if Rc::ptr_eq(&cur, &original_head) {
                            // The matched node is the head, so next is the new head.
                            
                            *head = next_node.clone();
                            let tail = original_head.borrow().as_any().downcast_ref::<WaiterQueueNode>().unwrap().prev_.clone().unwrap();
                           
                            tail.borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().next_ = next_node.clone();
                            next_node.clone().unwrap().borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().prev_ = Some(tail.clone());
                            
                        } else {
                            // The matched node is in the middle of the queue, so the head does
                            // not need to be updated.
                           
                            prev_node.clone().unwrap().borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().next_ = next_node.clone();
                            next_node.clone().unwrap().borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().prev_ = prev_node.clone();
                        }
                    }
                    
                    cur.borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().set_not_in_list_for_verification();
                    return Some(cur);
                }
                
                if Rc::ptr_eq(&cur, &original_head)
                {
                    break;
                }
                let next_node = cur.borrow().as_any().downcast_ref::<WaiterQueueNode>().unwrap().next_.clone();
                cur = next_node.unwrap();
               
            }
            None
        }

        pub fn dequeue_all_matching_for_async_cleanup(head: &mut Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>, matcher: &dyn Fn(&mut dyn WaiterQueueNodeTrait) -> bool) {
            if head.is_none() {
                return;
            }
            
            let original_tail = head.clone().unwrap().borrow().as_any().downcast_ref::<WaiterQueueNode>().unwrap().prev_.clone().unwrap();
            let mut cur = head.clone().unwrap();
            loop {
                let next_node = cur.borrow().as_any().downcast_ref::<WaiterQueueNode>().unwrap().next_.clone();
                if matcher(&mut *cur.borrow_mut()) {
                   
                    let next_node2 = cur.borrow().as_any().downcast_ref::<WaiterQueueNode>().unwrap().next_.clone();
                    let prev_node = cur.borrow().as_any().downcast_ref::<WaiterQueueNode>().unwrap().prev_.clone();
                   
                    if next_node2.clone().unwrap().as_ptr() == cur.as_ptr(){
                         // The queue contains exactly 1 node.
                        *head = None;
                    } else {
                         // The queue contains >1 nodes.
                        if Rc::ptr_eq(&cur, &head.clone().unwrap()) {
                            // The matched node is the head, so next is the new head.
                            
                            *head = next_node2.clone();
                            let tail = head.clone().unwrap().borrow().as_any().downcast_ref::<WaiterQueueNode>().unwrap().prev_.clone().unwrap();
                           
                            tail.borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().next_ = next_node2.clone();
                            next_node2.clone().unwrap().borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().prev_ = Some(tail.clone());
                            
                        } else {
                            // The matched node is in the middle of the queue, so the head does
                            // not need to be updated.
                           
                            prev_node.clone().unwrap().borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().next_ = next_node2.clone();
                            next_node2.clone().unwrap().borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().prev_ = prev_node.clone();
                        }
                    }
                    
                    cur.borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().set_not_in_list_for_verification();
                   
                    cur.borrow_mut().set_ready_for_async_cleanup();
                }
                if Rc::ptr_eq(&cur, &original_tail)
                {
                    break;
                }
                cur = next_node.unwrap();
            }
        }

        // static
        pub fn dequeue(head: &mut Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>) -> Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>> {
            WaiterQueueNode::dequeue_matching(head, &|_node| true)
        }

        // static
        pub fn split(head: &mut Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>>, count: u32) -> Option<Rc<RefCell<dyn WaiterQueueNodeTrait>>> {
            if count == 0 || head.is_none(){
                return None;
            }
            
            let front_head = head.clone().unwrap();
            let mut back_head = front_head.clone();
            let mut actual_count = 0;
            
            while actual_count < count {
                let next_node = back_head.borrow().as_any().downcast_ref::<WaiterQueueNode>().unwrap().next_.clone();
                
                back_head = next_node.clone().unwrap();
                // The queue is shorter than the requested count, return the whole queue.
                if Rc::ptr_eq(&back_head, &front_head) {
                    *head = None;
                    return Some(front_head);
                }
                actual_count += 1;
            }
            
            let front_tail = back_head.borrow().as_any().downcast_ref::<WaiterQueueNode>().unwrap().prev_.clone().unwrap();
            let back_tail = front_head.borrow().as_any().downcast_ref::<WaiterQueueNode>().unwrap().prev_.clone().unwrap();

            // Fix up the back list (i.e. remainder of the list).
            back_head.borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().prev_ = Some(back_tail.clone());
            back_tail.borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().next_ = Some(back_head.clone());
            *head = Some(back_head.clone());

            // Fix up and return the front list (i.e. the dequeued list).
            front_head.borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().prev_ = Some(front_tail.clone());
            front_tail.borrow_mut().as_any().downcast_mut::<WaiterQueueNode>().unwrap().next_ = Some(front_head.clone());
            return Some(front_head);
        }

        // static
        pub fn length_from_head(head: Rc<RefCell<dyn WaiterQueueNodeTrait>>) -> i32 {
            let mut cur = head.clone();
            let mut len = 0;
            loop {
                len += 1;
                let next_node = cur.borrow().as_any().downcast_ref::<WaiterQueueNode>().unwrap().next_.clone();
                cur = next_node.unwrap();
                if Rc::ptr_eq(&cur, &head) {
                    break;
                }
            }
            len
        }

        pub fn notify_all_in_list(&mut self) -> u32 {
            let head = Rc::new(RefCell::new(self));
            let mut cur = head.clone();
            let mut count = 0;
            loop {
                let next_node = cur.borrow().as_any().downcast_ref::<WaiterQueueNode>().unwrap().next_.clone();
                cur.borrow_mut().notify();
                cur = next_node.unwrap();
                count += 1;
                if Rc::ptr_eq(&cur, &head) {
                    break;
                }
            }
            count
        }

        fn verify_not_in_list(&self) {
            assert!(self.next_.is_none());
            assert!(self.prev_.is_none());
        }

        fn set_not_in_list_for_verification(&mut self) {
            self.next_ = None;
            self.prev_ = None;
        }
    }

    pub trait AsAny {
        fn as_any(&self) -> &dyn std::any::Any;
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    }

    impl <T: WaiterQueueNodeTrait + 'static> AsAny for T {
        fn as_any(&self) -> &dyn std::any::Any {
            self as &dyn std::any::Any
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self as &mut dyn std::any::Any
        }
    }
}

// src/objects/waiter-queue-node.cc

pub mod detail {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::objects::waiter_queue_node::WaiterQueueNode;
    use crate::objects::waiter_queue_node::WaiterQueueNodeTrait;
    use crate::objects::waiter_queue_node::AsAny;
    use crate::Isolate;

    impl WaiterQueueNodeTrait for WaiterQueueNode{
        fn notify(&mut self) {
            // Implement notify logic here
            println!("Notify called");
        }

        fn is_same_isolate_for_async_cleanup(&self, isolate: *mut Isolate) -> bool {
            self.requester_ == isolate
        }

        fn cleanup_matching_async_waiters(&mut self, matcher: &dyn Fn(&mut dyn WaiterQueueNodeTrait) -> bool) {
            // Implement cleanup logic here
            println!("CleanupMatchingAsyncWaiters called");
        }

        fn set_ready_for_async_cleanup(&mut self) {
            // Implement set_ready_for_async_cleanup logic here
            println!("SetReadyForAsyncCleanup called");
        }
    }

    impl Drop for WaiterQueueNode {
        fn drop(&mut self) {
            // Since waiter queue nodes are allocated on the stack, they must be removed
            // from the intrusive linked list once they go out of scope, otherwise there
            // will be dangling pointers.
           
            self.verify_not_in_list();
        }
    }

}
