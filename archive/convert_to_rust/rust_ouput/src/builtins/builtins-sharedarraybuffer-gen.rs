// Converted from V8 C++ source files:
// Header: N/A
// Implementation: builtins-sharedarraybuffer-gen.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;
use crate::JSArrayBuffer;
use crate::instruction;

struct SharedArrayBufferBuiltinsAssembler {
    assembler: CodeStubAssembler,
}

impl SharedArrayBufferBuiltinsAssembler {
    fn new(state: &mut compiler::CodeAssemblerState) -> Self {
        SharedArrayBufferBuiltinsAssembler {
            assembler: CodeStubAssembler::new(state),
        }
    }

    fn ValidateIntegerTypedArray(
        &mut self,
        maybe_array_or_shared_object: TNode<Object>,
        context: TNode<Context>,
        out_elements_kind: &mut TNode<Int32T>,
        out_backing_store: &mut TNode<RawPtrT>,
        detached: &mut Label,
        is_shared_struct_or_shared_array: Option<&mut Label>,
    ) {
        let mut not_float_or_clamped = Label::new("not_float_or_clamped");
        let mut invalid = Label::new("invalid");

        // The logic of TypedArrayBuiltinsAssembler::ValidateTypedArrayBuffer is
        // inlined to avoid duplicate error branches.

        // Fail if it is not a heap object.
        self.assembler.GotoIf(self.assembler.TaggedIsSmi(maybe_array_or_shared_object), &invalid);

        // Fail if the array's instance type is not JSTypedArray.
        let map = self.assembler.LoadMap(self.assembler.CAST(maybe_array_or_shared_object));
        self.assembler.GotoIfNot(self.assembler.IsJSTypedArrayMap(map), &invalid);
        let array: TNode<JSTypedArray> = self.assembler.CAST(maybe_array_or_shared_object);

        // Fail if the array's JSArrayBuffer is detached / out of bounds.
        self.assembler.GotoIf(self.assembler.IsJSArrayBufferViewDetachedOrOutOfBoundsBoolean(array), detached);

        // Fail if the array's element type is float16, float32, float64 or clamped.

        // clang-format off
        assert!(INT8_ELEMENTS >= FIRST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND &&
                INT8_ELEMENTS <= LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND);
        assert!(INT16_ELEMENTS >= FIRST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND &&
                INT16_ELEMENTS <= LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND);
        assert!(INT32_ELEMENTS >= FIRST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND &&
                INT32_ELEMENTS <= LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND);
        assert!(BIGINT64_ELEMENTS >= FIRST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND &&
                BIGINT64_ELEMENTS <= LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND);
        assert!(UINT8_ELEMENTS >= FIRST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND &&
                UINT8_ELEMENTS <= LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND);
        assert!(UINT16_ELEMENTS >= FIRST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND &&
                UINT16_ELEMENTS <= LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND);
        assert!(UINT32_ELEMENTS >= FIRST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND &&
                UINT32_ELEMENTS <= LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND);
        assert!(BIGUINT64_ELEMENTS >= FIRST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND &&
                BIGUINT64_ELEMENTS <= LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND);
        assert!(FLOAT16_ELEMENTS >= LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND);
        assert!(FLOAT32_ELEMENTS >= LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND);
        assert!(FLOAT64_ELEMENTS >= LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND);
        assert!(UINT8_CLAMPED_ELEMENTS >= LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND);
        // clang-format on

        *out_elements_kind = self.assembler.GetNonRabGsabElementsKind(self.assembler.LoadMapElementsKind(map));
        self.assembler.CSA_DCHECK(self, self.assembler.Int32GreaterThanOrEqual(
            *out_elements_kind,
            self.assembler.Int32Constant(FIRST_FIXED_TYPED_ARRAY_ELEMENTS_KIND)));
        self.assembler.CSA_DCHECK(self, self.assembler.Int32LessThanOrEqual(
            *out_elements_kind,
            self.assembler.Int32Constant(LAST_FIXED_TYPED_ARRAY_ELEMENTS_KIND)));
        self.assembler.CSA_DCHECK(
            self,
            self.assembler.Int32GreaterThanOrEqual(
                *out_elements_kind,
                self.assembler.Int32Constant(FIRST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND)));
        self.assembler.Branch(self.assembler.Int32LessThanOrEqual(
            *out_elements_kind,
            self.assembler.Int32Constant(LAST_VALID_ATOMICS_TYPED_ARRAY_ELEMENTS_KIND)),
                              &mut not_float_or_clamped, &mut invalid);

        self.assembler.BIND(&mut invalid);
        {
            if let Some(is_shared_struct_or_shared_array) = is_shared_struct_or_shared_array {
                self.assembler.GotoIf(self.assembler.IsJSSharedStruct(maybe_array_or_shared_object), is_shared_struct_or_shared_array);
                self.assembler.GotoIf(self.assembler.IsJSSharedArray(maybe_array_or_shared_object), is_shared_struct_or_shared_array);
            }
            self.assembler.ThrowTypeError(context, MessageTemplate::kNotIntegerTypedArray, maybe_array_or_shared_object);
        }

        self.assembler.BIND(&mut not_float_or_clamped);
        let array_buffer = self.assembler.GetTypedArrayBuffer(context, array);
        let backing_store = self.assembler.LoadJSArrayBufferBackingStorePtr(array_buffer);
        let byte_offset = self.assembler.LoadJSArrayBufferViewByteOffset(array);
        *out_backing_store = self.assembler.RawPtrAdd(backing_store, self.assembler.Signed(byte_offset));
    }

