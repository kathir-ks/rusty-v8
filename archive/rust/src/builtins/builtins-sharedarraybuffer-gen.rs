// NOTE: This is a partial conversion. Many parts rely on V8 internals that
// don't have direct equivalents in standard Rust. This code provides the
// structure, but some parts will need significant adaptation or mocking.

// src/builtins/builtins-utils-gen.h (no direct equivalent - functionality integrated where used)
// src/builtins/builtins.h (no direct equivalent - functionality integrated where used)
// src/codegen/code-stub-assembler-inl.h (functionality replaced with standard Rust)
// src/objects/objects.h (basic structures defined inline, complex objects need dedicated crates or hand-rolled implementations)

// Assuming these element kinds map to Rust types
const INT8_ELEMENTS: i32 = 0;
const UINT8_ELEMENTS: i32 = 1;
const INT16_ELEMENTS: i32 = 2;
const UINT16_ELEMENTS: i32 = 3;
const INT32_ELEMENTS: i32 = 4;
const UINT32_ELEMENTS: i32 = 5;
const BIGINT64_ELEMENTS: i32 = 6;
const BIGUINT64_ELEMENTS: i32 = 7;
const FLOAT16_ELEMENTS: i32 = 8;
const FLOAT32_ELEMENTS: i32 = 9;
const FLOAT64_ELEMENTS: i32 = 10;
const UINT8_CLAMPED_ELEMENTS: i32 = 11;

const FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND: i32 = 0;
const LAST_FIXED_TYPED_ARRAY_ELEMENTS_KIND: i32 = 7; // Assuming this maps to the last BigUint64
const FIRST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND: i32 = 0; // INT8_ELEMENTS
const LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND: i32 = 7;

// Placeholder types
type Object = usize; // Representing a V8 object
type JSAny = usize; // Representing a V8 JSAny
type Context = usize; // Representing a V8 Context
type Int32T = i32;
type UintPtrT = usize;
type RawPtrT = *mut u8;
type Word32T = u32;
type JSTypedArray = usize; // Replace with a proper struct representing a JS Typed Array if possible.
type JSArrayBuffer = usize; // Replace with a proper struct representing a JS Array Buffer if possible.
type Map = usize;
type Number = f64; // Assuming Number is a double-precision float
type BigInt = i64; //placeholder
type AtomicInt64 = i64; //placeholder
type AtomicUint64 = u64; //placeholder
type MessageTemplate = u32;
type MachineRepresentation = u32;

// Placeholder constants for runtime function IDs
mod Runtime {
    pub const kAtomicsLoadSharedStructOrArray: u32 = 1;
    pub const kAtomicsStoreSharedStructOrArray: u32 = 2;
    pub const kAtomicsExchangeSharedStructOrArray: u32 = 3;
    pub const kAtomicsCompareExchangeSharedStructOrArray: u32 = 4;
    pub const kAtomicsAdd: u32 = 5;
    pub const kAtomicsSub: u32 = 6;
    pub const kAtomicsAnd: u32 = 7;
    pub const kAtomicsOr: u32 = 8;
    pub const kAtomicsXor: u32 = 9;
    pub const kAtomicsExchange: u32 = 10;
    pub const kAtomicsCompareExchange: u32 = 11;
}

mod Descriptor {
    pub const kArrayOrSharedObject: usize = 0;
    pub const kIndexOrFieldName: usize = 1;
    pub const kValue: usize = 2;
    pub const kContext: usize = 3;
    // Add other descriptor constants if needed
}

struct SharedArrayBufferBuiltinsAssembler {}

impl SharedArrayBufferBuiltinsAssembler {
    fn new() -> Self {
        SharedArrayBufferBuiltinsAssembler {}
    }

    // type AssemblerFunction = fn(MachineType, RawPtrT, UintPtrT, Word32T) -> Word32T;
    // type AssemblerFunction64<Type> = fn(RawPtrT, UintPtrT, UintPtrT, UintPtrT) -> Type;

