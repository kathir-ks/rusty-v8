// Converted from V8 C++ source files:
// Header: v8-primitive.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8_primitive {
    use crate::v8::{
        v8_data::Data,
        v8_internal::{Address, Internals},
        v8_local_handle::Local,
        v8_value::Value,
        v8config,
    };
    use std::{
        ffi::{c_char, c_void},
        mem::MaybeUninit,
        ptr,
        sync::{Arc, Mutex, RwLock},
    };

    pub struct Context {}
    pub struct Isolate {}
    pub struct String {}

    pub mod internal {
        pub struct ExternalString {}
        pub struct ScopedExternalStringLock {}
        pub struct StringForwardingTable {}
    }

    /**
     * The superclass of primitive values.  See ECMA-262 4.3.2.
     */
    pub struct Primitive {}

    /**
     * A primitive boolean value (ECMA-262, 4.3.14).  Either the true
     * or false value.
     */
    pub struct Boolean {}

    impl Boolean {
        pub fn value(&self) -> bool {
            true // Placeholder
        }
        #[inline]
        pub fn cast(data: *mut Data) -> *mut Boolean {
            #[cfg(v8_enable_checks)]
            Self::check_cast(data);
            data as *mut Boolean
        }

        #[inline]
        pub fn new(isolate: *mut Isolate, value: bool) -> Local<'static, Boolean> {
            if value {
                True(isolate)
            } else {
                False(isolate)
            }
        }

        fn check_cast(that: *mut Data) {
            // Placeholder
        }
    }

    /**
     * An array to hold Primitive values. This is used by the embedder to
     * pass host defined options to the ScriptOptions during compilation.
     *
     * This is passed back to the embedder as part of
     * HostImportModuleDynamicallyCallback for module loading.
     */
    pub struct PrimitiveArray {}

    impl PrimitiveArray {
        pub fn new(isolate: *mut Isolate, length: i32) -> Local<'static, PrimitiveArray> {
            Local::empty() // Placeholder
        }
        pub fn length(&self) -> i32 {
            0 // Placeholder
        }
        pub fn set(isolate: *mut Isolate, index: i32, item: Local<'static, Primitive>) {
            // Placeholder
        }
        pub fn get(isolate: *mut Isolate, index: i32) -> Local<'static, Primitive> {
            Local::empty() // Placeholder
        }

        #[inline]
        pub fn cast(data: *mut Data) -> *mut PrimitiveArray {
            #[cfg(v8_enable_checks)]
            Self::check_cast(data);
            data as *mut PrimitiveArray
        }

        fn check_cast(obj: *mut Data) {
            // Placeholder
        }
    }

    /**
     * A superclass for symbols and strings.
     */
    pub struct Name {}

    impl Name {
        /**
         * Returns the identity hash for this object. The current implementation
         * uses an inline property on the object to store the identity hash.
         *
         * The return value will never be 0. Also, it is not guaranteed to be
         * unique.
         */
        pub fn get_identity_hash(&self) -> i32 {
            1 // Placeholder
        }

        #[inline]
        pub fn cast(data: *mut Data) -> *mut Name {
            #[cfg(v8_enable_checks)]
            Self::check_cast(data);
            data as *mut Name
        }

        fn check_cast(that: *mut Data) {
            // Placeholder
        }
    }

    /**
     * A flag describing different modes of string creation.
     *
     * Aside from performance implications there are no differences between the two
     * creation modes.
     */
    pub enum NewStringType {
        /**
         * Create a new string, always allocating new storage memory.
         */
        kNormal,

        /**
         * Acts as a hint that the string should be created in the
         * old generation heap space and be deduplicated if an identical string
         * already exists.
         */
        kInternalized,
    }

    /**
     * A JavaScript string value (ECMA-262, 4.3.17).
     */
    pub struct String {}

    impl String {
        pub const K_MAX_LENGTH: i32 = if 8 == 4 { (1 << 28) - 16 } else { (1 << 29) - 24 };

        pub enum Encoding {
            UNKNOWN_ENCODING = 0x1,
            TWO_BYTE_ENCODING = 0x0,
            ONE_BYTE_ENCODING = 0x8,
        }
        /**
         * Returns the number of characters (UTF-16 code units) in this string.
         */
        pub fn length(&self) -> i32 {
            0 // Placeholder
        }

        /**
         * Returns the number of bytes in the UTF-8 encoded
         * representation of this string.
         */
        #[deprecated(note = "Use Utf8LengthV2 instead.")]
        pub fn utf8_length(&self, isolate: *mut Isolate) -> i32 {
            0 // Placeholder
        }

        /**
         * Returns the number of bytes needed for the Utf8 encoding of this string.
         */
        pub fn utf8_length_v2(&self, isolate: *mut Isolate) -> usize {
            0 // Placeholder
        }

        /**
         * Returns whether this string is known to contain only one byte data,
         * i.e. ISO-8859-1 code points.
         * Does not read the string.
         * False negatives are possible.
         */
        pub fn is_one_byte(&self) -> bool {
            true // Placeholder
        }

        /**
         * Returns whether this string contain only one byte data,
         * i.e. ISO-8859-1 code points.
         * Will read the entire string in some cases.
         */
        pub fn contains_only_one_byte(&self) -> bool {
            true // Placeholder
        }

        /**
         * Write the contents of the string to an external buffer.
         * If no arguments are given, expects the buffer to be large
         * enough to hold the entire string and NULL terminator. Copies
         * the contents of the string and the NULL terminator into the
         * buffer.
         *
         * WriteUtf8 will not write partial UTF-8 sequences, preferring to stop
         * before the end of the buffer.
         *
         * Copies up to length characters into the output buffer.
         * Only null-terminates if there is enough space in the buffer.
         *
         * \param buffer The buffer into which the string will be copied.
         * \param start The starting position within the string at which
         * copying begins.
         * \param length The number of characters to copy from the string.  For
         *    WriteUtf8 the number of bytes in the buffer.
         * \param nchars_ref The number of characters written, can be NULL.
         * \param options Various options that might affect performance of this or
         *    subsequent operations.
         * \return The number of characters copied to the buffer excluding the null
         *    terminator.  For WriteUtf8: The number of bytes copied to the buffer
         *    including the null terminator (if written).
         */
        pub enum WriteOptions {
            NO_OPTIONS = 0,
            HINT_MANY_WRITES_EXPECTED = 1,
            NO_NULL_TERMINATION = 2,
            PRESERVE_ONE_BYTE_NULL = 4,
            // Used by WriteUtf8 to replace orphan surrogate code units with the
            // unicode replacement character. Needs to be set to guarantee valid UTF-8
            // output.
            REPLACE_INVALID_UTF8 = 8,
        }

        // 16-bit character codes.
        #[deprecated(note = "Use WriteV2 instead.")]
        pub fn write(
            &self,
            isolate: *mut Isolate,
            buffer: *mut u16,
            start: i32,
            length: i32,
            options: i32,
        ) -> i32 {
            0 // Placeholder
        }
        // One byte characters.
        #[deprecated(note = "Use WriteOneByteV2 instead.")]
        pub fn write_one_byte(
            &self,
            isolate: *mut Isolate,
            buffer: *mut u8,
            start: i32,
            length: i32,
            options: i32,
        ) -> i32 {
            0 // Placeholder
        }
        // UTF-8 encoded characters.
        #[deprecated(note = "Use WriteUtf8V2 instead.")]
        pub fn write_utf8(
            &self,
            isolate: *mut Isolate,
            buffer: *mut c_char,
            length: i32,
            nchars_ref: *mut i32,
            options: i32,
        ) -> i32 {
            0 // Placeholder
        }

        pub struct WriteFlags {}

        impl WriteFlags {
            pub const K_NONE: i32 = 0;
            // Indicates that the output string should be null-terminated. In that
            // case, the output buffer must include sufficient space for the
            // additional null character.
            pub const K_NULL_TERMINATE: i32 = 1;
            // Used by WriteUtf8 to replace orphan surrogate code units with the
            // unicode replacement character. Needs to be set to guarantee valid UTF-8
            // output.
            pub const K_REPLACE_INVALID_UTF8: i32 = 2;
        }

        /**
         * Write the contents of the string to an external buffer.
         *
         * Copies length characters into the output buffer starting at offset. The
         * output buffer must have sufficient space for all characters and the null
         * terminator if null termination is requested through the flags.
         *
         * \param offset The position within the string at which copying begins.
         * \param length The number of characters to copy from the string.
         * \param buffer The buffer into which the string will be copied.
         * \param flags Various flags that influence the behavior of this operation.
         */
        pub fn write_v2(
            &self,
            isolate: *mut Isolate,
            offset: u32,
            length: u32,
            buffer: *mut u16,
            flags: i32,
        ) {
            // Placeholder
        }
        pub fn write_one_byte_v2(
            &self,
            isolate: *mut Isolate,
            offset: u32,
            length: u32,
            buffer: *mut u8,
            flags: i32,
        ) {
            // Placeholder
        }

        /**
         * Encode the contents of the string as Utf8 into an external buffer.
         *
         * Encodes the characters of this string as Utf8 and writes them into the
         * output buffer until either all characters were encoded or the buffer is
         * full. Will not write partial UTF-8 sequences, preferring to stop before
         * the end of the buffer. If null termination is requested, the output buffer
         * will always be null terminated even if not all characters fit. In that
         * case, the capacity must be at least one. The required size of the output
         * buffer can be determined using Utf8Length().
         *
         * \param buffer The buffer into which the string will be written.
         * \param capacity The number of bytes available in the output buffer.
         * \param flags Various flags that influence the behavior of this operation.
         * \param processed_characters_return The number of processed characters from
         * the buffer.
         * \return The number of bytes copied to the buffer including the null
         * terminator (if written).
         */
        pub fn write_utf8_v2(
            &self,
            isolate: *mut Isolate,
            buffer: *mut c_char,
            capacity: usize,
            flags: i32,
            processed_characters_return: *mut usize,
        ) -> usize {
            0 // Placeholder
        }

        /**
         * A zero length string.
         */
        #[inline]
        pub fn empty(isolate: *mut Isolate) -> Local<'static, String> {
            let s: *mut Address = Internals::get_root_slot(isolate, Internals::K_EMPTY_STRING_ROOT_INDEX);
            Local::from_slot(s)
        }

        /**
         * Returns true if the string is external.
         */
        pub fn is_external(&self) -> bool {
            false // Placeholder
        }

        /**
         * Returns true if the string is both external and two-byte.
         */
        pub fn is_external_two_byte(&self) -> bool {
            false // Placeholder
        }

        /**
         * Returns true if the string is both external and one-byte.
         */
        pub fn is_external_one_byte(&self) -> bool {
            false // Placeholder
        }

        /**
         * Returns the internalized string. See `NewStringType::kInternalized` for
         * details on internalized strings.
         */
        pub fn internalize_string(&self, isolate: *mut Isolate) -> Local<'static, String> {
            Local::empty() // Placeholder
        }

        pub struct ExternalStringResourceBase {}

        impl ExternalStringResourceBase {
            /**
             * If a string is cacheable, the value returned by
             * ExternalStringResource::data() may be cached, otherwise it is not
             * expected to be stable beyond the current top-level task.
             */
            pub fn is_cacheable(&self) -> bool {
                true // Placeholder
            }

            /**
             * Internally V8 will call this Unaccount method when the external string
             * resource should be unaccounted for. This method can be overridden in
             * subclasses to control how allocated external bytes are accounted.
             */
            pub fn unaccount(&self, isolate: *mut Isolate) {
                // Placeholder
            }

            /**
             * Returns an estimate of the memory occupied by this external string, to be
             * used by V8 when producing a heap snapshot. If this function returns
             * kDefaultMemoryEstimate, then V8 will estimate the external size based on
             * the string length. This function should return only memory that is
             * uniquely owned by this resource. If the resource has shared ownership of
             * a secondary allocation, it can report that memory by implementing
             * EstimateSharedMemoryUsage.
             */
            pub fn estimate_memory_usage(&self) -> usize {
                Self::K_DEFAULT_MEMORY_ESTIMATE
            }
            pub const K_DEFAULT_MEMORY_ESTIMATE: usize = -1_isize as usize;

            pub struct SharedMemoryUsageRecorder {}

            impl SharedMemoryUsageRecorder {
                /**
                 * Record that a shared allocation at the given location has the given
                 * size.
                 */
                pub fn record_shared_memory_usage(&mut self, location: *const c_void, size: usize) {
                    // Placeholder
                }
            }

            /**
             * Estimates memory that this string resource may share with other string
             * resources, to be used by V8 when producing a heap snapshot.
             */
            pub fn estimate_shared_memory_usage(&self, recorder: *mut SharedMemoryUsageRecorder) {
                // Placeholder
            }

            /**
             * Internally V8 will call this Dispose method when the external string
             * resource is no longer needed. The default implementation will use the
             * delete operator. This method can be overridden in subclasses to
             * control how allocated external string resources are disposed.
             */
            pub fn dispose(&mut self) {
                // Placeholder
            }

            /**
             * For a non-cacheable string, the value returned by
             * |ExternalStringResource::data()| has to be stable between |Lock()| and
             * |Unlock()|, that is the string must behave as is |IsCacheable()| returned
             * true.
             *
             * These two functions must be thread-safe, and can be called from anywhere.
             * They also must handle lock depth, in the sense that each can be called
             * several times, from different threads, and unlocking should only happen
             * when the balance of Lock() and Unlock() calls is 0.
             */
            pub fn lock(&self) {}

            /**
             * Unlocks the string.
             */
            pub fn unlock(&self) {}
        }

        /**
         * An ExternalStringResource is a wrapper around a two-byte string
         * buffer that resides outside V8's heap. Implement an
         * ExternalStringResource to manage the life cycle of the underlying
         * buffer.  Note that the string data must be immutable.
         */
        pub struct ExternalStringResource {}

        impl ExternalStringResource {
            /**
             * The string data from the underlying buffer. If the resource is cacheable
             * then data() must return the same value for all invocations.
             */
            pub fn data(&self) -> *const u16 {
                ptr::null() // Placeholder
            }

            /**
             * The length of the string. That is, the number of two-byte characters.
             */
            pub fn length(&self) -> usize {
                0 // Placeholder
            }

            /**
             * Returns the cached data from the underlying buffer. This method can be
             * called only for cacheable resources (i.e. IsCacheable() == true) and only
             * after UpdateDataCache() was called.
             */
            pub fn cached_data(&self) -> *const u16 {
                self.check_cached_data_invariants();
                self.cached_data_
            }

            /**
             * Update {cached_data_} with the data from the underlying buffer. This can
             * be called only for cacheable resources.
             */
            pub fn update_data_cache(&mut self) {
                // Placeholder
            }

            fn check_cached_data_invariants(&self) {}

            cached_data_: *const u16,
        }

        /**
         * An ExternalOneByteStringResource is a wrapper around an one-byte
         * string buffer that resides outside V8's heap. Implement an
         * ExternalOneByteStringResource to manage the life cycle of the
         * underlying buffer.  Note that the string data must be immutable
         * and that the data must be Latin-1 and not UTF-8, which would require
         * special treatment internally in the engine and do not allow efficient
         * indexing.  Use String::New or convert to 16 bit data for non-Latin1.
         */
        pub struct ExternalOneByteStringResource {}

        impl ExternalOneByteStringResource {
            /**
             * The string data from the underlying buffer. If the resource is cacheable
             * then data() must return the same value for all invocations.
             */
            pub fn data(&self) -> *const c_char {
                ptr::null() // Placeholder
            }

            /** The number of Latin-1 characters in the string.*/
            pub fn length(&self) -> usize {
                0 // Placeholder
            }

            /**
             * Returns the cached data from the underlying buffer. If the resource is
             * uncacheable or if UpdateDataCache() was not called before, it has
             * undefined behaviour.
             */
            pub fn cached_data(&self) -> *const c_char {
                self.check_cached_data_invariants();
                self.cached_data_
            }

            /**
             * Update {cached_data_} with the data from the underlying buffer. This can
             * be called only for cacheable resources.
             */
            pub fn update_data_cache(&mut self) {
                // Placeholder
            }

            fn check_cached_data_invariants(&self) {}

            cached_data_: *const c_char,
        }

        /**
         * If the string is an external string, return the ExternalStringResourceBase
         * regardless of the encoding, otherwise return NULL.  The encoding of the
         * string is returned in encoding_out.
         */
        #[inline]
        pub fn get_external_string_resource_base(
            isolate: *mut Isolate,
            encoding_out: *mut String::Encoding,
        ) -> *mut ExternalStringResourceBase {
            let obj: *mut Address = unsafe {core::mem::transmute(self)};

            let type_: i32 = unsafe { Internals::get_instance_type(obj) & Internals::K_STRING_REPRESENTATION_AND_ENCODING_MASK };

            unsafe { *encoding_out = core::mem::transmute(type_ & Internals::K_STRING_ENCODING_MASK) };

            let resource: *mut ExternalStringResourceBase;

            if type_ == Internals::K_EXTERNAL_ONE_BYTE_REPRESENTATION_TAG ||
                type_ == Internals::K_EXTERNAL_TWO_BYTE_REPRESENTATION_TAG {
                    let value: *mut Address = unsafe { Internals::read_external_pointer_field::<Internals::K_EXTERNAL_STRING_RESOURCE_TAG>(isolate, obj, Internals::K_STRING_RESOURCE_OFFSET) };
                    resource = value as *mut ExternalStringResourceBase;
            } else {
                resource = self.get_external_string_resource_base_slow(unsafe { &mut *encoding_out });
            }
            #[cfg(v8_enable_checks)]
            self.verify_external_string_resource_base(unsafe{&mut *resource},unsafe{ *encoding_out });

            resource
        }
        #[inline]
        pub fn get_external_string_resource_base2(
            encoding_out: *mut String::Encoding,
        ) -> *mut ExternalStringResourceBase {
            let obj: *mut Address = unsafe {core::mem::transmute(self)};

            let type_: i32 = unsafe { Internals::get_instance_type(obj) & Internals::K_STRING_REPRESENTATION_AND_ENCODING_MASK };

            unsafe { *encoding_out = core::mem::transmute(type_ & Internals::K_STRING_ENCODING_MASK) };

            let resource: *mut ExternalStringResourceBase;

            if type_ == Internals::K_EXTERNAL_ONE_BYTE_REPRESENTATION_TAG ||
                type_ == Internals::K_EXTERNAL_TWO_BYTE_REPRESENTATION_TAG {
                    let isolate: *mut Isolate = unsafe { Internals::get_isolate_for_sandbox(obj)};
                    let value: *mut Address = unsafe { Internals::read_external_pointer_field::<Internals::K_EXTERNAL_STRING_RESOURCE_TAG>(isolate, obj, Internals::K_STRING_RESOURCE_OFFSET) };
                    resource = value as *mut ExternalStringResourceBase;
            } else {
                resource = self.get_external_string_resource_base_slow(unsafe { &mut *encoding_out });
            }
            #[cfg(v8_enable_checks)]
            self.verify_external_string_resource_base(unsafe{&mut *resource},unsafe{ *encoding_out });

            resource
        }

        /**
         * Get the ExternalStringResource for an external string.  Returns
         * NULL if IsExternal() doesn't return true.
         */
        #[inline]
        pub fn get_external_string_resource(&self) -> *mut ExternalStringResource {
            let obj: *mut Address = unsafe {core::mem::transmute(self)};
            let result: *mut ExternalStringResource;

            if unsafe { Internals::is_external_two_byte_string(Internals::get_instance_type(obj)) } {
                let isolate: *mut Isolate = unsafe { Internals::get_isolate_for_sandbox(obj) };
                let value: *mut Address = unsafe { Internals::read_external_pointer_field::<Internals::K_EXTERNAL_STRING_RESOURCE_TAG>(isolate, obj, Internals::K_STRING_RESOURCE_OFFSET) };

                result = value as *mut ExternalStringResource;
            } else {
                result = self.get_external_string_resource_slow();
            }

            #[cfg(v8_enable_checks)]
            self.verify_external_string_resource(unsafe { &mut *result });

            result
        }

        /**
         * Get the ExternalOneByteStringResource for an external one-byte string.
         * Returns NULL if IsExternalOneByte() doesn't return true.
         */
        pub fn get_external_one_byte_string_resource(&self) -> *const ExternalOneByteStringResource {
            ptr::null() // Placeholder
        }

        #[inline]
        pub fn cast(data: *mut Data) -> *mut String {
            #[cfg(v8_enable_checks)]
            Self::check_cast(data);
            data as *mut String
        }

        /**
         * Allocates a new string from a UTF-8 literal. This is equivalent to calling
         * String::NewFromUtf(isolate, "...").ToLocalChecked(), but without the check
         * overhead.
         *
         * When called on a string literal containing '\0', the inferred length is the
         * length of the input array minus 1 (for the final '\0') and not the value
         * returned by strlen.
         **/
        pub fn new_from_utf8_literal<const N: usize>(
            isolate: *mut Isolate,
            literal: &[c_char; N],
            type_: NewStringType,
        ) -> Local<'static, String> {
            assert!(N <= Self::K_MAX_LENGTH as usize);
            Self::new_from_utf8_literal2(isolate, literal.as_ptr(), type_, N - 1)
        }

        /** Allocates a new string from UTF-8 data. Only returns an empty value when
         * length > kMaxLength. **/
        pub fn new_from_utf8(
            isolate: *mut Isolate,
            data: *const c_char,
            type_: NewStringType,
            length: i32,
        ) -> Result<Local<'static, String>, StringError> {
            if length > Self::K_MAX_LENGTH {
                return Err(StringError::TooLong);
            }
            Ok(Local::empty()) // Placeholder
        }

        /** Allocates a new string from Latin-1 data.  Only returns an empty value
         * when length > kMaxLength. **/
        pub fn new_from_one_byte(
            isolate: *mut Isolate,
            data: *const u8,
            type_: NewStringType,
            length: i32,
        ) -> Result<Local<'static, String>, StringError> {
            if length > Self::K_MAX_LENGTH {
                return Err(StringError::TooLong);
            }
            Ok(Local::empty()) // Placeholder
        }

        /** Allocates a new string from UTF-16 data. Only returns an empty value when
         * length > kMaxLength. **/
        pub fn new_from_two_byte(
            isolate: *mut Isolate,
            data: *const u16,
            type_: NewStringType,
            length: i32,
        ) -> Result<Local<'static, String>, StringError> {
            if length > Self::K_MAX_LENGTH {
                return Err(StringError::TooLong);
            }
            Ok(Local::empty()) // Placeholder
        }

        /**
         * Creates a new string by concatenating the left and the right strings
         * passed in as parameters.
         */
        pub fn concat(
            isolate: *mut Isolate,
            left: Local<'static, String>,
            right: Local<'static, String>,
        ) -> Local<'static, String> {
            Local::empty() // Placeholder
        }

        /**
         * Creates a new external string using the data defined in the given
         * resource. When the external string is no longer live on V8's heap the
         * resource will be disposed by calling its Dispose method. The caller of
         * this function should not otherwise delete or modify the resource. Neither
         * should the underlying buffer be deallocated or modified except through the
         * destructor of the external string resource.
         */
        pub fn new_external_two_byte(
            isolate: *mut Isolate,
            resource: *mut ExternalStringResource,
        ) -> Result<Local<'static, String>, StringError> {
            Ok(Local::empty()) // Placeholder
        }

        /**
         * Associate an external string resource with this string by transforming it
         * in place so that existing references to this string in the JavaScript heap
         * will use the external string resource. The external string resource's
         * character contents need to be equivalent to this string.
         * Returns true if the string has been changed to be an external string.
         * The string is not modified if the operation fails. See NewExternal for
         * information on the lifetime of the resource.
         */
        #[deprecated(note = "Use the version with the isolate argument instead.")]
        pub fn make_external(&mut self, resource: *mut ExternalStringResource) -> bool {
            false // Placeholder
        }

        /**
         * Associate an external string resource with this string by transforming it
         * in place so that existing references to this string in the JavaScript heap
         * will use the external string resource. The external string resource's
         * character contents need to be equivalent to this string.
         * Returns true if the string has been changed to be an external string.
         * The string is not modified if the operation fails. See NewExternal for
         * information on the lifetime of the resource.
         */
        pub fn make_external2(&mut self, isolate: *mut Isolate, resource: *mut ExternalStringResource) -> bool {
            false // Placeholder
        }

        /**
         * Creates a new external string using the one-byte data defined in the given
         * resource. When the external string is no longer live on V8's heap the
         * resource will be disposed by calling its Dispose method. The caller of
         * this function should not otherwise delete or modify the resource. Neither
         * should the underlying buffer be deallocated or modified except through the
         * destructor of the external string resource.
         */
        pub fn new_external_one_byte(
            isolate: *mut Isolate,
            resource: *mut ExternalOneByteStringResource,
        ) -> Result<Local<'static, String>, StringError> {
            Ok(Local::empty()) // Placeholder
        }

        /**
         * Associate an external string resource with this string by transforming it
         * in place so that existing references to this string in the JavaScript heap
         * will use the external string resource. The external string resource's
         * character contents need to be equivalent to this string.
         * Returns true if the string has been changed to be an external string.
         * The string is not modified if the operation fails. See NewExternal for
         * information on the lifetime of the resource.
         */
        #[deprecated(note = "Use the version with the isolate argument instead.")]
        pub fn make_external3(&mut self, resource: *mut ExternalOneByteStringResource) -> bool {
            false // Placeholder
        }

        /**
         * Associate an external string resource with this string by transforming it
         * in place so that existing references to this string in the JavaScript heap
         * will use the external string resource. The external string resource's
         * character contents need to be equivalent to this string.
         * Returns true if the string has been changed to be an external string.
         * The string is not modified if the operation fails. See NewExternal for
         * information on the lifetime of the resource.
         */
        pub fn make_external4(&mut self, isolate: *mut Isolate, resource: *mut ExternalOneByteStringResource) -> bool {
            false // Placeholder
        }

        /**
         * Returns true if this string can be made external, given the encoding for
         * the external string resource.
         */
        pub fn can_make_external(&self, encoding: String::Encoding) -> bool {
            true // Placeholder
        }

        /**
         * Returns true if the strings values are equal. Same as JS ==/===.
         */
        pub fn string_equals(&self, str: Local<'static, String>) -> bool {
            true // Placeholder
        }

        /**
         * Converts an object to a UTF-8-encoded character array.  Useful if
         * you want to print the object.  If conversion to a string fails
         * (e.g. due to an exception in the toString() method of the object)
         * then the length() method returns 0 and the * operator returns
         * NULL.
         *
         * WARNING: This will unconditionally copy the contents of the JavaScript
         * string, and should be avoided in situations where performance is a concern.
         * Consider using WriteUtf8() instead.
         */
        pub struct Utf8Value {}

        impl Utf8Value {
            pub fn new(
                isolate: *mut Isolate,
                obj: Local<'static, v8::v8_value::Value>,
                options: String::WriteOptions,
            ) -> Self {
                Utf8Value {
                    str_: ptr::null_mut(),
                    length_: 0,
                }
            }
            pub fn operator_star(&self) -> *mut c_char {
                self.str_
            }
            pub fn operator_star_const(&self) -> *const c_char {
                self.str_
            }
            pub fn length(&self) -> usize {
                self.length_
            }

            str_: *mut c_char,
            length_: usize,
        }

        /**
         * Converts an object to a two-byte (UTF-16-encoded) string.
         *
         * If conversion to a string fails (eg. due to an exception in the toString()
         * method of the object) then the length() method returns 0 and the * operator
         * returns NULL.
         *
         * WARNING: This will unconditionally copy the contents of the JavaScript
         * string, and should be avoided in situations where performance is a concern.
         */
        pub struct Value {}

        impl Value {
            #[deprecated(
                note = "Prefer using String::ValueView if you can, or string->Write to a buffer if you cannot."
            )]
            pub fn new(isolate: *mut Isolate, obj: Local<'static, v8::v8_value::Value>) -> Self {
                Value {
                    str_: ptr::null_mut(),
                    length_: 0,
                }
            }
            pub fn operator_