    // https://tc39.github.io/ecma262/#sec-validateatomicaccess
    // ValidateAtomicAccess( typedArray, requestIndex )
    fn ValidateAtomicAccess(
        &mut self,
        array: TNode<JSTypedArray>,
        index: TNode<JSAny>,
        context: TNode<Context>,
    ) -> TNode<UintPtrT> {
        let mut done = Label::new("done");
        let mut range_error = Label::new("range_error");
        let mut unreachable = Label::new("unreachable");

        // 1. Assert: typedArray is an Object that has a [[ViewedArrayBuffer]]
        // internal slot.
        // 2. Let length be IntegerIndexedObjectLength(typedArray);
        let array_length =
            self.assembler.LoadJSTypedArrayLengthAndCheckDetached(array, &mut unreachable);

        // 3. Let accessIndex be ? ToIndex(requestIndex).
        let index_uintptr = self.assembler.ToIndex(context, index, &mut range_error);

        // 4. Assert: accessIndex ≥ 0.
        // 5. If accessIndex ≥ length, throw a RangeError exception.
        self.assembler.Branch(self.assembler.UintPtrLessThan(index_uintptr, array_length), &mut done, &mut range_error);

        self.assembler.BIND(&mut unreachable);
        // This should not happen, since we've just called ValidateIntegerTypedArray.
        self.assembler.Unreachable();

        self.assembler.BIND(&mut range_error);
        self.assembler.ThrowRangeError(context, MessageTemplate::kInvalidAtomicAccessIndex);

        // 6. Return accessIndex.
        self.assembler.BIND(&mut done);
        index_uintptr
    }