    fn validate_integer_typed_array(
        &self,
        maybe_array_or_shared_object: Object,
        context: Context,
        out_elements_kind: &mut Int32T,
        out_backing_store: &mut RawPtrT,
        detached: &mut bool, //Label equivalent
        is_shared_struct_or_shared_array: &mut bool, //Option<Label> equivalent
    ) {
        if is_tagged_smi(maybe_array_or_shared_object) {
            *detached = true; //Goto(&invalid);
            return;
        }

        let map = load_map(maybe_array_or_shared_object);
        if !is_js_typed_array_map(map) {
            *detached = true; //Goto(&invalid);
            return;
        }
        let array: JSTypedArray = maybe_array_or_shared_object;

        if is_js_array_buffer_view_detached_or_out_of_bounds_boolean(array) {
            *detached = true; //Goto(detached);
            return;
        }

        let elements_kind = get_non_rab_gsab_elements_kind(load_map_elements_kind(map));

        if elements_kind < FIRST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND
            || elements_kind > LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND
        {
            if is_shared_struct_or_shared_array {
                if is_js_shared_struct(maybe_array_or_shared_object) {
                    *is_shared_struct_or_shared_array = true;
                    return;
                }
                if is_js_shared_array(maybe_array_or_shared_object) {
                    *is_shared_struct_or_shared_array = true;
                    return;
                }
            }
            //ThrowTypeError(context, MessageTemplate::kNotIntegerTypedArray, maybe_array_or_shared_object);
            *detached = true; //replace with an error.
            return;
        }

        *out_elements_kind = elements_kind;

        let array_buffer = get_typed_array_buffer(context, array);
        let backing_store = load_js_array_buffer_backing_store_ptr(array_buffer);
        let byte_offset = load_js_array_buffer_view_byte_offset(array);
        *out_backing_store = raw_ptr_add(backing_store, byte_offset as isize);
    }

    // https://tc39.github.io/ecma262/#sec-validateatomicaccess
    // ValidateAtomicAccess( typedArray, requestIndex )
    fn validate_atomic_access(
        &self,
        array: JSTypedArray,
        index: JSAny,
        context: Context,
    ) -> Result<UintPtrT, ()> {
        let array_length = load_js_typed_array_length_and_check_detached(array).ok_or(())?; //&unreachable
        let index_uintptr = to_index(context, index).ok_or(())?; //&range_error

        if index_uintptr < array_length {
            Ok(index_uintptr)
        } else {
            Err(()) //ThrowRangeError(context, MessageTemplate::kInvalidAtomicAccessIndex);
        }
    }

    fn debug_check_atomic_index(&self, _array: JSTypedArray, _index: UintPtrT) {
        //Implementation ommitted due to dependency on DEBUG flag and internal v8 calls
    }

    fn big_int_from_signed64(&self, signed64: AtomicInt64) -> BigInt {
        signed64 as BigInt //implementation omitted due to dependency on V8_HOST_ARCH_32_BIT flag
    }

    fn big_int_from_unsigned64(&self, unsigned64: AtomicUint64) -> BigInt {
        unsigned64 as BigInt //implementation omitted due to dependency on V8_HOST_ARCH_32_BIT flag
    }

    // https://tc39.es/ecma262/#sec-atomicload
    fn atomics_load(
        &self,
        maybe_array_or_shared_object: Object,
        index_or_field_name: JSAny,
        context: Context,
    ) -> Result<Object, ()> {
        let mut detached_or_out_of_bounds = false;
        let mut is_shared_struct_or_shared_array = false;
        let mut elements_kind: Int32T = 0;
        let mut backing_store: RawPtrT = std::ptr::null_mut();

        self.validate_integer_typed_array(
            maybe_array_or_shared_object,
            context,
            &mut elements_kind,
            &mut backing_store,
            &mut detached_or_out_of_bounds,
            &mut is_shared_struct_or_shared_array,
        );

        if detached_or_out_of_bounds {
            return Err(()); //detatched_or_out_of_bounds label replacement, throws error
        }

        let array: JSTypedArray = maybe_array_or_shared_object;

        let index_word = self
            .validate_atomic_access(array, index_or_field_name, context)
            .map_err(|_| ())?;

        if is_detached_buffer(load_js_array_buffer_view_buffer(array)) {
            //CheckJSTypedArrayIndex(array, index_word, &detached_or_out_of_bounds);
            return Err(()); //detatched_or_out_of_bounds label replacement, throws error
        }
        // Steps 5-10.

        match elements_kind {
            INT8_ELEMENTS => {
                let value = atomic_load::<i8>(backing_store, index_word);
                Ok(smi_from_int32(value as i32) as Object)
            }
            UINT8_ELEMENTS => {
                let value = atomic_load::<u8>(backing_store, index_word);
                Ok(smi_from_int32(value as i32) as Object)
            }
            INT16_ELEMENTS => {
                let value = atomic_load::<i16>(backing_store, index_word * 2);
                Ok(smi_from_int32(value as i32) as Object)
            }
            UINT16_ELEMENTS => {
                let value = atomic_load::<u16>(backing_store, index_word * 2);
                Ok(smi_from_int32(value as i32) as Object)
            }
            INT32_ELEMENTS => {
                let value = atomic_load::<i32>(backing_store, index_word * 4);
                Ok(change_int32_to_tagged(value) as Object)
            }
            UINT32_ELEMENTS => {
                let value = atomic_load::<u32>(backing_store, index_word * 4);
                Ok(change_uint32_to_tagged(value) as Object)
            }
            BIGINT64_ELEMENTS => {
                let value = atomic_load64::<AtomicInt64>(backing_store, index_word * 8);
                Ok(self.big_int_from_signed64(value) as Object)
            }
            BIGUINT64_ELEMENTS => {
                let value = atomic_load64::<AtomicUint64>(backing_store, index_word * 8);
                Ok(self.big_int_from_unsigned64(value) as Object)
            }
            _ => {
                //Unreachable();
                Err(())
            }
        }
    }

