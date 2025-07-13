// Converted from V8 C++ source files:
// Header: keys.h
// Implementation: keys.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod keys {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::v8::internal::OrderedHashSet;
    use crate::v8::internal::JSProxy;
    use crate::v8::internal::FixedArray;
    use crate::v8::internal::Isolate;
    use crate::v8::internal::JSReceiver;
    use crate::v8::internal::PropertyFilter;
    use crate::v8::internal::MaybeHandle;
    use crate::v8::internal::DirectHandle;
    use crate::v8::internal::JSObject;
    use crate::v8::internal::Tagged;
    use crate::v8::internal::Object;
    use crate::v8::internal::Symbol;
    use crate::v8::internal::String;
    use crate::v8::internal::PropertyDescriptor;
    use crate::v8::internal::Maybe;
    use crate::v8::internal::ExceptionStatus;
    use crate::v8::internal::HeapObjectReference;
    use crate::v8::internal::AllowGarbageCollection;
    use crate::v8::internal::InterceptorInfo;
    use crate::v8::internal::JSObjectOrUndefined;
    use crate::v8::internal::Map;
    use crate::v8::internal::PropertyDetails;
    use crate::v8::internal::AccessCheckInfo;
    use crate::v8::internal::FieldIndex;
    use crate::v8::internal::PropertyLocation;
    use crate::v8::internal::AllocationType;
    use crate::v8::internal::SwissNameDictionary;
    use crate::v8::internal::JSGlobalObject;
    use crate::v8::internal::JSModuleNamespace;
    use crate::v8::internal::Name;
    use crate::v8::internal::Zone;
    use crate::v8::internal::base;
    use crate::v8::internal::Execution;
    use crate::v8::internal::ElementTypes;
    use crate::v8::internal::PrototypeInfo;
    use crate::v8::internal::ReadOnlyRoots;
    use crate::v8::internal::IsAccessCheckNeeded;
    use crate::v8::internal::PropertyKind;
    use crate::v8::internal::DisallowGarbageCollection;
    use crate::v8::internal::JSRuntimeCall;
    use crate::v8::internal::kAcquireLoad;
    use crate::v8::internal::AtomicSlot;
    use crate::v8::internal::base::TemplateHashMapImpl;
    use crate::v8::internal::ZoneAllocationPolicy;
    use crate::v8::internal::EnumIndexComparator;
    use crate::v8::internal::v8_flags;
    use std::cmp::Ordering;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering::SeqCst;
    use crate::v8::internal::Smi;
    use crate::v8::internal::kInvalidEnumCacheSentinel;
    use crate::v8::internal::kDontThrow;
    use crate::v8::internal::PrototypeIterator;
    use crate::v8::internal::kStartAtReceiver;
    use crate::v8::internal::kThrowOnError;
    use crate::v8::internal::MessageTemplate;
    use crate::v8::internal::ObjectHashSet;
    use crate::v8::internal::IsCustomElementsReceiverMap;
    use crate::v8::internal::ONLY_ENUMERABLE;
    use crate::v8::internal::HeapObject;
    use crate::v8::internal::JSArray;
    use crate::v8::internal::ElementsAccessor;
    use crate::v8::internal::direct_handle;
    use crate::v8::internal::Cast;
    use crate::v8::internal::isJSObjectMap;
    use crate::v8::internal::IsJSObject;
    use crate::v8::internal::IsName;
    use crate::v8::internal::IsString;
    use crate::v8::internal::IsNumber;
    use crate::v8::internal::IsJSGlobalObject;
    use crate::v8::internal::IsUndefined;
    use crate::v8::internal::IsSymbol;
    use crate::v8::internal::IsWasmObject;
    use crate::v8::internal::kIncludePrototypes;
    use crate::v8::internal::kOwnOnly;
    use crate::v8::internal::ALL_PROPERTIES;
    use crate::v8::internal::PRIVATE_NAMES_ONLY;
    use crate::v8::internal::SKIP_SYMBOLS;
    use crate::v8::internal::SKIP_STRINGS;
    use crate::v8::internal::ENUMERABLE_STRINGS;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AddKeyConversion {
        DO_NOT_CONVERT,
        CONVERT_TO_ARRAY_INDEX,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum GetKeysConversion {
        kKeepNumbers = 0, 
        kConvertToString = 1,
        kNoNumbers = 2
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum KeyCollectionMode {
        kOwnOnly = 0,
        kIncludePrototypes = 1,
    }

    pub struct KeyAccumulator {
        isolate_: *mut Isolate,
        keys_: Handle<OrderedHashSet>,
        first_prototype_map_: DirectHandle<Map>,
        receiver_: DirectHandle<JSReceiver>,
        last_non_empty_prototype_: DirectHandle<JSReceiver>,
        shadowing_keys_: Handle<ObjectHashSet>,
        mode_: KeyCollectionMode,
        filter_: PropertyFilter,
        is_for_in_: bool,
        skip_indices_: bool,
        skip_shadow_check_: bool,
        may_have_elements_: bool,
        try_prototype_info_cache_: bool,
    }

    impl KeyAccumulator {
        pub fn new(isolate: *mut Isolate, mode: KeyCollectionMode, filter: PropertyFilter) -> Self {
            KeyAccumulator {
                isolate_: isolate,
                keys_: Handle::empty(),
                first_prototype_map_: DirectHandle::empty(),
                receiver_: DirectHandle::empty(),
                last_non_empty_prototype_: DirectHandle::empty(),
                shadowing_keys_: Handle::empty(),
                mode_: mode,
                filter_: filter,
                is_for_in_: false,
                skip_indices_: false,
                skip_shadow_check_: true,
                may_have_elements_: true,
                try_prototype_info_cache_: false,
            }
        }

        pub fn get_keys(
            isolate: *mut Isolate,
            object: &DirectHandle<JSReceiver>,
            mode: KeyCollectionMode,
            filter: PropertyFilter,
            keys_conversion: GetKeysConversion,
            is_for_in: bool,
            skip_indices: bool,
        ) -> MaybeHandle<FixedArray> {
            let accumulator = FastKeyAccumulator::new(isolate, object.clone(), mode, filter, is_for_in, skip_indices);
            accumulator.get_keys(keys_conversion)
        }

        pub fn get_keys_(&self, convert: GetKeysConversion) -> Handle<FixedArray> {
            if self.keys_.is_empty() {
                unsafe { (*self.isolate_).factory().empty_fixed_array() }
            } else {
                use ContainsOnlyValidKeys;
                let result = OrderedHashSet::convert_to_keys_array(unsafe { self.isolate_ }, self.keys(), convert);
                debug_assert!(ContainsOnlyValidKeys(&result));

                if self.try_prototype_info_cache_ && !self.first_prototype_map_.is_empty() {
                  let _ =  unsafe {(*self.isolate_).factory()};
                   // Cast::<PrototypeInfo>(&*(*self.first_prototype_map_).prototype_info()).set_prototype_chain_enum_cache(*result);
                   // Map::GetOrCreatePrototypeChainValidityCell(direct_handle(self.receiver_.map()), unsafe { self.isolate_ });
                   //debug_assert!(self.first_prototype_map_.IsPrototypeValidityCellValid());
                }
                result
            }
        }

        fn keys(&self) -> Handle<OrderedHashSet> {
          unsafe { Handle::cast(self.keys_.clone()) }
        }

        pub fn add_key(&mut self, key: Tagged<Object>, convert: AddKeyConversion) -> ExceptionStatus {
            self.add_key_(&direct_handle(key, self.isolate_), convert)
        }

        pub fn add_key_(&mut self, key: &DirectHandle<Object>, convert: AddKeyConversion) -> ExceptionStatus {
            if self.filter_ == PropertyFilter::PRIVATE_NAMES_ONLY {
                if !IsSymbol(*key) {
                    return ExceptionStatus::kSuccess;
                }
                if !Cast::<Symbol>(*key).is_private_name() {
                    return ExceptionStatus::kSuccess;
                }
            } else if IsSymbol(*key) {
                if self.filter_.contains(PropertyFilter::SKIP_SYMBOLS) {
                    return ExceptionStatus::kSuccess;
                }
                 if Cast::<Symbol>(*key).is_private() {
                    return ExceptionStatus::kSuccess;
                 }
            } else if self.filter_.contains(PropertyFilter::SKIP_STRINGS) {
                return ExceptionStatus::kSuccess;
            }

            if self.is_shadowed(key) {
                return ExceptionStatus::kSuccess;
            }
            if self.keys_.is_empty() {
                 unsafe {
                    let keys = OrderedHashSet::allocate(self.isolate_, 16);
                    if keys.is_err() {
                         return ExceptionStatus::kException;
                    }
                    self.keys_ = keys.unwrap();
                 }
            }
            let mut index: u32;
            let mut key_handle: DirectHandle<Object> = key.clone();
            if convert == AddKeyConversion::CONVERT_TO_ARRAY_INDEX && IsString(*key) {
              let index_option = Cast::<String>(*key).as_array_index();
                if index_option.is_some() {
                    index = index_option.unwrap();
                    unsafe {
                         key_handle = direct_handle((*self.isolate_).factory().new_number_from_uint(index), self.isolate_);
                    }
                }
            }

            unsafe {
              let new_set_candidate = OrderedHashSet::add(self.isolate_, self.keys().clone(), &key_handle);
              if new_set_candidate.is_none() {
                  let has_exception = Isolate::has_exception(self.isolate_);
                   debug_assert!(has_exception);
                   return ExceptionStatus::kException;
              }
               let new_set = new_set_candidate.unwrap();
                if *new_set != *self.keys_ {
                     (*self.keys_.location()).set(OrderedHashSet::next_table_index() as i32, 0);
                     self.keys_ = new_set.clone();
                }
            }
            ExceptionStatus::kSuccess
        }

        pub fn add_keys(&mut self, array: &DirectHandle<FixedArray>, convert: AddKeyConversion) -> ExceptionStatus {
            let add_length = array.length();
            for i in 0..add_length {
                let current = direct_handle(array.get(i), self.isolate_);
                let status = self.add_key_(&current, convert);
                if !status {
                    return status;
                }
            }
            ExceptionStatus::kSuccess
        }

        pub fn add_keys_from_js_object(&mut self, array_like: &DirectHandle<JSObject>, convert: AddKeyConversion) -> ExceptionStatus {
            debug_assert!(IsJSArray(*array_like) || array_like.has_sloppy_arguments_elements());
            let accessor = array_like.get_elements_accessor();
            accessor.add_elements_to_key_accumulator(array_like, self, convert)
        }
        
        fn is_shadowed(&self, key: &DirectHandle<Object>) -> bool {
          if !self.has_shadowing_keys() || self.skip_shadow_check_ {
              return false;
          }
           unsafe { self.shadowing_keys_.location().is_empty() }
        }
        
        fn has_shadowing_keys(&self) -> bool {
            !self.shadowing_keys_.is_empty()
        }
        
        pub fn add_shadowing_key(&mut self, key: Tagged<Object>, _allow_gc: &mut AllowGarbageCollection) {
            if self.mode_ == KeyCollectionMode::kOwnOnly {
              return;
            }
            self.add_shadowing_key_(&direct_handle(key, self.isolate_));
        }
        
        pub fn add_shadowing_key_(&mut self, key: &DirectHandle<Object>) {
            if self.mode_ == KeyCollectionMode::kOwnOnly {
              return;
            }
            if self.shadowing_keys_.is_empty() {
               unsafe { self.shadowing_keys_.location().set_null() };
            }
            unsafe { self.shadowing_keys_.location().is_empty() };
        }
        
        pub fn collect_keys(&mut self, receiver: &DirectHandle<JSReceiver>, object: &DirectHandle<JSReceiver>) -> Maybe<bool> {
            if self.mode_ == KeyCollectionMode::kOwnOnly && IsJSProxy(*object) {
                 let maybe_bool = self.collect_own_js_proxy_keys(receiver, &Cast::<JSProxy>(object.clone()));
                if maybe_bool.is_nothing() {
                     return Maybe::nothing();
                }
                return Maybe::just(true);
            }

            let end = if self.mode_ == KeyCollectionMode::kOwnOnly {
                PrototypeIterator::WhereToEnd::END_AT_NON_HIDDEN
            } else {
                PrototypeIterator::WhereToEnd::END_AT_NULL
            };

            for iter in PrototypeIterator::new(self.isolate_, object.clone(), kStartAtReceiver, end) {
                if self.has_shadowing_keys() {
                    self.skip_shadow_check_ = false;
                }
                let current = iter.get_current::<JSReceiver>();

                 let result = if IsJSProxy(*current) {
                    self.collect_own_js_proxy_keys(receiver, &Cast::<JSProxy>(current.clone()))
                  } else if IsWasmObject(*current) {
                      if self.mode_ == KeyCollectionMode::kIncludePrototypes {
                           let _throw = unsafe {(*self.isolate_).factory().new_type_error(MessageTemplate::kWasmObjectsAreOpaque)};
                        return Maybe::nothing();
                      } else {
                         debug_assert_eq!(self.mode_, KeyCollectionMode::kOwnOnly);
                         Maybe::just(false)
                      }
                  } else {
                    debug_assert!(IsJSObject(*current));
                    self.collect_own_keys(receiver, &Cast::<JSObject>(current.clone()))
                };

                if result.is_nothing() {
                    return Maybe::nothing();
                }
                if !result.from_just() {
                    break;
                }

                if !iter.advance_following_proxies_ignoring_access_checks() {
                  return Maybe::nothing();
                }

                if !self.last_non_empty_prototype_.is_empty() && *self.last_non_empty_prototype_ == *current {
                     break;
                }
            }
            Maybe::just(true)
        }

        fn collect_own_keys(&mut self, receiver: &DirectHandle<JSReceiver>, object: &DirectHandle<JSObject>) -> Maybe<bool> {
          
            if IsAccessCheckNeeded(*object) && unsafe { !Isolate::may_access(self.isolate_, (*self.isolate_).native_context(), object) } {
                if self.mode_ == KeyCollectionMode::kIncludePrototypes {
                    return Maybe::just(false);
                }
                debug_assert_eq!(self.mode_, KeyCollectionMode::kOwnOnly);

                let mut access_check_info: DirectHandle<AccessCheckInfo> = DirectHandle::empty();
                unsafe {
                    let maybe_info = AccessCheckInfo::get(self.isolate_, *object);
                    if !maybe_info.is_null() {
                        access_check_info = direct_handle(maybe_info, self.isolate_);
                    }
                }
                
                if !access_check_info.is_empty() && !unsafe { (*access_check_info.location()).is_null() } {
                  let maybe = self.collect_access_check_interceptor_keys(&access_check_info, receiver, object);
                   if maybe.is_nothing() {
                        return Maybe::nothing();
                   }
                }
                return Maybe::just(false);
            }
            if self.filter_ & PropertyFilter::PRIVATE_NAMES_ONLY {
              if ExceptionStatus::kSuccess != self.collect_private_names(receiver, object) {
                   return Maybe::nothing();
              }
                return Maybe::just(true);
            }

             if self.may_have_elements_ {
                let maybe = self.collect_own_element_indices(receiver, object);
                  if maybe.is_nothing() {
                        return Maybe::nothing();
                   }
            }
            let maybe = self.collect_own_property_names(receiver, object);
             if maybe.is_nothing() {
                  return Maybe::nothing();
            }
            Maybe::just(true)
        }
        
        fn collect_own_element_indices(&mut self, receiver: &DirectHandle<JSReceiver>, object: &DirectHandle<JSObject>) -> Maybe<bool> {
           if self.filter_ & PropertyFilter::SKIP_STRINGS != PropertyFilter::empty() || self.skip_indices_ {
              return Maybe::just(true);
           }
           
           let accessor = object.get_elements_accessor();
           let maybe_bool = accessor.collect_element_indices(object, self);
           if maybe_bool.is_err() {
                return Maybe::nothing();
           }
           self.collect_interceptor_keys(receiver, object, IndexedOrNamed::kIndexed)
        }
        
         fn collect_private_names(&mut self, receiver: &DirectHandle<JSReceiver>, object: &DirectHandle<JSObject>) -> ExceptionStatus {
             debug_assert_eq!(self.mode_, KeyCollectionMode::kOwnOnly);

            if object.has_fast_properties() {
                let limit = object.map().number_of_own_descriptors();
                let descs = direct_handle(object.map().instance_descriptors(self.isolate_), self.isolate_);
                let _void = collect_own_property_names_internal::<false>(object, self, &descs, 0, limit);
            } else if IsJSGlobalObject(*object) {
               unsafe { let object = Cast::<JSGlobalObject>(object.clone()); }
                let global_dictionary = unsafe { Cast::<JSGlobalObject>(object.clone()).global_dictionary(kAcquireLoad) };
                let dic_handle = direct_handle(global_dictionary, self.isolate_);
                if ExceptionStatus::kSuccess != collect_keys_from_dictionary(&dic_handle, self) {
                    return ExceptionStatus::kException;
                }
            } else if V8_ENABLE_SWISS_NAME_DICTIONARY_BOOL {
                let dic_handle = direct_handle(object.property_dictionary_swiss(), self.isolate_);
                if ExceptionStatus::kSuccess != collect_keys_from_dictionary(&dic_handle, self) {
                    return ExceptionStatus::kException;
                }
            } else {
                let dic_handle = direct_handle(object.property_dictionary(), self.isolate_);
                if ExceptionStatus::kSuccess != collect_keys_from_dictionary(&dic_handle, self) {
                    return ExceptionStatus::kException;
                }
            }

            ExceptionStatus::kSuccess
        }
        
        fn collect_interceptor_keys(&mut self, receiver: &DirectHandle<JSReceiver>, object: &DirectHandle<JSObject>, type_: IndexedOrNamed) -> Maybe<bool> {
           if type_ == IndexedOrNamed::kIndexed {
             if !object.has_indexed_interceptor() {
                 return Maybe::just(true);
             }
           } else {
                if !object.has_named_interceptor() {
                 return Maybe::just(true);
                }
           }
            let interceptor = direct_handle(if type_ == IndexedOrNamed::kIndexed { object.get_indexed_interceptor() } else { object.get_named_interceptor() }, self.isolate_);
            self.collect_interceptor_keys_internal(receiver, object, &interceptor, type_)
        }
        
         fn collect_interceptor_keys_internal(&mut self, receiver: &DirectHandle<JSReceiver>, object: &DirectHandle<JSObject>, interceptor: &DirectHandle<InterceptorInfo>, type_: IndexedOrNamed) -> Maybe<bool> {
                let enum_args = PropertyCallbackArguments::new(self.isolate_, interceptor.data(), *receiver, *object, Maybe::just(kDontThrow));

                if IsUndefined(interceptor.enumerator(), self.isolate_) {
                    return Maybe::just(true);
                }

                let maybe_result = if type_ == IndexedOrNamed::kIndexed {
                    enum_args.call_indexed_enumerator(interceptor)
                } else {
                    debug_assert_eq!(type_, IndexedOrNamed::kNamed);
                    enum_args.call_named_enumerator(interceptor)
                };

                if unsafe { Isolate::has_pending_exception(self.isolate_) } {
                    return Maybe::nothing();
                }

                if IsUndefined(*maybe_result) {
                    return Maybe::just(true);
                }

                debug_assert!(IsJSObject(*maybe_result));
                let result = Cast::<JSObject>(*maybe_result);
                
                let _value = enum_args.accept_side_effects();

                if (self.filter_ & PropertyFilter::ONLY_ENUMERABLE) != PropertyFilter::empty() && !IsUndefined(interceptor.query(), self.isolate_) {
                   let maybe_bool = self.filter_for_enumerable_properties(receiver, object, interceptor, &result, type_);
                   if maybe_bool.is_err() {
                         return Maybe::nothing();
                   }
                } else {
                   let convert = if type_ == IndexedOrNamed::kIndexed { AddKeyConversion::CONVERT_TO_ARRAY_INDEX } else { AddKeyConversion::DO_NOT_CONVERT };
                   let status = self.add_keys(&direct_handle(result, self.isolate_), convert);
                   if ExceptionStatus::kSuccess != status {
                        return Maybe::nothing();
                   }
                }
                Maybe::just(true)
            }
        
        
        fn filter_for_enumerable_properties(&mut self, receiver: &DirectHandle<JSReceiver>, object: &DirectHandle<JSObject>, interceptor: &DirectHandle<InterceptorInfo>, result: &DirectHandle<JSObject>, type_: IndexedOrNamed) -> Result<(), ()> {
            debug_assert!(IsJSArray(*result) || result.has_sloppy_arguments_elements());
            let accessor = result.get_elements_accessor();

            let length = accessor.get_capacity(*result, result.elements());
            for entry in 0..length as i32 {
                if !accessor.has_entry(*result, InternalIndex::new(entry)) {
                    continue;
                }
                let args = PropertyCallbackArguments::new(self.isolate_, interceptor.data(), *receiver, *object, Maybe::just(kDontThrow));
                let element = accessor.get(self.isolate_, result, InternalIndex::new(entry));
                let attributes: DirectHandle<Object>;
                if type_ == IndexedOrNamed::kIndexed {
                   unsafe {
                        let mut number: u32 = 0;
                        if !Object::to_uint32(*element, &mut number) {
                            continue;
                        }
                        let _res = args.call_indexed_query(interceptor, number);
                   }
                    attributes = args.call_indexed_query(interceptor, 0);
                } else {
                  debug_assert!(IsName(*element));
                    attributes = args.call_named_query(interceptor, Cast::<Name>(*element));
                }

               if unsafe { Isolate::has_pending_exception(self.isolate_) } {
                 return Err(());
               }
                if !attributes.is_empty() {
                    let mut value: i32 = 0;
                   unsafe { let _res = Object::to_i32(*attributes, &mut value); }
                    if (value & ONLY_ENUMERABLE.bits()) == 0 {
                         unsafe {
                             let ex = self.add_key_(&element, AddKeyConversion::DO_NOT_CONVERT);
                            if ExceptionStatus::kSuccess != ex {
                                return Err(());
                            }
                         }
                    }
                }
            }

            Ok(())
        }

        fn collect_own_property_names(&mut self, receiver: &DirectHandle<JSReceiver>, object: &DirectHandle<JSObject>) -> Maybe<bool> {
             if self.filter_ == PropertyFilter::ENUMERABLE_STRINGS {
                let enum_keys: Handle<FixedArray>;
                if object.has_fast_properties() {
                     enum_keys = KeyAccumulator::get_own_enum_property_keys(self.isolate_, object);
                     let map = object.map();
                     let nof_descriptors = map.number_of_own_descriptors();
                     if enum_keys.location().is_empty() || unsafe { (*enum_keys.location()).length() } != nof_descriptors {
                          if unsafe {(*map.location()).prototype(self.isolate_) != (*self.isolate_).read_only_roots().null_value()} {
                             let mut allow_gc = AllowGarbageCollection::new();
                             let descs = direct_handle(object.map().instance_descriptors(self.isolate_), self.isolate_);
                              for i in 0..nof_descriptors as i32 {
                                    unsafe {
                                        let details = descs.get_details(InternalIndex::new(i));
                                       if details.is_dont_enum() {
                                            self.add_shadowing_key(descs.get_key(InternalIndex::new(i)), &mut allow_gc);
                                       }
                                    }
                              }
                          }
                     }
                } else if IsJSGlobalObject(*object) {
                    unsafe { let object = Cast::<JSGlobalObject>(object.clone()); }
                    let global_dictionary = unsafe { Cast::<JSGlobalObject>(object.clone()).global_dictionary(kAcquireLoad) };
                    let dic_handle = direct_handle(global_dictionary, self.isolate_);
                    enum_keys = self.get_own_enum_property_dictionary_keys(
                         object,
                         global_dictionary
                     );
                }  else if V8_ENABLE_SWISS_NAME_DICTIONARY_BOOL {
                    let dic_handle = direct_handle(object.property_dictionary_swiss(), self.isolate_);
                    enum_keys = self.get_own_enum_property_dictionary_keys(
                         object,
                         object.property_dictionary_swiss()
                     );
                } else {
                     enum_keys = self.get_own_enum_property_dictionary_keys(
                         object,
                         object.property_dictionary()
                     );
                }
                 if IsJSModuleNamespace(*object) {
                   for i in 0..unsafe { (*enum_keys.location()).length() } {
                       unsafe {
                            let key_obj = (*enum_keys.location()).get(i);
                            if !IsString(key_obj) { continue;}
                            let key = Cast::<String>(key_obj);
                            if Cast::<JSModuleNamespace>(*object).get_export(self.isolate_, &direct_handle(key, self.isolate_)).is_null() {
                              return Maybe::nothing();
                            }
                       }
                   }
                }

                if ExceptionStatus::kSuccess != self.add_keys(&enum_keys, AddKeyConversion::DO_NOT_CONVERT) {
                  return Maybe::nothing();
                 }
             }  else {
                if object.has_fast_properties() {
                    let limit = object.map().number_of_own_descriptors();
                    let descs = direct_handle(object.map().instance_descriptors(self.isolate_), self.isolate_);
                   let first_symbol = collect_own_property_names_internal::<true>(object, self, &descs, 0, limit);
                   if first_symbol.is_err() {
                        return Maybe::nothing();
                   }
                    if first_symbol.ok().unwrap() != -1 {
                      let _ = collect_own_property_names_internal::<false>(object, self, &descs, first_symbol.ok().unwrap(), limit);
                         if first_symbol.is_err() {
                              return Maybe::nothing();
                         }
                    }
                }  else if IsJSGlobalObject(*object) {
                   unsafe { let object = Cast::<JSGlobalObject>(object.clone()); }
                    let global_dictionary = unsafe { Cast::<JSGlobalObject>(object.clone()).global_dictionary(kAcquireLoad) };
                    let dic_handle = direct_handle(global_dictionary, self.isolate_);
                    if ExceptionStatus::kSuccess != collect_keys_from_dictionary(&dic_handle, self) {
                         return Maybe::nothing();
                    }
                }   else if V8_ENABLE_SWISS_NAME_DICTIONARY_BOOL {
                    let dic_handle = direct_handle(object.property_dictionary_swiss(), self.isolate_);
                    if ExceptionStatus::kSuccess != collect_keys_from_dictionary(&dic_handle, self) {
                         return Maybe::nothing();
                    }
                } else {
                    let dic_handle = direct_handle(object.property_dictionary(), self.isolate_);
                    if ExceptionStatus::kSuccess != collect_keys_from_dictionary(&dic_handle, self) {
                         return Maybe::nothing();
                    }
                }
             }

             self.collect_interceptor_keys(receiver, object, IndexedOrNamed::kNamed)
        }
        
        fn get_own_enum_property_dictionary_keys(&mut self, object: &DirectHandle<JSObject>, dictionary: Tagged<SwissNameDictionary>) -> Handle<FixedArray> {
            let local_bool = self.mode_ == KeyCollectionMode::kOwnOnly;
            let handle = direct_handle(dictionary, self.isolate_);
             KeyAccumulator::get_own_enum_property_dictionary_keys(self.isolate_, self.mode_, if local_bool {None} else {Some(self)}, object, dictionary)
        }

         fn collect_access_check_interceptor_keys(&mut self, access_check_info: &DirectHandle<AccessCheckInfo>, receiver: &DirectHandle<JSReceiver>, object: &DirectHandle<JSObject>) -> Maybe<bool> {
           if !self.skip_indices_ {
              let info = unsafe { (*access_check_info.location()).indexed_interceptor() };
               let indexed_interceptor = direct_handle(unsafe{Cast::<InterceptorInfo>(info)}, self.isolate_);
               let maybe = self.collect_interceptor_keys_internal(
                  receiver,
                  object,
                  &indexed_interceptor,
                  IndexedOrNamed::kIndexed
               );
                if maybe.is_nothing() {
                     return Maybe::nothing();
                }
           }

           let info = unsafe { (*access_check_info.location()).named_interceptor() };
           let named_interceptor = direct_handle(unsafe{Cast::<InterceptorInfo>(info)}, self.isolate_);
           let maybe = self.collect_interceptor_keys_internal(
              receiver,
              object,
              &named_interceptor,
              IndexedOrNamed::kNamed
           );
           if maybe.is_nothing() {
                return Maybe::nothing();
           }
           Maybe::just(true)
        }
       
       fn collect_own_js_proxy_keys(&mut self, receiver: &DirectHandle<JSReceiver>, proxy: &DirectHandle<JSProxy>) -> Maybe<bool> {
         unsafe { self.collect_own_js_proxy_keys_(receiver, proxy) }
        }
    
        unsafe fn collect_own_js_proxy_keys_(&mut self, receiver: &DirectHandle<JSReceiver>, proxy: &DirectHandle<JSProxy>) -> Maybe<bool> {
            if self.filter_ == PropertyFilter::PRIVATE_NAMES_ONLY {
                if V8_ENABLE_SWISS_NAME_DICTIONARY_BOOL {
                  unsafe { let pointer = proxy.property_dictionary_swiss(); }
                    let keys = self.collect_keys_from_dictionary_inner(
                        proxy.property_dictionary_swiss()
                    );
                    if ExceptionStatus::kSuccess != keys {
                         return Maybe::nothing();
                    }
                } else {
                   unsafe { let pointer = proxy.property_dictionary(); }
                   let keys = self.collect_keys_from_dictionary_inner(
                    proxy.property_dictionary()
                );
                 if ExceptionStatus::kSuccess != keys {
                      return Maybe::nothing();
                   }
                }
                
                return Maybe::just(true);
            }
            
           let _isolate = self.isolate_;
            let handler: DirectHandle<Object> = direct_handle((*proxy.location()).handler(), self.isolate_);
           
             
            if proxy.is_revoked() {
              let e = (*self.isolate_).factory().new_type_error(
                   MessageTemplate::kProxyRevoked,
                   (*self.isolate_).factory().ownKeys_string()
              );
               Isolate::throw(self.isolate_, *e);
                return Maybe::nothing();
            }
            
            let target = direct_handle(Cast::<JSReceiver>(proxy.target()), self.isolate_);

           let get_method_result = Object::get_method(self.isolate_, Cast::<JSReceiver>(*handler), (*self.isolate_).factory().ownKeys_string());
           if get_method_result.is_err() {
                 return Maybe::nothing();
           }
            let trap = get_method_result.ok().unwrap();

           if IsUndefined(*trap, self.isolate_) {
             let x = self.collect_own_js_