    fn DebugCheckAtomicIndex(&mut self, array: TNode<JSTypedArray>, index: TNode<UintPtrT>) {
        #[cfg(debug_assertions)]
        {
            // In Debug mode, we re-validate the index as a sanity check because ToInteger
            // above calls out to JavaScript. Atomics work on ArrayBuffers, which may be
            // detached, and detachment state must be checked and throw before this
            // check. Moreover, resizable ArrayBuffers can be shrunk.
            //
            // This function must always be called after ValidateIntegerTypedArray, which
            // will ensure that LoadJSArrayBufferViewBuffer will not be null.
            let mut detached_or_out_of_bounds = Label::new("detached_or_out_of_bounds");
            let mut end = Label::new("end");
            self.assembler.CSA_DCHECK(self, self.assembler.Word32BinaryNot(
                self.assembler.IsDetachedBuffer(self.assembler.LoadJSArrayBufferViewBuffer(array))));

            self.assembler.CSA_DCHECK(self,
                                       self.assembler.UintPtrLessThan(index, self.assembler.LoadJSTypedArrayLengthAndCheckDetached(array, &mut detached_or_out_of_bounds)));
            self.assembler.Goto(&mut end);

            self.assembler.BIND(&mut detached_or_out_of_bounds);
            self.assembler.Unreachable();

            self.assembler.BIND(&mut end);
        }
    }

    fn BigIntFromSigned64(&mut self, signed64: TNode<AtomicInt64>) -> TNode<BigInt> {
        #[cfg(target_arch = "x86_64")]
        {
            self.assembler.BigIntFromInt64(signed64)
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            let low = self.assembler.Projection::<0>(signed64);
            let high = self.assembler.Projection::<1>(signed64);
            self.assembler.BigIntFromInt32Pair(low, high)
        }
    }

    fn BigIntFromUnsigned64(&mut self, unsigned64: TNode<AtomicUint64>) -> TNode<BigInt> {
        #[cfg(target_arch = "x86_64")]
        {
            self.assembler.BigIntFromUint64(unsigned64)
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            let low = self.assembler.Projection::<0>(unsigned64);
            let high = self.assembler.Projection::<1>(unsigned64);
            self.assembler.BigIntFromUint32Pair(low, high)
        }
    }
    