    // https://tc39.es/ecma262/#sec-atomics.store
    fn atomics_store(
        &self,
        maybe_array_or_shared_object: Object,
        index_or_field_name: JSAny,
        value: JSAny,
        context: Context,
    ) -> Result<Object, ()> {
        let mut detached_or_out_of_bounds = false;
        let mut is_shared_struct_or_shared_array = false;

        let mut elements_kind: Int32T = 0;
        let mut backing_store: RawPtrT = std::ptr::null_mut();

        self.validate_integer_typed_array(
            maybe_array_or_shared_object,
            context,
            &mut elements_kind,
            &mut backing_store,
            &mut detached_or_out_of_bounds,
            &mut is_shared_struct_or_shared_array,
        );

        if detached_or_out_of_bounds {
            return Err(());
        }

        let array: JSTypedArray = maybe_array_or_shared_object;

        let index_word = self
            .validate_atomic_access(array, index_or_field_name, context)
            .map_err(|_| ())?;

        if elements_kind > INT32_ELEMENTS {
            //BIGINT64_ELEMENTS || BIGUINT64_ELEMENTS
            let value_bigint = to_bigint(context, value).map_err(|_| ())?;

            if is_detached_buffer(load_js_array_buffer_view_buffer(array)) {
                //CheckJSTypedArrayIndex(array, index_word, &detached_or_out_of_bounds);
                return Err(());
            }

            self.debug_check_atomic_index(array, index_word);

            let (low, high) = big_int_to_raw_bytes(value_bigint);

            if is_64_bit() {
                atomic_store64(backing_store, index_word * 8, low, high);
            } else {
                //If not 64bit
                atomic_store64(backing_store, index_word * 8, low, high);
            }

            return Ok(value_bigint as Object);
        }

        let value_integer = to_integer_inline(context, value).map_err(|_| ())?;

        if is_detached_buffer(load_js_array_buffer_view_buffer(array)) {
            //CheckJSTypedArrayIndex(array, index_word, &detached_or_out_of_bounds);
            return Err(());
        }

        let value_word32 = truncate_tagged_to_word32(context, value_integer);

        self.debug_check_atomic_index(array, index_word);

        match elements_kind {
            INT8_ELEMENTS | UINT8_ELEMENTS => {
                atomic_store8(backing_store, index_word, value_word32);
            }
            INT16_ELEMENTS | UINT16_ELEMENTS => {
                atomic_store16(backing_store, index_word * 2, value_word32);
            }
            INT32_ELEMENTS | UINT32_ELEMENTS => {
                atomic_store32(backing_store, index_word * 4, value_word32);
            }
            _ => {
                return Err(());
            }
        }

        Ok(value_integer as Object)
    }

