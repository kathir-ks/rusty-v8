// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

//use std::os::raw::c_void;

mod v8_internal; // Assuming v8-internal.h is converted to v8_internal.rs
mod v8_local_handle;
mod v8_maybe;
mod v8_persistent_handle;
mod v8_primitive;
mod v8_sandbox;
mod v8_traced_handle;
mod v8_value;
mod v8config;

pub mod v8 {
    use super::*;
    use v8_internal::*;
    use v8_local_handle::*;
    use v8_maybe::*;
    use v8_persistent_handle::*;
    use v8_primitive::*;
    use v8_sandbox::*;
    use v8_traced_handle::*;
    use v8_value::*;
    use v8config::*;

    // Placeholder types.  Need to be properly defined.
    pub struct Array {}
    pub struct Function {}
    pub struct FunctionTemplate {}

    pub struct PropertyCallbackInfo<T> {
        _marker: std::marker::PhantomData<T>,
    }

    /// A private symbol
    ///
    /// This is an experimental feature. Use at your own risk.
    pub struct Private {
        // Opaque data.  In a real implementation, this would contain
        // the actual data for the Private.
    }

    impl Private {
        /// Returns the print name string of the private symbol, or undefined if none.
        pub fn name(&self) -> Local<Value> {
            todo!()
        }

        /// Create a private symbol. If name is not empty, it will be the description.
        pub fn new(isolate: *mut Isolate, name: Local<String>) -> Local<Private> {
            todo!()
        }

        /// Retrieve a global private symbol. If a symbol with this name has not
        /// been retrieved in the same isolate before, it is created.
        /// Note that private symbols created this way are never collected, so
        /// they should only be used for statically fixed properties.
        /// Also, there is only one global name space for the names used as keys.
        /// To minimize the potential for clashes, use qualified names as keys,
        /// e.g., "Class#property".
        pub fn for_api(isolate: *mut Isolate, name: Local<String>) -> Local<Private> {
            todo!()
        }

        #[inline]
        pub fn cast(data: *mut Data) -> *mut Private {
            todo!()
        }

        fn check_cast(that: *mut Data) {
            todo!()
        }
    }

    impl Private {
        fn new_internal() -> Self {
            Private {}
        }
    }

    /// An instance of a Property Descriptor, see Ecma-262 6.2.4.
    ///
    /// Properties in a descriptor are present or absent. If you do not set
    /// `enumerable`, `configurable`, and `writable`, they are absent. If `value`,
    /// `get`, or `set` are absent, but you must specify them in the constructor, use
    /// empty handles.
    ///
    /// Accessors `get` and `set` must be callable or undefined if they are present.
    ///
    /// \note Only query properties if they are present, i.e., call `x()` only if
    /// `has_x()` returns true.
    ///
    /// \code
    /// // var desc = {writable: false}
    /// v8::PropertyDescriptor d(Local<Value>()), false);
    /// d.value(); // error, value not set
    /// if (d.has_writable()) {
    ///   d.writable(); // false
    /// }
    ///
    /// // var desc = {value: undefined}
    /// v8::PropertyDescriptor d(v8::Undefined(isolate));
    ///
    /// // var desc = {get: undefined}
    /// v8::PropertyDescriptor d(v8::Undefined(isolate), Local<Value>()));
    /// \endcode
    #[derive(Debug)]
    pub struct PropertyDescriptor {
        private_: *mut PrivateData,
    }

    impl PropertyDescriptor {
        // GenericDescriptor
        pub fn new() -> Self {
            PropertyDescriptor { private_: std::ptr::null_mut() }
        }

        // DataDescriptor
        pub fn new_data(value: Local<Value>) -> Self {
            PropertyDescriptor { private_: std::ptr::null_mut() }
        }

        // DataDescriptor with writable property
        pub fn new_data_writable(value: Local<Value>, writable: bool) -> Self {
            PropertyDescriptor { private_: std::ptr::null_mut() }
        }

        // AccessorDescriptor
        pub fn new_accessor(get: Local<Value>, set: Local<Value>) -> Self {
            PropertyDescriptor { private_: std::ptr::null_mut() }
        }