    fn AtomicsLoad(
        &mut self,
        maybe_array_or_shared_object: TNode<Object>,
        index_or_field_name: TNode<JSAny>,
        context: TNode<Context>
    ) {
        let mut detached_or_out_of_bounds = Label::new("detached_or_out_of_bounds");
        let mut is_shared_struct_or_shared_array = Label::new("is_shared_struct_or_shared_array");
        let mut elements_kind: TNode<Int32T> = TNode::<Int32T>::default();
        let mut backing_store: TNode<RawPtrT> = TNode::<RawPtrT>::default();

        self.ValidateIntegerTypedArray(maybe_array_or_shared_object, context, &mut elements_kind, &mut backing_store, &mut detached_or_out_of_bounds, Some(&mut is_shared_struct_or_shared_array));
        
        let array: TNode<JSTypedArray> = self.assembler.CAST(maybe_array_or_shared_object);
    
        let index_word: TNode<UintPtrT> = self.ValidateAtomicAccess(array, index_or_field_name, context);
    
        self.assembler.CheckJSTypedArrayIndex(array, index_word, &mut detached_or_out_of_bounds);
    
        let mut i8 = Label::new("i8");
        let mut u8 = Label::new("u8");
        let mut i16 = Label::new("i16");
        let mut u16 = Label::new("u16");
        let mut i32 = Label::new("i32");
        let mut u32 = Label::new("u32");
        let mut i64 = Label::new("i64");
        let mut u64 = Label::new("u64");
        let mut other = Label::new("other");
    
        let case_values: [i32; 8] = [
            INT8_ELEMENTS,  UINT8_ELEMENTS,  INT16_ELEMENTS,    UINT16_ELEMENTS,
            INT32_ELEMENTS, UINT32_ELEMENTS, BIGINT64_ELEMENTS, BIGUINT64_ELEMENTS,
        ];
        let mut case_labels: [&mut Label; 8] = [&mut i8, &mut u8, &mut i16, &mut u16, &mut i32, &mut u32, &mut i64, &mut u64];
    
        self.assembler.Switch(elements_kind, &mut other, &case_values, &mut case_labels, case_labels.len());
    
        self.assembler.BIND(&mut i8);
        self.assembler.Return(self.assembler.SmiFromInt32(self.assembler.AtomicLoad::<Int8T>(AtomicMemoryOrder::kSeqCst, backing_store, index_word)));
    
        self.assembler.BIND(&mut u8);
        self.assembler.Return(self.assembler.SmiFromInt32(self.assembler.AtomicLoad::<Uint8T>(AtomicMemoryOrder::kSeqCst, backing_store, index_word)));
    
        self.assembler.BIND(&mut i16);
        self.assembler.Return(self.assembler.SmiFromInt32(self.assembler.AtomicLoad::<Int16T>(AtomicMemoryOrder::kSeqCst, backing_store, self.assembler.WordShl(index_word, 1))));
    
        self.assembler.BIND(&mut u16);
        self.assembler.Return(self.assembler.SmiFromInt32(self.assembler.AtomicLoad::<Uint16T>(AtomicMemoryOrder::kSeqCst, backing_store, self.assembler.WordShl(index_word, 1))));
    
        self.assembler.BIND(&mut i32);
        self.assembler.Return(self.assembler.ChangeInt32ToTagged(self.assembler.AtomicLoad::<Int32T>(AtomicMemoryOrder::kSeqCst, backing_store, self.assembler.WordShl(index_word, 2))));
    
        self.assembler.BIND(&mut u32);
        self.assembler.Return(self.assembler.ChangeUint32ToTagged(self.assembler.AtomicLoad::<Uint32T>(AtomicMemoryOrder::kSeqCst, backing_store, self.assembler.WordShl(index_word, 2))));
    
        self.assembler.BIND(&mut i64);
        self.assembler.Return(self.BigIntFromSigned64(self.assembler.AtomicLoad64::<AtomicInt64>(AtomicMemoryOrder::kSeqCst, backing_store, self.assembler.WordShl(index_word, 3))));
    
        self.assembler.BIND(&mut u64);
        self.assembler.Return(self.BigIntFromUnsigned64(self.assembler.AtomicLoad64::<AtomicUint64>(AtomicMemoryOrder::kSeqCst, backing_store, self.assembler.WordShl(index_word, 3))));
    
        self.assembler.BIND(&mut other);
        self.assembler.Unreachable();
    
        self.assembler.BIND(&mut detached_or_out_of_bounds);
        self.assembler.ThrowTypeError(context, MessageTemplate::kDetachedOperation, "Atomics.load");
    
        self.assembler.BIND(&mut is_shared_struct_or_shared_array);
        self.assembler.Return(self.assembler.CallRuntime(Runtime::kAtomicsLoadSharedStructOrArray, context, maybe_array_or_shared_object, index_or_field_name));
    }
    