    // https://tc39.es/ecma262/#sec-atomics.exchange
    fn atomics_exchange(
        &self,
        maybe_array_or_shared_object: Object,
        index_or_field_name: JSAny,
        value: JSAny,
        context: Context,
    ) -> Result<Object, ()> {
        let mut detached_or_out_of_bounds = false;
        let mut is_shared_struct_or_shared_array = false;

        let mut elements_kind: Int32T = 0;
        let mut backing_store: RawPtrT = std::ptr::null_mut();

        self.validate_integer_typed_array(
            maybe_array_or_shared_object,
            context,
            &mut elements_kind,
            &mut backing_store,
            &mut detached_or_out_of_bounds,
            &mut is_shared_struct_or_shared_array,
        );

        if detached_or_out_of_bounds {
            return Err(());
        }

        let array: JSTypedArray = maybe_array_or_shared_object;

        let index_word = self
            .validate_atomic_access(array, index_or_field_name, context)
            .map_err(|_| ())?;

        if elements_kind > INT32_ELEMENTS {
            //BIGINT64_ELEMENTS || BIGUINT64_ELEMENTS
            let value_bigint = to_bigint(context, value).map_err(|_| ())?;

            if is_detached_buffer(load_js_array_buffer_view_buffer(array)) {
                //CheckJSTypedArrayIndex(array, index_word, &detached_or_out_of_bounds);
                return Err(());
            }

            self.debug_check_atomic_index(array, index_word);

            let (low, high) = big_int_to_raw_bytes(value_bigint);
            if elements_kind == BIGINT64_ELEMENTS {
                let old_value = atomic_exchange64::<AtomicInt64>(backing_store, index_word * 8, low, high);
                return Ok(self.big_int_from_signed64(old_value) as Object);
            } else if elements_kind == BIGUINT64_ELEMENTS {
                let old_value = atomic_exchange64::<AtomicUint64>(backing_store, index_word * 8, low, high);
                return Ok(self.big_int_from_unsigned64(old_value) as Object);
            } else {
                return Err(());
            }
        }

        let value_integer = to_integer_inline(context, value).map_err(|_| ())?;

        if is_detached_buffer(load_js_array_buffer_view_buffer(array)) {
            //CheckJSTypedArrayIndex(array, index_word, &detached_or_out_of_bounds);
            return Err(());
        }

        let value_word32 = truncate_tagged_to_word32(context, value_integer);

        self.debug_check_atomic_index(array, index_word);

        match elements_kind {
            INT8_ELEMENTS => {
                let old_value = atomic_exchange8::<i8>(backing_store, index_word, value_word32);
                Ok(smi_from_int32(old_value as i32) as Object)
            }
            UINT8_ELEMENTS => {
                let old_value = atomic_exchange8::<u8>(backing_store, index_word, value_word32);
                Ok(smi_from_int32(old_value as i32) as Object)
            }
            INT16_ELEMENTS => {
                let old_value = atomic_exchange16::<i16>(backing_store, index_word * 2, value_word32);
                Ok(smi_from_int32(old_value as i32) as Object)
            }
            UINT16_ELEMENTS => {
                let old_value = atomic_exchange16::<u16>(backing_store, index_word * 2, value_word32);
                Ok(smi_from_int32(old_value as i32) as Object)
            }
            INT32_ELEMENTS => {
                let old_value = atomic_exchange32::<i32>(backing_store, index_word * 4, value_word32);
                Ok(change_int32_to_tagged(old_value) as Object)
            }
            UINT32_ELEMENTS => {
                let old_value = atomic_exchange32::<u32>(backing_store, index_word * 4, value_word32);
                Ok(change_uint32_to_tagged(old_value) as Object)
            }
            _ => {
                return Err(());
            }
        }
    }