        pub fn value(&self) -> Local<Value> {
            todo!()
        }
        pub fn has_value(&self) -> bool {
            todo!()
        }

        pub fn get(&self) -> Local<Value> {
            todo!()
        }
        pub fn has_get(&self) -> bool {
            todo!()
        }
        pub fn set(&self) -> Local<Value> {
            todo!()
        }
        pub fn has_set(&self) -> bool {
            todo!()
        }

        pub fn set_enumerable(&mut self, enumerable: bool) {
            todo!()
        }
        pub fn enumerable(&self) -> bool {
            todo!()
        }
        pub fn has_enumerable(&self) -> bool {
            todo!()
        }

        pub fn set_configurable(&mut self, configurable: bool) {
            todo!()
        }
        pub fn configurable(&self) -> bool {
            todo!()
        }
        pub fn has_configurable(&self) -> bool {
            todo!()
        }

        pub fn writable(&self) -> bool {
            todo!()
        }
        pub fn has_writable(&self) -> bool {
            todo!()
        }

        pub fn get_private(&self) -> *mut PrivateData {
            self.private_
        }
    }

    impl Drop for PropertyDescriptor {
        fn drop(&mut self) {
            todo!()
        }
    }

    pub struct PrivateData {}

    /// PropertyAttribute.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum PropertyAttribute {
        /// None.
        None = 0,
        /// ReadOnly, i.e., not writable.
        ReadOnly = 1 << 0,
        /// DontEnum, i.e., not enumerable.
        DontEnum = 1 << 1,
        /// DontDelete, i.e., not configurable.
        DontDelete = 1 << 2,
    }

    /// Accessor[Getter|Setter] are used as callback functions when setting|getting
    /// a particular data property. See Object::SetNativeDataProperty and
    /// ObjectTemplate::SetNativeDataProperty methods.
    pub type AccessorNameGetterCallback =
        extern "C" fn(property: Local<Name>, info: &PropertyCallbackInfo<Value>);

    pub type AccessorNameSetterCallback =
        extern "C" fn(property: Local<Name>, value: Local<Value>, info: &PropertyCallbackInfo<void>);

    /// Access control specifications.
    ///
    /// Some accessors should be accessible across contexts. These
    /// accessors have an explicit access control parameter which specifies
    /// the kind of cross-context access that should be allowed.
    // TODO: Remove in V8 12.9
    #[deprecated = "This enum is no longer used and will be removed in V8 12.9."]
    pub enum AccessControl {
        DEFAULT, // V8_ENUM_DEPRECATE_SOON("not used") = 0,
    }

    /// Property filter bits. They can be or'ed to build a composite filter.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum PropertyFilter {
        ALL_PROPERTIES = 0,
        ONLY_WRITABLE = 1,
        ONLY_ENUMERABLE = 2,
        ONLY_CONFIGURABLE = 4,
        SKIP_STRINGS = 8,
        SKIP_SYMBOLS = 16,
    }

    /// Options for marking whether callbacks may trigger JS-observable side effects.
    /// Side-effect-free callbacks are allowlisted during debug evaluation with
    /// throwOnSideEffect. It applies when calling a Function, FunctionTemplate,
    /// or an Accessor callback. For Interceptors, please see
    /// PropertyHandlerFlags's kHasNoSideEffect.
    /// Callbacks that only cause side effects to the receiver are allowlisted if
    /// invoked on receiver objects that are created within the same debug-evaluate
    /// call, as these objects are temporary and the side effect does not escape.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum SideEffectType {
        kHasSideEffect,
        kHasNoSideEffect,
        kHasSideEffectToReceiver,
    }

    /// Keys/Properties filter enums:
    ///
    /// KeyCollectionMode limits the range of collected properties. kOwnOnly limits
    /// the collected properties to the given Object only. kIncludesPrototypes will
    /// include all keys of the objects's prototype chain as well.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum KeyCollectionMode {
        kOwnOnly,
        kIncludePrototypes,
    }

    /// kIncludesIndices allows for integer indices to be collected, while
    /// kSkipIndices will exclude integer indices from being collected.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum IndexFilter {
        kIncludeIndices,
        kSkipIndices,
    }

    /// kConvertToString will convert integer indices to strings.
    /// kKeepNumbers will return numbers for integer indices.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum KeyConversionMode {
        kConvertToString,
        kKeepNumbers,
        kNoNumbers,
    }

    /// Integrity level for objects.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum IntegrityLevel {
        kFrozen,
        kSealed,
    }

    /// A JavaScript object (ECMA-262, 4.3.3)
    pub struct Object {
        // Opaque data.  In a real implementation, this would contain
        // the actual data for the Object.
    }

    impl Object {
        /// Set only return Just(true) or Empty(), so if it should never fail, use
        /// result.Check().
        pub fn set(
            &self,
            context: Local<Context>,
            key: Local<Value>,
            value: Local<Value>,
        ) -> Maybe<bool> {
            todo!()
        }
        pub fn set_with_receiver(
            &self,
            context: Local<Context>,
            key: Local<Value>,
            value: Local<Value>,
            receiver: MaybeLocal<Object>,
        ) -> Maybe<bool> {
            todo!()
        }

        pub fn set_index(
            &self,
            context: Local<Context>,
            index: u32,
            value: Local<Value>,
        ) -> Maybe<bool> {
            todo!()
        }

        /// Implements CreateDataProperty(O, P, V), see
        /// https://tc39.es/ecma262/#sec-createdataproperty.
        ///
        /// Defines a configurable, writable, enumerable property with the given value
        /// on the object unless the property already exists and is not configurable
        /// or the object is not extensible.
        ///
        /// Returns true on success.
        pub fn create_data_property(
            &self,
            context: Local<Context>,
            key: Local<Name>,
            value: Local<Value>,
        ) -> Maybe<bool> {
            todo!()
        }
        pub fn create_data_property_index(
            &self,
            context: Local<Context>,
            index: u32,
            value: Local<Value>,
        ) -> Maybe<bool> {
            todo!()
        }

        /// Implements [[DefineOwnProperty]] for data property case, see
        /// https://tc39.es/ecma262/#table-essential-internal-methods.
        ///
        /// In general, CreateDataProperty will be faster, however, does not allow
        /// for specifying attributes.
        ///
        /// Returns true on success.
        pub fn define_own_property(
            &self,
            context: Local<Context>,
            key: Local<Name>,
            value: Local<Value>,
            attributes: PropertyAttribute,
        ) -> Maybe<bool> {
            todo!()
        }

        /// Implements Object.defineProperty(O, P, Attributes), see
        /// https://tc39.es/ecma262/#sec-object.defineproperty.
        ///
        /// The defineProperty function is used to add an own property or
        /// update the attributes of an existing own property of an object.
        ///
        /// Both data and accessor descriptors can be used.
        ///
        /// In general, CreateDataProperty is faster, however, does not allow
        /// for specifying attributes or an accessor descriptor.
        ///
        /// The PropertyDescriptor can change when redefining a property.
        ///
        /// Returns true on success.
        pub fn define_property(
            &self,
            context: Local<Context>,
            key: Local<Name>,
            descriptor: &mut PropertyDescriptor,
        ) -> Maybe<bool> {
            todo!()
        }

        pub fn get(&self, context: Local<Context>, key: Local<Value>) -> MaybeLocal<Value> {
            todo!()
        }
        pub fn get_with_receiver(
            &self,
            context: Local<Context>,
            key: Local<Value>,
            receiver: MaybeLocal<Object>,
        ) -> MaybeLocal<Value> {
            todo!()
        }

        pub fn get_index(&self, context: Local<Context>, index: u32) -> MaybeLocal<Value> {
            todo!()
        }

        /// Gets the property attributes of a property which can be None or
        /// any combination of ReadOnly, DontEnum and DontDelete. Returns
        /// None when the property doesn't exist.
        pub fn get_property_attributes(
            &self,
            context: Local<Context>,
            key: Local<Value>,
        ) -> Maybe<PropertyAttribute> {
            todo!()
        }

        /// Implements Object.getOwnPropertyDescriptor(O, P), see
        /// https://tc39.es/ecma262/#sec-object.getownpropertydescriptor.
        pub fn get_own_property_descriptor(
            &self,
            context: Local<Context>,
            key: Local<Name>,
        ) -> MaybeLocal<Value> {
            todo!()
        }

        /// Object::Has() calls the abstract operation HasProperty(O, P), see
        /// https://tc39.es/ecma262/#sec-hasproperty. Has() returns
        /// true, if the object has the property, either own or on the prototype chain.
        /// Interceptors, i.e., PropertyQueryCallbacks, are called if present.
        ///
        /// Has() has the same side effects as JavaScript's `variable in object`.
        /// For example, calling Has() on a revoked proxy will throw an exception.
        ///
        /// \note Has() converts the key to a name, which possibly calls back into
        /// JavaScript.
        ///
        /// See also v8::Object::HasOwnProperty() and
        /// v8::Object::HasRealNamedProperty().
        pub fn has(&self, context: Local<Context>, key: Local<Value>) -> Maybe<bool> {
            todo!()
        }

        pub fn delete(&self, context: Local<Context>, key: Local<Value>) -> Maybe<bool> {
            todo!()
        }

        pub fn has_index(&self, context: Local<Context>, index: u32) -> Maybe<bool> {
            todo!()
        }

        pub fn delete_index(&self, context: Local<Context>, index: u32) -> Maybe<bool> {
            todo!()
        }

        /// Sets an accessor property like Template::SetAccessorProperty, but
        /// this method sets on this object directly.
        pub fn set_accessor_property(
            &self,
            name: Local<Name>,
            getter: Local<Function>,
            setter: Local<Function>,
            attributes: PropertyAttribute,
        ) {
            todo!()
        }

        /// Sets a native data property like Template::SetNativeDataProperty, but
        /// this method sets on this object directly.
        pub fn set_native_data_property(
            &self,
            context: Local<Context>,
            name: Local<Name>,
            getter: AccessorNameGetterCallback,
            setter: Option<AccessorNameSetterCallback>,
            data: Local<Value>,
            attributes: PropertyAttribute,
            getter_side_effect_type: SideEffectType,
            setter_side_effect_type: SideEffectType,
        ) -> Maybe<bool> {
            todo!()
        }

        /// Attempts to create a property with the given name which behaves like a data
        /// property, except that the provided getter is invoked (and provided with the
        /// data value) to supply its value the first time it is read. After the
        /// property is accessed once, it is replaced with an ordinary data property.
        ///
        /// Analogous to Template::SetLazyDataProperty.
        pub fn set_lazy_data_property(
            &self,
            context: Local<Context>,
            name: Local<Name>,
            getter: AccessorNameGetterCallback,
            data: Local<Value>,
            attributes: PropertyAttribute,
            getter_side_effect_type: SideEffectType,
            setter_side_effect_type: SideEffectType,
        ) -> Maybe<bool> {
            todo!()
        }

        /// Functionality for private properties.
        /// This is an experimental feature, use at your own risk.
        /// Note: Private properties are not inherited. Do not rely on this, since it
        /// may change.
        pub fn has_private(&self, context: Local<Context>, key: Local<Private>) -> Maybe<bool> {
            todo!()
        }
        pub fn set_private(
            &self,
            context: Local<Context>,
            key: Local<Private>,
            value: Local<Value>,
        ) -> Maybe<bool> {
            todo!()
        }
        pub fn delete_private(
            &self,
            context: Local<Context>,
            key: Local<Private>,
        ) -> Maybe<bool> {
            todo!()
        }
        pub fn get_private(
            &self,
            context: Local<Context>,
            key: Local<Private>,
        ) -> MaybeLocal<Value> {
            todo!()
        }

        /// Returns an array containing the names of the enumerable properties
        /// of this object, including properties from prototype objects.  The
        /// array returned by this method contains the same values as would
        /// be enumerated by a for-in statement over this object.
        pub fn get_property_names(&self, context: Local<Context>) -> MaybeLocal<Array> {
            todo!()
        }
        pub fn get_property_names_filtered(
            &self,
            context: Local<Context>,
            mode: KeyCollectionMode,
            property_filter: PropertyFilter,
            index_filter: IndexFilter,
            key_conversion: KeyConversionMode,
        ) -> MaybeLocal<Array> {
            todo!()
        }

        /// This function has the same functionality as GetPropertyNames but
        /// the returned array doesn't contain the names of properties from
        /// prototype objects.
        pub fn get_own_property_names(&self, context: Local<Context>) -> MaybeLocal<Array> {
            todo!()
        }

        /// Returns an array containing the names of the filtered properties
        /// of this object, including properties from prototype objects.  The
        /// array returned by this method contains the same values as would
        /// be enumerated by a for-in statement over this object.
        pub fn get_own_property_names_filtered(
            &self,
            context: Local<Context>,
            filter: PropertyFilter,
            key_conversion: KeyConversionMode,
        ) -> MaybeLocal<Array> {
            todo!()
        }

        /// Get the prototype object.  This does not skip objects marked to
        /// be skipped by __proto__ and it does not consult the security
        /// handler.
        #[deprecated = "V8 will stop providing access to hidden prototype (i.e. JSGlobalObject). Use GetPrototypeV2() instead. See http://crbug.com/333672197."]
        pub fn get_prototype(&self) -> Local<Value> {
            todo!()
        }

        /// Get the prototype object (same as getting __proto__ property).  This does
        /// not consult the security handler.
        /// TODO(333672197): rename back to GetPrototype() once the old version goes
        /// through the deprecation process and is removed.
        pub fn get_prototype_v2(&self) -> Local<Value> {
            todo!()
        }

        /// Set the prototype object.  This does not skip objects marked to
        /// be skipped by __proto__ and it does not consult the security
        /// handler.
        #[deprecated = "V8 will stop providing access to hidden prototype (i.e. JSGlobalObject). Use SetPrototypeV2() instead. See http://crbug.com/333672197."]
        pub fn set_prototype(
            &self,
            context: Local<Context>,
            prototype: Local<Value>,
        ) -> Maybe<bool> {
            todo!()
        }

        /// Set the prototype object (same as setting __proto__ property).  This does
        /// does not consult the security handler.
        /// TODO(333672197): rename back to SetPrototype() once the old version goes
        /// through the deprecation process and is removed.
        pub fn set_prototype_v2(
            &self,
            context: Local<Context>,
            prototype: Local<Value>,
        ) -> Maybe<bool> {
            todo!()
        }

        /// Finds an instance of the given function template in the prototype
        /// chain.
        pub fn find_instance_in_prototype_chain(
            &self,
            tmpl: Local<FunctionTemplate>,
        ) -> Local<Object> {
            todo!()
        }

        /// Call builtin Object.prototype.toString on this object.
        /// This is different from Value::ToString() that may call
        /// user-defined toString function. This one does not.
        pub fn object_proto_to_string(&self, context: Local<Context>) -> MaybeLocal<String> {
            todo!()
        }

        /// Returns the name of the function invoked as a constructor for this object.
        pub fn get_constructor_name(&self) -> Local<String> {
            todo!()
        }

        /// Sets the integrity level of the object.
        pub fn set_integrity_level(
            &self,
            context: Local<Context>,
            level: IntegrityLevel,
        ) -> Maybe<bool> {
            todo!()
        }

        /// Gets the number of internal fields for this Object.
        pub fn internal_field_count(&self) -> i32 {
            todo!()
        }

        /// Same as above, but works for PersistentBase.
        #[inline]
        pub fn internal_field_count_persistent(object: &PersistentBase<Object>) -> i32 {
            object.value::<Object>().internal_field_count()
        }

        /// Same as above, but works for BasicTracedReference.
        #[inline]
        pub fn internal_field_count_traced(object: &BasicTracedReference<Object>) -> i32 {
            object.value::<Object>().internal_field_count()
        }

        /// Gets the data from an internal field.
        /// To cast the return value into v8::Value subtypes, it needs to be
        /// casted to a v8::Value first. For example, to cast it into v8::External:
        ///
        /// object->GetInternalField(index).As<v8::Value>().As<v8::External>();
        ///
        /// The embedder should make sure that the internal field being retrieved
        /// using this method has already been set with SetInternalField() before.
        #[inline]
        pub fn get_internal_field(&self, index: i32) -> Local<Data> {
            todo!()
        }

        /// Sets the data in an internal field.
        pub fn set_internal_field(&self, index: i32, data: Local<Data>) {
            todo!()
        }

        /// Gets a 2-byte-aligned native pointer from an internal field. This field
        /// must have been set by SetAlignedPointerInInternalField, everything else
        /// leads to undefined behavior.
        #[inline]
        pub fn get_aligned_pointer_from_internal_field(&self, index: i32) -> *mut std::ffi::c_void {
            todo!()
        }
        #[inline]
        pub fn get_aligned_pointer_from_internal_field_isolate(&self, isolate: *mut Isolate, index: i32) -> *mut std::ffi::c_void {
            todo!()
        }

        /// Same as above, but works for PersistentBase.
        #[inline]
        pub fn get_aligned_pointer_from_internal_field_persistent(
            object: &PersistentBase<Object>,
            index: i32,
        ) -> *mut std::ffi::c_void {
            object.value::<Object>().get_aligned_pointer_from_internal_field(index)
        }

        /// Same as above, but works for TracedReference.
        #[inline]
        pub fn get_aligned_pointer_from_internal_field_traced(
            object: &BasicTracedReference<Object>,
            index: i32,
        ) -> *mut std::ffi::c_void {
            object.value::<Object>().get_aligned_pointer_from_internal_field(index)
        }

        /// Sets a 2-byte-aligned native pointer in an internal field. To retrieve such
        /// a field, GetAlignedPointerFromInternalField must be used, everything else
        /// leads to undefined behavior.
        pub fn set_aligned_pointer_in_internal_field(&self, index: i32, value: *mut std::ffi::c_void) {
            todo!()
        }
        pub fn set_aligned_pointer_in_internal_fields(
            &self,
            argc: i32,
            indices: &mut [i32],
            values: &mut [*mut std::ffi::c_void],
        ) {
            todo!()
        }

        /// Unwraps a JS wrapper object.
        ///
        /// \param tag The tag for retrieving the wrappable instance. Must match the
        /// tag that has been used for a previous `Wrap()` operation.
        /// \param isolate The Isolate for the `wrapper` object.
        /// \param wrapper The JS wrapper object that should be unwrapped.
        /// \returns the C++ wrappable instance, or nullptr if the JS object has never
        /// been wrapped.
        pub fn unwrap<T>(
            isolate: *mut Isolate,
            wrapper: &Local<Object>,
            tag_range: CppHeapPointerTagRange
        ) -> *mut T {
            todo!()
        }
        pub fn unwrap_persistent<T>(
            isolate: *mut Isolate,
            wrapper: &PersistentBase<Object>,
            tag_range: CppHeapPointerTagRange
        ) -> *mut T {
            todo!()
        }
        pub fn unwrap_traced<T>(
            isolate: *mut Isolate,
            wrapper: &BasicTracedReference<Object>,
            tag_range: CppHeapPointerTagRange
        ) -> *mut T {
            todo!()
        }
        pub fn wrap(
            isolate: *mut Isolate,
            wrapper: &Local<Object>,
            wrappable: *mut std::ffi::c_void,
            tag: CppHeapPointerTag
        ) {
            todo!()
        }
        pub fn wrap_persistent(
            isolate: *mut Isolate,
            wrapper: &PersistentBase<Object>,
            wrappable: *mut std::ffi::c_void,
            tag: CppHeapPointerTag
        ) {
            todo!()
        }
        pub fn wrap_traced(
            isolate: *mut Isolate,
            wrapper: &BasicTracedReference<Object>,
            wrappable: *mut std::ffi::c_void,
            tag: CppHeapPointerTag
        ) {
            todo!()
        }
        /// HasOwnProperty() is like JavaScript's
        /// Object.prototype.hasOwnProperty().
        ///
        /// See also v8::Object::Has() and v8::Object::HasRealNamedProperty().
        pub fn has_own_property(&self, context: Local<Context>, key: Local<Name>) -> Maybe<bool> {
            todo!()
        }
        pub fn has_own_property_index(&self, context: Local<Context>, index: u32) -> Maybe<bool> {
            todo!()
        }
        /// Use HasRealNamedProperty() if you want to check if an object has an own
        /// property without causing side effects, i.e., without calling interceptors.
        ///
        /// This function is similar to v8::Object::HasOwnProperty(), but it does not
        /// call interceptors.
        ///
        /// \note Consider using non-masking interceptors, i.e., the interceptors are
        /// not called if the receiver has the real named property. See
        /// `v8::PropertyHandlerFlags::kNonMasking`.
        ///
        /// See also v8::Object::Has().
        pub fn has_real_named_property(&self, context: Local<Context>, key: Local<Name>) -> Maybe<bool> {
            todo!()
        }
        pub fn has_real_indexed_property(&self, context: Local<Context>, index: u32) -> Maybe<bool> {
            todo!()
        }
        pub fn has_real_named_callback_property(&self, context: Local<Context>, key: Local<Name>) -> Maybe<bool> {
            todo!()
        }

        /// If result.IsEmpty() no real property was located in the prototype chain.
        /// This means interceptors in the prototype chain are not called.
        pub fn get_real_named_property_in_prototype_chain(
            &self,
            context: Local<Context>,
            key: Local<Name>,
        ) -> MaybeLocal<Value> {
            todo!()
        }

        /// Gets the property attributes of a real property in the prototype chain,
        /// which can be None or any combination of ReadOnly, DontEnum and DontDelete.
        /// Interceptors in the prototype chain are not called.
        pub fn get_real_named_property_attributes_in_prototype_chain(
            &self,
            context: Local<Context>,
            key: Local<Name>,
        ) -> Maybe<PropertyAttribute> {
            todo!()
        }

        /// If result.IsEmpty() no real property was located on the object or
        /// in the prototype chain.
        /// This means interceptors in the prototype chain are not called.
        pub fn get_real_named_property(
            &self,
            context: Local<Context>,
            key: Local<Name>,
        ) -> MaybeLocal<Value> {
            todo!()
        }

        /// Gets the property attributes of a real property which can be
        /// None or any combination of ReadOnly, DontEnum and DontDelete.
        /// Interceptors in the prototype chain are not called.
        pub fn get_real_named_property_attributes(
            &self,
            context: Local<Context>,
            key: Local<Name>,
        ) -> Maybe<PropertyAttribute> {
            todo!()
        }

        /// Tests for a named lookup interceptor.
        pub fn has_named_lookup_interceptor(&self) -> bool {
            todo!()
        }

        /// Tests for an index lookup interceptor.
        pub fn has_indexed_lookup_interceptor(&self) -> bool {
            todo!()
        }

        /// Returns the identity hash for this object. The current implementation
        /// uses a hidden property on the object to store the identity hash.
        ///
        /// The return value will never be 0. Also, it is not guaranteed to be
        /// unique.
        pub fn get_identity_hash(&self) -> i32 {
            todo!()
        }

        /// Clone this object with a fast but shallow copy. Values will point to the
        /// same values as the original object.
        ///
        /// Prefer using version with Isolate parameter.
        pub fn clone_with_isolate(&self, isolate: *mut Isolate) -> Local<Object> {
            todo!()
        }
        pub fn clone(&self) -> Local<Object> {
            todo!()
        }

        /// Returns the context in which the object was created.
        ///
        /// Prefer using version with Isolate parameter.
        pub fn get_creation_context_with_isolate(&self, isolate: *mut Isolate) -> MaybeLocal<Context> {
            todo!()
        }
        #[deprecated = "Use the version with the isolate argument."]
        pub fn get_creation_context(&self) -> MaybeLocal<Context> {
            todo!()
        }

        /// Shortcut for GetCreationContext(...).ToLocalChecked().
        ///
        /// Prefer using version with Isolate parameter.
        pub fn get_creation_context_checked_with_isolate(&self, isolate: *mut Isolate) -> Local<Context> {
            todo!()
        }
        #[deprecated = "Use the version with the isolate argument."]
        pub fn get_creation_context_checked(&self) -> Local<Context> {
            todo!()
        }

        /// Same as above, but works for Persistents
        #[inline]
        pub fn get_creation_context_persistent_with_isolate(
            isolate: *mut Isolate,
            object: &PersistentBase<Object>,
        ) -> MaybeLocal<Context> {
            object.