    fn AtomicsStore(
        &mut self,
        maybe_array_or_shared_object: TNode<Object>,
        index_or_field_name: TNode<JSAny>,
        value: TNode<JSAny>,
        context: TNode<Context>
    ) {
        let mut detached_or_out_of_bounds = Label::new("detached_or_out_of_bounds");
        let mut is_shared_struct_or_shared_array = Label::new("is_shared_struct_or_shared_array");
        let mut elements_kind: TNode<Int32T> = TNode::<Int32T>::default();
        let mut backing_store: TNode<RawPtrT> = TNode::<RawPtrT>::default();

        self.ValidateIntegerTypedArray(maybe_array_or_shared_object, context, &mut elements_kind, &mut backing_store, &mut detached_or_out_of_bounds, Some(&mut is_shared_struct_or_shared_array));
        let array: TNode<JSTypedArray> = self.assembler.CAST(maybe_array_or_shared_object);

        let index_word: TNode<UintPtrT> = self.ValidateAtomicAccess(array, index_or_field_name, context);

        let mut u8 = Label::new("u8");
        let mut u16 = Label::new("u16");
        let mut u32 = Label::new("u32");
        let mut u64 = Label::new("u64");
        let mut other = Label::new("other");

        if elements_kind.clone() > self.assembler.Int32Constant(INT32_ELEMENTS) {
            self.assembler.Goto(&mut u64);
        }

        let value_integer: TNode<Number> = self.assembler.ToInteger_Inline(context, value);

        self.assembler.CheckJSTypedArrayIndex(array, index_word, &mut detached_or_out_of_bounds);

        let value_word32: TNode<Word32T> = self.assembler.TruncateTaggedToWord32(context, value_integer);

        self.DebugCheckAtomicIndex(array, index_word);

        let case_values: [i32; 6] = [
            INT8_ELEMENTS,   UINT8_ELEMENTS, INT16_ELEMENTS,
            UINT16_ELEMENTS, INT32_ELEMENTS, UINT32_ELEMENTS,
        ];
        let mut case_labels: [&mut Label; 6] = [&mut u8, &mut u8, &mut u16, &mut u16, &mut u32, &mut u32];
    
        self.assembler.Switch(elements_kind, &mut other, &case_values, &mut case_labels, case_labels.len());

        self.assembler.BIND(&mut u8);
        self.assembler.AtomicStore(MachineRepresentation::kWord8, AtomicMemoryOrder::kSeqCst, backing_store, index_word, value_word32);
        self.assembler.Return(value_integer);

        self.assembler.BIND(&mut u16);
        self.assembler.AtomicStore(MachineRepresentation::kWord16, AtomicMemoryOrder::kSeqCst, backing_store, self.assembler.WordShl(index_word, 1), value_word32);
        self.assembler.Return(value_integer);

        self.assembler.BIND(&mut u32);
        self.assembler.AtomicStore(MachineRepresentation::kWord32, AtomicMemoryOrder::kSeqCst, backing_store, self.assembler.WordShl(index_word, 2), value_word32);
        self.assembler.Return(value_integer);

        self.assembler.BIND(&mut u64);
        let value_bigint: TNode<BigInt> = self.assembler.ToBigInt(context, value);

        self.assembler.CheckJSTypedArrayIndex(array, index_word, &mut detached_or_out_of_bounds);

        self.DebugCheckAtomicIndex(array, index_word);

        let mut var_low: TVARIABLE<UintPtrT> = TVARIABLE::new();
        let mut var_high: TVARIABLE<UintPtrT> = TVARIABLE::new();
        self.assembler.BigIntToRawBytes(value_bigint, &mut var_low, &mut var_high);
        let high: TNode<UintPtrT> = if self.assembler.Is64() { TNode::<UintPtrT>::default() } else { var_high.value() };
        self.assembler.AtomicStore64(AtomicMemoryOrder::kSeqCst, backing_store, self.assembler.WordShl(index_word, 3), var_low.value(), high);
        self.assembler.Return(value_bigint);

        self.assembler.BIND(&mut other);
        self.assembler.Unreachable();

        self.assembler.BIND(&mut detached_or_out_of_bounds);
        self.assembler.ThrowTypeError(context, MessageTemplate::kDetachedOperation, "Atomics.store");

        self.assembler.BIND(&mut is_shared_struct_or_shared_array);
        self.assembler.Return(self.assembler.CallRuntime(Runtime::kAtomicsStoreSharedStructOrArray, context, maybe_array_or_shared_object, index_or_field_name, value));
    }
    