    // https://tc39.es/ecma262/#sec-atomics.compareexchange
    fn atomics_compare_exchange(
        &self,
        maybe_array_or_shared_object: Object,
        index_or_field_name: JSAny,
        old_value: JSAny,
        new_value: JSAny,
        context: Context,
    ) -> Result<Object, ()> {
        let mut detached_or_out_of_bounds = false;
        let mut is_shared_struct_or_shared_array = false;

        let mut elements_kind: Int32T = 0;
        let mut backing_store: RawPtrT = std::ptr::null_mut();

        self.validate_integer_typed_array(
            maybe_array_or_shared_object,
            context,
            &mut elements_kind,
            &mut backing_store,
            &mut detached_or_out_of_bounds,
            &mut is_shared_struct_or_shared_array,
        );

        if detached_or_out_of_bounds {
            return Err(());
        }

        let array: JSTypedArray = maybe_array_or_shared_object;

        let index_word = self
            .validate_atomic_access(array, index_or_field_name, context)
            .map_err(|_| ())?;

        if elements_kind > INT32_ELEMENTS {
            //BIGINT64_ELEMENTS || BIGUINT64_ELEMENTS
            let old_value_bigint = to_bigint(context, old_value).map_err(|_| ())?;
            let new_value_bigint = to_bigint(context, new_value).map_err(|_| ())?;

            if is_detached_buffer(load_js_array_buffer_view_buffer(array)) {
                //CheckJSTypedArrayIndex(array, index_word, &detached_or_out_of_bounds);
                return Err(());
            }

            self.debug_check_atomic_index(array, index_word);

            let (old_low, old_high) = big_int_to_raw_bytes(old_value_bigint);
            let (new_low, new_high) = big_int_to_raw_bytes(new_value_bigint);
            if elements_kind == BIGINT64_ELEMENTS {
                let actual_value = atomic_compare_exchange64::<AtomicInt64>(backing_store, index_word * 8, old_low, new_low, old_high, new_high);
                return Ok(self.big_int_from_signed64(actual_value) as Object);
            } else if elements_kind == BIGUINT64_ELEMENTS {
                let actual_value = atomic_compare_exchange64::<AtomicUint64>(backing_store, index_word * 8, old_low, new_low, old_high, new_high);
                return Ok(self.big_int_from_unsigned64(actual_value) as Object);
            } else {
                return Err(());
            }
        }

        let old_value_integer = to_integer_inline(context, old_value).map_err(|_| ())?;
        let new_value_integer = to_integer_inline(context, new_value).map_err(|_| ())?;

        if is_detached_buffer(load_js_array_buffer_view_buffer(array)) {
            //CheckJSTypedArrayIndex(array, index_word, &detached_or_out_of_bounds);
            return Err(());
        }

        let old_value_word32 = truncate_tagged_to_word32(context, old_value_integer);
        let new_value_word32 = truncate_tagged_to_word32(context, new_value_integer);

        self.debug_check_atomic_index(array, index_word);

        match elements_kind {
            INT8_ELEMENTS => {
                let actual_value = atomic_compare_exchange8::<i8>(backing_store, index_word, old_value_word32, new_value_word32);
                Ok(smi_from_int32(actual_value as i32) as Object)
            }
            UINT8_ELEMENTS => {
                let actual_value = atomic_compare_exchange8::<u8>(backing_store, index_word, old_value_word32, new_value_word32);
                Ok(smi_from_int32(actual_value as i32) as Object)
            }
            INT16_ELEMENTS => {
                let actual_value = atomic_compare_exchange16::<i16>(backing_store, index_word * 2, old_value_word32, new_value_word32);
                Ok(smi_from_int32(actual_value as i32) as Object)
            }
            UINT16_ELEMENTS => {
                let actual_value = atomic_compare_exchange16::<u16>(backing_store, index_word * 2, old_value_word32, new_value_word32);
                Ok(smi_from_int32(actual_value as i32) as Object)
            }
            INT32_ELEMENTS => {
                let actual_value = atomic_compare_exchange32::<i32>(backing_store, index_word * 4, old_value_word32, new_value_word32);
                Ok(change_int32_to_tagged(actual_value) as Object)
            }
            UINT32_ELEMENTS => {
                let actual_value = atomic_compare_exchange32::<u32>(backing_store, index_word * 4, old_value_word32, new_value_word32);
                Ok(change_uint32_to_tagged(actual_value) as Object)
            }
            _ => {
                return Err(());
            }
        }
    }

