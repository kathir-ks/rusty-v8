// Converted from V8 C++ source files:
// Header: prototype-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod prototype_inl {
    use crate::objects::prototype::*;
    use crate::objects::js_proxy::*;
    use crate::objects::map_inl::*;
    use crate::objects::fixed_array_inl::V8;
    use std::marker::PhantomData;
    use crate::objects::string::v8;

    pub struct PrototypeIterator<'a> {
        isolate_: *mut Isolate,
        handle_: Option<IndirectHandle<'a, JSReceiver>>,
        object_: Option<Tagged<JSReceiver>>, // Changed to Option
        where_to_end_: WhereToEnd,
        is_at_end_: bool,
        seen_proxies_: i32,
    }

    impl<'a> PrototypeIterator<'a> {
        pub fn new_from_handle(
            isolate: *mut Isolate,
            receiver: DirectHandle<'a, JSReceiver>,
            where_to_start: WhereToStart,
            where_to_end: WhereToEnd,
        ) -> Self {
            unsafe {
                let mut iterator = PrototypeIterator {
                    isolate_: isolate,
                    handle_: Some(indirect_handle(receiver, isolate)),
                    object_: None,
                    where_to_end_: where_to_end,
                    is_at_end_: false,
                    seen_proxies_: 0,
                };
                if where_to_start == WhereToStart::kStartAtPrototype {
                    iterator.advance();
                }
                iterator
            }
        }

        pub fn new_from_tagged(
            isolate: *mut Isolate,
            receiver: Tagged<JSReceiver>,
            where_to_start: WhereToStart,
            where_to_end: WhereToEnd,
        ) -> Self {
            let mut iterator = PrototypeIterator {
                isolate_: isolate,
                handle_: None,
                object_: Some(receiver),
                where_to_end_: where_to_end,
                is_at_end_: false,
                seen_proxies_: 0,
            };
            if where_to_start == WhereToStart::kStartAtPrototype {
                iterator.advance();
            }
            iterator
        }

        pub fn new_from_map(
            isolate: *mut Isolate,
            receiver_map: Tagged<Map>,
            where_to_end: WhereToEnd,
        ) -> Self {
            unsafe {
                let prototype = receiver_map.GetPrototypeChainRootMap(isolate).prototype();
                let is_null = prototype.is_null();
                let mut iterator = PrototypeIterator {
                    isolate_: isolate,
                    handle_: None,
                    object_: Some(prototype.clone()),//Changed from prototype()
                    where_to_end_: where_to_end,
                    is_at_end_: is_null,
                    seen_proxies_: 0,
                };
                if !iterator.is_at_end_ && iterator.where_to_end_ == WhereToEnd::END_AT_NON_HIDDEN {
                    if let Some(object_) = iterator.object_.clone(){
                        let map = object_.map();
                        iterator.is_at_end_ = map.is_js_global_proxy_map();
                    }
                }
                iterator
            }
        }

        pub fn new_from_map_handle(
            isolate: *mut Isolate,
            receiver_map: DirectHandle<'a, Map>,
            where_to_end: WhereToEnd,
        ) -> Self {
             unsafe {
                let prototype = receiver_map.GetPrototypeChainRootMap(isolate).prototype();
                 let is_null = prototype.is_null();
                 let mut iterator = PrototypeIterator {
                     isolate_: isolate,
                     handle_: Some(indirect_handle(prototype, isolate)),
                     object_: None,
                     where_to_end_: where_to_end,
                     is_at_end_: is_null,
                     seen_proxies_: 0,
                 };
                 if !iterator.is_at_end_ && iterator.where_to_end_ == WhereToEnd::END_AT_NON_HIDDEN {
                     if let Some(handle_) = iterator.handle_.clone(){
                         if let Some(object) = handle_.object_ {
                            let map = object.map();
                            iterator.is_at_end_ = map.is_js_global_proxy_map();
                         } else {
                             iterator.is_at_end_ = true;
                         }
                     }
                 }
                 iterator
             }
        }

        pub fn has_access(&self) -> bool {
            // We can only perform access check in the handlified version of the
            // PrototypeIterator.
            if let Some(handle_) = &self.handle_ {
                if let Some(handle_obj) = handle_.object_ {
                    if handle_obj.is_access_check_needed() {
                        unsafe {
                            return (*self.isolate_).MayAccess((*self.isolate_).native_context(), handle_obj.into());
                        }
                    }
                }
            }
            true
        }

        pub fn advance(&mut self) {
            if let Some(object_) = self.object_ {
                if object_.is_js_proxy() {
                    self.is_at_end_ = true;
                    self.object_ = None;
                    return;
                }
            }
            if let Some(handle_) = &self.handle_ {
                if let Some(handle_obj) = handle_.object_{
                    if handle_obj.is_js_proxy() {
                        self.is_at_end_ = true;
                        self.handle_ = None;
                        return;
                    }
                }

            }
            self.advance_ignoring_proxies();
        }

        pub fn advance_ignoring_proxies(&mut self) {
            unsafe {
                let object = if let Some(handle_) = &self.handle_ {
                    if let Some(handle_obj) = handle_.object_{
                         handle_obj
                    } else{
                        if let Some(object_) = self.object_{
                            object_
                         } else{
                             return;
                         }
                    }

                } else{
                    if let Some(object_) = self.object_{
                       object_
                    }else {
                        return;
                    }

                };
                 let map = object.map();
                let prototype = map.prototype();
                self.is_at_end_ = prototype.is_null()
                    || (self.where_to_end_ == WhereToEnd::END_AT_NON_HIDDEN && !map.is_js_global_proxy_map());

                if self.handle_.is_none() {
                    self.object_ = Some(prototype);
                } else {
                    self.handle_ = Some(indirect_handle(prototype, self.isolate_));
                }
            }
        }

        pub fn advance_following_proxies(&mut self) -> bool {
            if !self.has_access() {
                self.handle_ = None;
                self.is_at_end_ = true;
                return true;
            }
            self.advance_following_proxies_ignoring_access_checks()
        }

        pub fn advance_following_proxies_ignoring_access_checks(&mut self) -> bool {
             unsafe {
                 if self.handle_.is_none()  {
                     self.advance_ignoring_proxies();
                     return true;
                 } else{
                      if let Some(handle_) = self.handle_.clone() {
                         if let Some(handle_obj) = handle_.object_ {
                             if !handle_obj.is_js_proxy() {
                                 self.advance_ignoring_proxies();
                                 return true;
                             }
                         }
                     }
                 }
                 // Due to possible __proto__ recursion limit the number of Proxies
                 // we visit to an arbitrarily chosen large number.
                 self.seen_proxies_ += 1;
                 if self.seen_proxies_ > JSProxy::kMaxIterationLimit {
                     (*self.isolate_).StackOverflow();
                     return false;
                 }
                if let Some(handle_) = &self.handle_{
                   if let Some(proxy) = handle_.object_{
                         let proto = proxy.get_prototype();

                         let mut proto_direct_handle:Option<Tagged<JSPrototype>> = Some(proto.clone());//TODO check if this is right

                        if let Some(proto_direct_handle_val) = proto_direct_handle {
                             self.handle_ = Some(indirect_handle(proto_direct_handle_val, self.isolate_));
                         } else {
                             return false;
                         }
                   }
                }

                 self.is_at_end_ = self.where_to_end_ == WhereToEnd::END_AT_NON_HIDDEN || self.handle_.is_none();
                 return true;
             }
        }
    }
}