    fn AtomicsExchange(
        &mut self,
        maybe_array_or_shared_object: TNode<Object>,
        index_or_field_name: TNode<JSAny>,
        value: TNode<JSAny>,
        context: TNode<Context>
    ) {
        let mut detached_or_out_of_bounds = Label::new("detached_or_out_of_bounds");
        let mut is_shared_struct_or_shared_array = Label::new("is_shared_struct_or_shared_array");
        let mut elements_kind: TNode<Int32T> = TNode::<Int32T>::default();
        let mut backing_store: TNode<RawPtrT> = TNode::<RawPtrT>::default();
    
        self.ValidateIntegerTypedArray(maybe_array_or_shared_object, context, &mut elements_kind, &mut backing_store, &mut detached_or_out_of_bounds, Some(&mut is_shared_struct_or_shared_array));
        let array: TNode<JSTypedArray> = self.assembler.CAST(maybe_array_or_shared_object);
    
        let index_word: TNode<UintPtrT> = self.ValidateAtomicAccess(array, index_or_field_name, context);
    
        #[cfg(target_arch = "mips64")]
        {
            let index_number: TNode<Number> = self.assembler.ChangeUintPtrToTagged(index_word);
            self.assembler.Return(self.assembler.CallRuntime(Runtime::kAtomicsExchange, context, array, index_number, value));
        }
    
        #[cfg(not(target_arch = "mips64"))]
        {
            let mut i8 = Label::new("i8");
            let mut u8 = Label::new("u8");
            let mut i16 = Label::new("i16");
            let mut u16 = Label::new("u16");
            let mut i32 = Label::new("i32");
            let mut u32 = Label::new("u32");
            let mut i64 = Label::new("i64");
            let mut u64 = Label::new("u64");
            let mut big = Label::new("big");
            let mut other = Label::new("other");
    
            if elements_kind.clone() > self.assembler.Int32Constant(INT32_ELEMENTS) {
                self.assembler.Goto(&mut big);
            }
    
            let value_integer: TNode<Number> = self.assembler.ToInteger_Inline(context, value);
    
            self.assembler.CheckJSTypedArrayIndex(array, index_word, &mut detached_or_out_of_bounds);
    
            self.DebugCheckAtomicIndex(array, index_word);
    
            let value_word32: TNode<Word32T> = self.assembler.TruncateTaggedToWord32(context, value_integer);
    
            let case_values: [i32; 6] = [
                INT8_ELEMENTS,   UINT8_ELEMENTS, INT16_ELEMENTS,
                UINT16_ELEMENTS, INT32_ELEMENTS, UINT32_ELEMENTS,
            ];
            let mut case_labels: [&mut Label; 6] = [
                &mut i8, &mut u8, &mut i16, &mut u16, &mut i32, &mut u32,
            ];
            self.assembler.Switch(elements_kind, &mut other, &case_values, &mut case_labels, case_labels.len());
    
            self.assembler.BIND(&mut i8);
            self.assembler.Return(self.assembler.SmiFromInt32(self.assembler.Signed(self.assembler.AtomicExchange(MachineType::Int8(), backing_store, index_word, value_word32))));
    
            self.assembler.BIND(&mut u8);
            self.assembler.Return(self.assembler.SmiFromInt32(self.assembler.Signed(self.assembler.AtomicExchange(MachineType::Uint8(), backing_store, index_word, value_word32))));
    
            self.assembler.BIND(&mut i16);
            self.assembler.Return(self.assembler.SmiFromInt32(self.assembler.Signed(self.assembler.AtomicExchange(MachineType::Int16(), backing_store, self.assembler.WordShl(index_word, UintPtrConstant(1)), value_word32))));
    
            self.assembler.BIND(&mut u16);
            self.assembler.Return(self.assembler.SmiFromInt32(self.assembler.Signed(self.assembler.AtomicExchange(MachineType::Uint16(), backing_store, self.assembler.WordShl(index_word, UintPtrConstant(1)), value_word32))));
    
            self.assembler.BIND(&mut i32);
            self.assembler.Return(self.assembler.ChangeInt32ToTagged(self.assembler.Signed(self.assembler.AtomicExchange(MachineType::Int32(), backing_store, self.assembler.WordShl(index_word, UintPtrConstant(2)), value_word32))));
    
            self.assembler.BIND(&mut u32);
            self.assembler.Return(self.assembler.ChangeUint32ToTagged(self.assembler.Unsigned(self.assembler.AtomicExchange(MachineType::Uint32(), backing_store, self.assembler.WordShl(index_word, UintPtrConstant(2)), value_word32))));
    
            self.assembler.BIND(&mut big);
            let value_bigint: TNode<BigInt> = self.assembler.ToBigInt(context, value);
    
            self.assembler.CheckJSTypedArrayIndex(array, index_word, &mut detached_or_out_of_bounds);
    
            self.DebugCheckAtomicIndex(array, index_word);
    
            let mut var_low: TVARIABLE<UintPtrT> = TVARIABLE::new();
            let mut var_high: TVARIABLE<UintPtrT> = TVARIABLE::new();
            self.assembler.BigIntToRawBytes(value_bigint, &mut var_low, &mut var_high);
            let high: TNode<UintPtrT> = if self.assembler.Is64() { TNode::<UintPtrT>::default() } else { var_high.value() };
    
            if elements_kind == self.assembler.Int32Constant(BIGINT64_ELEMENTS) {
                self.assembler.Goto(&mut i64);
            } else if elements_kind == self.assembler.Int32Constant(BIGUINT64_ELEMENTS) {
                self.assembler.Goto(&mut u64);
            } else {
                self.assembler.Unreachable();
            }
    
            self.assembler.BIND(&mut i64);
            self.assembler.Return(self.BigIntFromSigned64(self.assembler.AtomicExchange64::<AtomicInt64>(backing_store, self.assembler.WordShl(index_word, UintPtrConstant(3)), var_low.value(), high)));
    
            self.assembler.BIND(&mut u64);
            self.assembler.Return(self.BigIntFromUnsigned64(self.assembler.AtomicExchange64::<AtomicUint64>(backing_store, self.assembler.WordShl(index_word, UintPtrConstant(3)), var_low.value(), high)));
    
            self.assembler.BIND(&mut other);
            self.assembler.Unreachable();
        }
    
        self.assembler.BIND(&mut detached_or_out_of_bounds);
        self.assembler.ThrowTypeError(context, MessageTemplate::kDetachedOperation, "Atomics.exchange");
    
        self.assembler.BIND(&mut is_shared_struct_or_shared_array);
        self.assembler.Return(self.assembler.CallRuntime(Runtime::kAtomicsExchangeSharedStructOrArray, context, maybe_array_or_shared_object, index_or_field_name, value));
    }
    