    fn atomic_binop_builtin_common(
        &self,
        maybe_array: Object,
        index: JSAny,
        value: JSAny,
        context: Context,
        operation: BinOp,
        method_name: &str,
    ) -> Result<Object, ()> {
        let mut detached_or_out_of_bounds = false;
        let mut elements_kind: Int32T = 0;
        let mut backing_store: RawPtrT = std::ptr::null_mut();
        self.validate_integer_typed_array(
            maybe_array,
            context,
            &mut elements_kind,
            &mut backing_store,
            &mut detached_or_out_of_bounds,
            &mut false, //&detached_or_out_of_bounds,
        );
        if detached_or_out_of_bounds {
            //ThrowTypeError(context, MessageTemplate::kDetachedOperation, method_name);
            return Err(());
        }
        let array: JSTypedArray = maybe_array;
        let index_word = self.validate_atomic_access(array, index, context)?;

        if elements_kind > INT32_ELEMENTS {
            // BigInt
            let value_bigint = to_bigint(context, value).map_err(|_| ())?;
            if is_detached_buffer(load_js_array_buffer_view_buffer(array)) {
                //CheckJSTypedArrayIndex(array, index_word, &detached_or_out_of_bounds);
                return Err(());
            }
            self.debug_check_atomic_index(array, index_word);
            let (value_low, value_high) = big_int_to_raw_bytes(value_bigint);

            if elements_kind == BIGINT64_ELEMENTS {
                let old_value = match operation {
                    BinOp::Add => atomic_add64::<AtomicInt64>(backing_store, index_word * 8, value_low, value_high),
                    BinOp::Sub => atomic_sub64::<AtomicInt64>(backing_store, index_word * 8, value_low, value_high),
                    BinOp::And => atomic_and64::<AtomicInt64>(backing_store, index_word * 8, value_low, value_high),
                    BinOp::Or => atomic_or64::<AtomicInt64>(backing_store, index_word * 8, value_low, value_high),
                    BinOp::Xor => atomic_xor64::<AtomicInt64>(backing_store, index_word * 8, value_low, value_high),
                };
                return Ok(self.big_int_from_signed64(old_value) as Object);
            } else if elements_kind == BIGUINT64_ELEMENTS {
                 let old_value = match operation {
                    BinOp::Add => atomic_add64::<AtomicUint64>(backing_store, index_word * 8, value_low, value_high),
                    BinOp::Sub => atomic_sub64::<AtomicUint64>(backing_store, index_word * 8, value_low, value_high),
                    BinOp::And => atomic_and64::<AtomicUint64>(backing_store, index_word * 8, value_low, value_high),
                    BinOp::Or => atomic_or64::<AtomicUint64>(backing_store, index_word * 8, value_low, value_high),
                    BinOp::Xor => atomic_xor64::<AtomicUint64>(backing_store, index_word * 8, value_low, value_high),
                };
                return Ok(self.big_int_from_unsigned64(old_value) as Object);
            } else {
                return Err(());
            }
        }

        // Integer
        let value_integer = to_integer_inline(context, value).map_err(|_| ())?;
        if is_detached_buffer(load_js_array_buffer_view_buffer(array)) {
            //CheckJSTypedArrayIndex(array, index_word, &detached_or_out_of_bounds);
            return Err(());
        }
        self.debug_check_atomic_index(array, index_word);
        let value_word32 = truncate_tagged_to_word32(context, value_integer);

        match elements_kind {
            INT8_ELEMENTS => {
                let old_value = match operation {
                    BinOp::Add => atomic_add8::<i8>(backing_store, index_word, value_word32),
                    BinOp::Sub => atomic_sub8::<i8>(backing_store, index_word, value_word32),
                    BinOp::And => atomic_and8::<i8>(backing_store, index_word, value_word32),
                    BinOp::Or => atomic_or8::<i8>(backing_store, index_word, value_word32),
                    BinOp::Xor => atomic_xor8::<i8>(backing_store, index_word, value_word32),
                };
                Ok(smi_from_int32(old_value as i32) as Object)
            }
            UINT8_ELEMENTS => {
                 let old_value = match operation {
                    BinOp::Add => atomic_add8::<u8>(backing_store, index_word, value_word32),
                    BinOp::Sub => atomic_sub8::<u8>(backing_store, index_word, value_word32),
                    BinOp::And => atomic_and8::<u8>(backing_store, index_word, value_word32),
                    BinOp::Or => atomic_or8::<u8>(backing_store, index_word, value_word32),
                    BinOp::Xor => atomic_xor8::<u8>(backing_store, index_word, value_word32),
                };
                Ok(smi_from_int32(old_value as i32) as Object)
            }
            INT16_ELEMENTS => {
                 let old_value = match operation {
                    BinOp::Add => atomic_add16::<i16>(backing_store, index_word * 2, value_word32),
                    BinOp::Sub => atomic_sub16::<i16>(backing_store, index_word * 2, value_word32),
                    BinOp::And => atomic_and16::<i16>(backing_store, index_word * 2, value_word32),
                    BinOp::Or => atomic_or16::<i16>(backing_store, index_word * 2, value_word32),
                    BinOp::Xor => atomic_xor16::<i16>(backing_store, index_word * 2, value_word32),
                };
                Ok(smi_from_int32(old_value as i32) as Object)
            }
            UINT16_ELEMENTS => {
                let old_value = match operation {
                    BinOp::Add => atomic_add16::<u16>(backing_store, index_word * 2, value_word32),
                    BinOp::