    fn AtomicsCompareExchange(
        &mut self,
        maybe_array_or_shared_object: TNode<Object>,
        index_or_field_name: TNode<JSAny>,
        old_value: TNode<JSAny>,
        new_value: TNode<JSAny>,
        context: TNode<Context>
    ) {
        let mut detached_or_out_of_bounds = Label::new("detached_or_out_of_bounds");
        let mut is_shared_struct_or_shared_array = Label::new("is_shared_struct_or_shared_array");
        let mut elements_kind: TNode<Int32T> = TNode::<Int32T>::default();
        let mut backing_store: TNode<RawPtrT> = TNode::<RawPtrT>::default();
    
        self.ValidateIntegerTypedArray(maybe_array_or_shared_object, context, &mut elements_kind, &mut backing_store, &mut detached_or_out_of_bounds, Some(&mut is_shared_struct_or_shared_array));
        let array: TNode<JSTypedArray> = self.assembler.CAST(maybe_array_or_shared_object);
    
        let index_word: TNode<UintPtrT> = self.ValidateAtomicAccess(array, index_or_field_name, context);
    
        #[cfg(target_arch = "mips64")]
        {
            let index_number: TNode<Number> = self.assembler.ChangeUintPtrToTagged(index_word);
            self.assembler.Return(self.assembler.CallRuntime(Runtime::kAtomicsCompareExchange, context, array, index_number, old_value, new_value));
        }
    
        #[cfg(not(target_arch = "mips64"))]
        {
            let mut i8 = Label::new("i8");
            let mut u8 = Label::new("u8");
            let mut i16 = Label::new("i16");
            let mut u16 = Label::new("u16");
            let mut i32 = Label::new("i32");
            let mut u32 = Label::new("u32");
            let mut i64 = Label::new("i64");
            let mut u64 = Label::new("u64");
            let mut big = Label::new("big");
            let mut other = Label::new("other");
    
            if elements_kind.clone() > self.assembler.Int3
