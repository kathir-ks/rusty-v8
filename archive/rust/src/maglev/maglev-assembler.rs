// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This conversion is a direct translation of the C++ code.
// Some parts might require further adaptation to fully leverage Rust's
// features and memory management capabilities.  Specifically, the
// RegisterSnapshot and other V8-specific classes are stubbed out, and
// memory management needs to be carefully considered for production use.

mod maglev {
    use std::mem;

    //use crate::builtins::builtins_inl; // Stub
    //use crate::codegen::reglist; // Stub
    //use crate::maglev::maglev_assembler_inl; // Stub
    //use crate::maglev::maglev_code_generator; // Stub
    //use crate::numbers::conversions; // Stub

    // Placeholder structs and enums for V8 specific types. These need
    // proper definitions based on the original C++ code.
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Register {
        id: u32,
    }
    impl Register {
        pub const fn no_reg() -> Self {
            Register { id: 0 }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct DoubleRegister {
        id: u32,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct RootIndex; // Placeholder

    impl RootIndex {
        const kHeapNumberMap: Self = RootIndex;
        const kSeqTwoByteStringMap: Self = RootIndex;
        const kSingleCharacterStringTable: Self = RootIndex;
        const kFalseValue: Self = RootIndex;
        const kTrueValue: Self = RootIndex;
        const kempty_string: Self = RootIndex;
        const kUndefinedValue: Self = RootIndex;
        const kNullValue: Self = RootIndex;
        const kBigIntMap: Self = RootIndex;
        const kFixedArrayMap: Self = RootIndex;
    }

    #[derive(Debug, Copy, Clone)]
    pub struct FieldMemOperand {
        base: Register,
        offset: usize,
    }

    impl FieldMemOperand {
        pub fn new(base: Register, offset: usize) -> Self {
            FieldMemOperand { base, offset }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct SeqTwoByteString;
    impl SeqTwoByteString {
        fn SizeFor(length: i32) -> i32 {
            length * 2 // Example size calculation
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Name;
    impl Name {
        const kEmptyHashField: i32 = 0;
    }

    #[derive(Debug, Copy, Clone)]
    pub struct String;
    impl String {
        const kMaxOneByteCharCode: i32 = 256;
        const length_: usize = 0;
    }
    #[derive(Debug, Copy, Clone)]
    pub struct Input;
    impl Input {
        fn operand(&self) -> compiler::AllocatedOperand {
            compiler::AllocatedOperand {
                kind: compiler::AllocatedOperandKind::Constant,
            } // Stubbed
        }
        fn node(&self) -> *mut ValueNode {
            std::ptr::null_mut() // Placeholder
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct HeapNumber;
    impl HeapNumber {
        const value_: usize = 0;
    }

    #[derive(Debug, Copy, Clone)]
    pub struct BigInt;
    impl BigInt {
        const bitfield_: usize = 0;
    }

    #[derive(Debug, Copy, Clone)]
    pub struct ContextSidePropertyCell;
    impl ContextSidePropertyCell {
        const kPropertyDetailsRawOffset: usize = 0;
        fn Const() -> i32 {
            0
        }
    }
    #[derive(Debug, Copy, Clone)]
    pub struct FixedArray;
    impl FixedArray {
        fn OffsetOfElementAt(index: i32) -> usize {
            index as usize
        }
    }

    pub mod compiler {
        #[derive(Debug, Copy, Clone)]
        pub enum AllocatedOperandKind {
            Register,
            StackSlot,
            Constant,
        }

        #[derive(Debug, Copy, Clone)]
        pub struct AllocatedOperand {
            pub kind: AllocatedOperandKind,
        }

        impl AllocatedOperand {
            pub fn cast(_input: &Input) -> AllocatedOperand {
                AllocatedOperand {
                    kind: AllocatedOperandKind::Register,
                } // Stubbed
            }
            pub fn IsRegister(&self) -> bool {
                self.kind == AllocatedOperandKind::Register
            }
            pub fn IsStackSlot(&self) -> bool {
                self.kind == AllocatedOperandKind::StackSlot
            }
            pub fn IsConstant(&self) -> bool {
                self.kind == AllocatedOperandKind::Constant
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct JSReceiver;
    impl JSReceiver {
        const kPropertiesOrHashOffset: usize = 0;
    }

    #[derive(Debug, Copy, Clone)]
    pub struct FieldIndex {
        index: u32,
    }

    impl FieldIndex {
        pub fn offset(&self) -> usize {
            self.index as usize
        }
        pub fn is_inobject(&self) -> bool {
            true
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct PolymorphicAccessInfo {
        field_index: FieldIndex,
        holder: Option<PropertyHolder>,
    }

    impl PolymorphicAccessInfo {
        pub fn field_index(&self) -> FieldIndex {
            self.field_index
        }
        pub fn holder(&self) -> &Option<PropertyHolder> {
            &self.holder
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct PropertyHolder {
        object_: Register,
    }

    impl PropertyHolder {
        pub fn object(&self) -> Register {
            self.object_
        }
        pub fn has_value(&self) -> bool {
            true
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Smi;
    impl Smi {
        pub fn zero() -> Self {
            Smi
        }
        pub fn FromInt(value: i32) -> Self {
            Smi
        }
        pub fn IsValid(value: i32) -> bool {
            true
        }
        pub fn IsValidUint(value: u32) -> bool {
            true
        }
    }

    // Placeholder for MaglevAssembler functionality.
    pub struct MaglevAssembler {
        // Placeholders. These need proper initialization in real code.
        masm: *mut u8,
        isolate_: *mut u8,
        compilation_info_: *mut u8, //CompilationInfo,
        native_context_: NativeContext,
    }

    impl MaglevAssembler {
        pub fn new() -> Self {
            MaglevAssembler {
                masm: std::ptr::null_mut(),
                isolate_: std::ptr::null_mut(),
                compilation_info_: std::ptr::null_mut(),
                native_context_: NativeContext::new(),
            }
        }
        pub fn native_context(&self) -> &NativeContext {
            &self.native_context_
        }
        pub fn compilation_info(&self) -> CompilationInfo {
            CompilationInfo::new()
        }
        fn __(&self) -> *mut u8 {
            self.masm
        } // Placeholder

        pub fn AllocateHeapNumber(
            &mut self,
            register_snapshot: RegisterSnapshot,
            result: Register,
            value: DoubleRegister,
        ) {
            // In the case we need to call the runtime, we should spill the value
            // register. Even if it is not live in the next node, otherwise the
            // allocation call might trash it.
            let mut register_snapshot = register_snapshot;
            register_snapshot.live_double_registers.set(value);
            self.Allocate(register_snapshot, result, mem::size_of::<HeapNumber>() as i32);
            self.SetMapAsRoot(result, RootIndex::kHeapNumberMap);
            self.StoreFloat64(FieldMemOperand::new(result, HeapNumber::value_), value);
        }

        pub fn AllocateTwoByteString(
            &mut self,
            register_snapshot: RegisterSnapshot,
            result: Register,
            length: i32,
        ) {
            let size = SeqTwoByteString::SizeFor(length);
            self.Allocate(register_snapshot, result, size);
            self.StoreTaggedSignedField(result, (size as usize) - kObjectAlignment, Smi::zero());
            self.SetMapAsRoot(result, RootIndex::kSeqTwoByteStringMap);
            self.StoreInt32Field(result, Name::kEmptyHashField as usize, Name::kEmptyHashField);
            self.StoreInt32Field(result, String::length_, length);
        }

        pub fn FromAnyToRegister(&mut self, input: &Input, scratch: Register) -> Register {
            if input.operand().IsConstant() {
                unsafe {
                    input.node().as_mut().unwrap().LoadToRegister(self, scratch);
                }
                return scratch;
            }
            let operand = compiler::AllocatedOperand::cast(input);
            if operand.IsRegister() {
                return self.ToRegister(input);
            } else {
                //DCHECK(operand.IsStackSlot());
                self.Move(scratch, self.ToMemOperand(input));
                return scratch;
            }
        }

        pub fn LoadSingleCharacterString(&mut self, result: Register, char_code: i32) {
            //DCHECK_GE(char_code, 0);
            //DCHECK_LT(char_code, String::kMaxOneByteCharCode);
            let table = result;
            self.LoadRoot(table, RootIndex::kSingleCharacterStringTable);
            self.LoadTaggedField(
                result,
                table,
                (0 + (char_code as usize) * mem::size_of::<usize>()) as usize,
            ); // Placeholder
        }

        pub fn LoadDataField(
            &mut self,
            access_info: &PolymorphicAccessInfo,
            result: Register,
            object: Register,
            scratch: Register,
        ) {
            let mut load_source = object;
            // Resolve property holder.
            if access_info.holder().is_some() {
                load_source = scratch;
                self.Move(load_source, access_info.holder().unwrap().object());
            }
            let field_index = access_info.field_index();
            if !field_index.is_inobject() {
                let load_source_object = load_source;
                if load_source == object {
                    load_source = scratch;
                }
                // The field is in the property array, first load it from there.
                self.AssertNotSmi(load_source_object);
                self.LoadTaggedField(
                    load_source,
                    load_source_object,
                    JSReceiver::kPropertiesOrHashOffset,
                );
            }
            self.AssertNotSmi(load_source);
            self.LoadTaggedField(result, load_source, field_index.offset());
        }

        pub fn JumpIfNotUndetectable(
            &mut self,
            object: Register,
            scratch: Register,
            check_type: CheckType,
            target: &mut Label,
            distance: Label::Distance,
        ) {
            if check_type == CheckType::kCheckHeapObject {
                self.JumpIfSmi(object, target, distance);
            } else {
                //v8_flags.debug_code
                self.AssertNotSmi(object);
            }
            // For heap objects, check the map's undetectable bit.
            self.LoadMap(scratch, object);
            self.TestUint8AndJumpIfAllClear(
                FieldMemOperand::new(scratch, Map::kBitFieldOffset),
                Map::Bits1::IsUndetectableBit::kMask,
                target,
                distance,
            );
        }

        pub fn JumpIfUndetectable(
            &mut self,
            object: Register,
            scratch: Register,
            check_type: CheckType,
            target: &mut Label,
            distance: Label::Distance,
        ) {
            let mut detectable = Label::new();
            if check_type == CheckType::kCheckHeapObject {
                self.JumpIfSmi(object, &mut detectable, Label::Distance::kNear);
            } else {
                //v8_flags.debug_code
                self.AssertNotSmi(object);
            }
            // For heap objects, check the map's undetectable bit.
            self.LoadMap(scratch, object);
            self.TestUint8AndJumpIfAnySet(
                FieldMemOperand::new(scratch, Map::kBitFieldOffset),
                Map::Bits1::IsUndetectableBit::kMask,
                target,
                distance,
            );
            self.bind(&mut detectable);
        }

        pub fn JumpIfNotCallable(
            &mut self,
            object: Register,
            scratch: Register,
            check_type: CheckType,
            target: &mut Label,
            distance: Label::Distance,
        ) {
            if check_type == CheckType::kCheckHeapObject {
                self.JumpIfSmi(object, target, distance);
            } else {
                //v8_flags.debug_code
                self.AssertNotSmi(object);
            }
            self.LoadMap(scratch, object);
            //static_assert(Map::kBitFieldOffsetEnd + 1 - Map::kBitFieldOffset == 1);
            self.TestUint8AndJumpIfAllClear(
                FieldMemOperand::new(scratch, Map::kBitFieldOffset),
                Map::Bits1::IsCallableBit::kMask,
                target,
                distance,
            );
        }

        pub fn EnsureWritableFastElements(
            &mut self,
            register_snapshot: RegisterSnapshot,
            elements: Register,
            object: Register,
            scratch: Register,
        ) {
            let done = ZoneLabelRef::new();
            self.CompareMapWithRoot(elements, RootIndex::kFixedArrayMap, scratch);
            self.JumpToDeferredIf(
                kNotEqual,
                |masm: &mut MaglevAssembler,
                 done: ZoneLabelRef,
                 object: Register,
                 result_reg: Register,
                 snapshot: RegisterSnapshot| {
                    let mut snapshot = snapshot;
                    snapshot.live_registers.clear(result_reg);
                    snapshot.live_tagged_registers.clear(result_reg);
                    let mut save_register_state = SaveRegisterStateForCall::new(masm, snapshot);
                    // Builtin::kCopyFastSmiOrObjectElements
                    masm.CallBuiltin(object); // Placeholder
                    save_register_state.DefineSafepoint();
                    masm.Move(result_reg, kReturnRegister0);
                    masm.Jump(*done);
                },
                done,
                object,
                elements,
                register_snapshot,
            );
            self.bind(*done);
        }

        pub fn ToBoolean(
            &mut self,
            value: Register,
            check_type: CheckType,
            is_true: ZoneLabelRef,
            is_false: ZoneLabelRef,
            fallthrough_when_true: bool,
        ) {
            let mut temps = TemporaryRegisterScope::new(self);

            if check_type == CheckType::kCheckHeapObject {
                // Check if {{value}} is Smi.
                let is_smi = self.CheckSmi(value);
                self.JumpToDeferredIf(
                    is_smi,
                    |masm: &mut MaglevAssembler,
                     value: Register,
                     is_true: ZoneLabelRef,
                     is_false: ZoneLabelRef| {
                        // Check if {value} is not zero.
                        masm.CompareSmiAndJumpIf(value, Smi::FromInt(0), kEqual, *is_false);
                        masm.Jump(*is_true);
                    },
                    value,
                    is_true,
                    is_false,
                );
            } else {
                //v8_flags.debug_code
                self.AssertNotSmi(value);
            }

            // V8_STATIC_ROOTS_BOOL
            // Check if {{value}} is a falsey root or the true value.
            // Undefined is the first root, so it's the smallest possible pointer
            // value, which means we don't have to subtract it for the range check.
            // ReadOnlyRoots roots(isolate_);
            // static_assert(StaticReadOnlyRoot::kFirstAllocatedRoot ==
            //             StaticReadOnlyRoot::kUndefinedValue);
            // static_assert(StaticReadOnlyRoot::kUndefinedValue + sizeof(Undefined) ==
            //             StaticReadOnlyRoot::kNullValue);
            // static_assert(StaticReadOnlyRoot::kNullValue + sizeof(Null) ==
            //             StaticReadOnlyRoot::kempty_string);
            // static_assert(StaticReadOnlyRoot::kempty_string +
            //                 SeqOneByteString::SizeFor(0) ==
            //             StaticReadOnlyRoot::kFalseValue);
            // static_assert(StaticReadOnlyRoot::kFalseValue + sizeof(False) ==
            //             StaticReadOnlyRoot::kTrueValue);
            // CompareInt32AndJumpIf(value, StaticReadOnlyRoot::kTrueValue,
            //                     kUnsignedLessThan, *is_false);
            // // Reuse the condition flags from the above int32 compare to also check for
            // // the true value itself.
            // JumpIf(kEqual, *is_true);

            // Check if {{value}} is false.
            self.JumpIfRoot(value, RootIndex::kFalseValue, *is_false);

            // Check if {{value}} is true.
            self.JumpIfRoot(value, RootIndex::kTrueValue, *is_true);

            // Check if {{value}} is empty string.
            self.JumpIfRoot(value, RootIndex::kempty_string, *is_false);

            // Only check null and undefined if we're not going to check the
            // undetectable bit.
            if self
                .compilation_info()
                .broker()
                .dependencies()
                .DependOnNoUndetectableObjectsProtector()
            {
                // Check if {{value}} is undefined.
                self.JumpIfRoot(value, RootIndex::kUndefinedValue, *is_false);

                // Check if {{value}} is null.
                self.JumpIfRoot(value, RootIndex::kNullValue, *is_false);
            }

            let map = temps.AcquireScratch();
            self.LoadMap(map, value);

            if !self
                .compilation_info()
                .broker()
                .dependencies()
                .DependOnNoUndetectableObjectsProtector()
            {
                // Check if {{value}} is undetectable.
                self.TestUint8AndJumpIfAnySet(
                    FieldMemOperand::new(map, Map::kBitFieldOffset),
                    Map::Bits1::IsUndetectableBit::kMask,
                    *is_false,
                );
            }

            // Check if {{value}} is a HeapNumber.
            self.JumpIfRoot(
                map,
                RootIndex::kHeapNumberMap,
                self.MakeDeferredCode(
                    |masm: &mut MaglevAssembler,
                     value: Register,
                     is_true: ZoneLabelRef,
                     is_false: ZoneLabelRef| {
                        masm.CompareDoubleAndJumpIfZeroOrNaN(
                            FieldMemOperand::new(value, HeapNumber::value_),
                            *is_false,
                        );
                        masm.Jump(*is_true);
                    },
                    value,
                    is_true,
                    is_false,
                ),
            );

            // Check if {{value}} is a BigInt.
            // {{map}} is not needed after this check, we pass to the deferred code, so it
            // can be added to the temporary registers.
            self.JumpIfRoot(
                map,
                RootIndex::kBigIntMap,
                self.MakeDeferredCode(
                    |masm: &mut MaglevAssembler,
                     value: Register,
                     map: Register,
                     is_true: ZoneLabelRef,
                     is_false: ZoneLabelRef| {
                        let mut temps = TemporaryRegisterScope::new(masm);
                        temps.IncludeScratch(map);
                        masm.TestInt32AndJumpIfAllClear(
                            FieldMemOperand::new(value, BigInt::bitfield_),
                            0, //BigInt::LengthBits::kMask,
                            *is_false,
                        );
                        masm.Jump(*is_true);
                    },
                    value,
                    map,
                    is_true,
                    is_false,
                ),
            );
            // Otherwise true.
            if !fallthrough_when_true {
                self.Jump(*is_true);
            }
        }

        pub fn MaterialiseValueNode(&mut self, dst: Register, value: *mut ValueNode) {
            unsafe {
                match (*value).opcode() {
                    Opcode::kInt32Constant => {
                        let int_value = (*value).Cast::<Int32Constant>().value();
                        if Smi::IsValid(int_value) {
                            self.Move(dst, Smi::FromInt(int_value));
                        } else {
                            self.MoveHeapNumber(dst, int_value);
                        }
                        return;
                    }
                    Opcode::kUint32Constant => {
                        let uint_value = (*value).Cast::<Uint32Constant>().value();
                        if Smi::IsValidUint(uint_value) {
                            self.Move(dst, Smi::FromInt(uint_value as i32));
                        } else {
                            self.MoveHeapNumber(dst, uint_value as i32);
                        }
                        return;
                    }
                    Opcode::kFloat64Constant => {
                        let double_value =
                            (*value).Cast::<Float64Constant>().value().get_scalar();
                        let mut smi_value: i32 = 0;
                        //if (DoubleToSmiInteger(double_value, &smi_value)) {
                        // Placeholder check
                        if true {
                            self.Move(dst, Smi::FromInt(smi_value));
                        } else {
                            self.MoveHeapNumber(dst, double_value as i32);
                        }
                        return;
                    }
                    _ => {}
                }
                //DCHECK(!value->allocation().IsConstant());
                //DCHECK(value->allocation().IsAnyStackSlot());
                //using D = NewHeapNumberDescriptor;
                //DoubleRegister builtin_input_value = D::GetDoubleRegisterParameter(D::kValue);
                //MemOperand src = ToMemOperand(value->allocation());
                //switch (value->properties().value_representation()) {
                //ValueRepresentation::kInt32 => {
                //    Label done;
                //    TemporaryRegisterScope temps(this);
                //    Register scratch = temps.AcquireScratch();
                //    Move(scratch, src);
                //    SmiTagInt32AndJumpIfSuccess(dst, scratch, &done, Label::kNear);
                //    // If smi tagging fails, instead of bailing out (deopting), we change
                //    // representation to a HeapNumber.
                //    Int32ToDouble(builtin_input_value, scratch);
                //    CallBuiltin<Builtin::kNewHeapNumber>(builtin_input_value);
                //    Move(dst, kReturnRegister0);
                //    bind(&done);
                //    break;
                //}
                //ValueRepresentation::kUint32 => {
                //    Label done;
                //    TemporaryRegisterScope temps(this);
                //    Register scratch = temps.AcquireScratch();
                //    Move(scratch, src);
                //    SmiTagUint32AndJumpIfSuccess(dst, scratch, &done, Label::kNear);
                //    // If smi tagging fails, instead of bailing out (deopting), we change
                //    // representation to a HeapNumber.
                //    Uint32ToDouble(builtin_input_value, scratch);
                //    CallBuiltin<Builtin::kNewHeapNumber>(builtin_input_value);
                //    Move(dst, kReturnRegister0);
                //    bind(&done);
                //    break;
                //}
                //ValueRepresentation::kFloat64 =>
                //    LoadFloat64(builtin_input_value, src);
                //    CallBuiltin<Builtin::kNewHeapNumber>(builtin_input_value);
                //    Move(dst, kReturnRegister0);
                //    break;
                //ValueRepresentation::kHoleyFloat64 => {
                //    Label done, box;
                //    JumpIfNotHoleNan(src, &box, Label::kNear);
                //    LoadRoot(dst, RootIndex::kUndefinedValue);
                //    Jump(&done);
                //    bind(&box);
                //    LoadFloat64(builtin_input_value, src);
                //    CallBuiltin<Builtin::kNewHeapNumber>(builtin_input_value);
                //    Move(dst, kReturnRegister0);
                //    bind(&done);
                //    break;
                //}
                //ValueRepresentation::kIntPtr => {
                //    Label done;
                //    TemporaryRegisterScope temps(this);
                //    Register scratch = temps.AcquireScratch();
                //    Move(scratch, src);
                //    SmiTagIntPtrAndJumpIfSuccess(dst, scratch, &done, Label::kNear);
                //    // If smi tagging fails, instead of bailing out (deopting), we change
                //    // representation to a HeapNumber.
                //    IntPtrToDouble(builtin_input_value, scratch);
                //    CallBuiltin<Builtin::kNewHeapNumber>(builtin_input_value);
                //    Move(dst, kReturnRegister0);
                //    bind(&done);
                //    break;
                //}
                //ValueRepresentation::kTagged =>
                //    UNREACHABLE();
                //}
            }
        }

        pub fn TestTypeOf(
            &mut self,
            object: Register,
            literal: interpreter::TestTypeOfFlags::LiteralFlag,
            is_true: &mut Label,
            true_distance: Label::Distance,
            fallthrough_when_true: bool,
            is_false: &mut Label,
            false_distance: Label::Distance,
            fallthrough_when_false: bool,
        ) {
            // If both true and false are fallthroughs, we don't have to do anything.
            if fallthrough_when_true && fallthrough_when_false {
                return;
            }

            // IMPORTANT: Note that `object` could be a register that aliases registers in
            // the TemporaryRegisterScope. Make sure that all reads of `object` are before
            // any writes to scratch registers
            //using LiteralFlag = interpreter::TestTypeOfFlags::LiteralFlag;
            match literal {
                interpreter::TestTypeOfFlags::LiteralFlag::kNumber => {
                    let mut temps = TemporaryRegisterScope::new(self);
                    let scratch = temps.AcquireScratch();
                    self.JumpIfSmi(object, is_true, true_distance);
                    self.CompareMapWithRoot(object, RootIndex::kHeapNumberMap, scratch);
                    self.Branch(
                        kEqual,
                        is_true,
                        true_distance,
                        fallthrough_when_true,
                        is_false,
                        false_distance,
                        fallthrough_when_false,
                    );
                    return;
                }
                interpreter::TestTypeOfFlags::LiteralFlag::kString => {
                    self.JumpIfSmi(object, is_false, false_distance);
                    self.CheckJSAnyIsStringAndBranch(
                        object,
                        is_true,
                        true_distance,
                        fallthrough_when_true,
                        is_false,
                        false_distance,
                        fallthrough_when_false,
                    );
                    return;
                }
                interpreter::TestTypeOfFlags::LiteralFlag::kSymbol => {
                    self.JumpIfSmi(object, is_false, false_distance);
                    self.BranchOnObjectType(
                        object,
                        ObjectType::SYMBOL_TYPE,
                        is_true,
                        true_distance,
                        fallthrough_when_true,
                        is_false,
                        false_distance,
                        fallthrough_when_false,
                    );
                    return;
                }
                interpreter::TestTypeOfFlags::LiteralFlag::kBoolean => {
                    self.JumpIfRoot(object, RootIndex::kTrueValue, is_true, true_distance);
                    self.CompareRoot(object, RootIndex::kFalseValue);
                    self.Branch(
                        kEqual,
                        is_true,
                        true_distance,
                        fallthrough_when_true,
                        is_false,
                        false_distance,
                        fallthrough_when_false,
                    );
                    return;
                }
                interpreter::TestTypeOfFlags::LiteralFlag::kBigInt => {
                    self.JumpIfSmi(object, is_false, false_distance);
                    self.BranchOnObjectType(
                        object,
                        ObjectType::BIGINT_TYPE,
                        is_true,
                        true_distance,
                        fallthrough_when_true,
                        is_false,
                        false_distance,
                        fallthrough_when_false,
                    );
                    return;
                }
                interpreter::TestTypeOfFlags::LiteralFlag::kUndefined => {
                    let mut temps = TemporaryRegisterScope::new(self);
                    let map = temps.AcquireScratch();
                    // Make sure `object` isn't a valid temp here, since we reuse it.
                    //DCHECK(!temps.Available().has(object));
                    self.JumpIfSmi(object, is_false, false_distance);
                    // Check it has the undetectable bit set and it is not null.
                    self.LoadMap(map, object);
                    self.TestUint8AndJumpIfAllClear(
                        FieldMemOperand::new(map, Map::kBitFieldOffset),
                        Map::Bits1::IsUndetectableBit::kMask,
                        is_false,
                        false_distance,
                    );
                    self.CompareRoot(object, RootIndex::kNullValue);
                    self.Branch(
                        kNotEqual,
                        is_true,
                        true_distance,
                        fallthrough_when_true,
                        is_false,
                        false_distance,
                        fallthrough_when_false,
                    );
                    return;
                }
                interpreter::TestTypeOfFlags::LiteralFlag::kFunction => {
                    let mut temps = TemporaryRegisterScope::new(self);
                    let scratch = temps.AcquireScratch();
                    self.JumpIfSmi(object, is_false, false_distance);
                    // Check if callable bit is set and not undetectable.
                    self.LoadMap(scratch, object);
                    self.Branch(
                        self.IsCallableAndNotUndetectable(scratch, scratch),
                        is_true,
                        true_distance,
                        fallthrough_when_true,
                        is_false,
                        false_distance,
                        fallthrough_when_false,
                    );
                    return;
                }
                interpreter::TestTypeOfFlags::LiteralFlag::kObject => {
                    let mut temps = TemporaryRegisterScope::new(self);
                    let scratch = temps.AcquireScratch();
                    self.JumpIfSmi(object, is_false, false_distance);
                    // If the object is null then return true.
                    self.JumpIfRoot(object, RootIndex::kNullValue, is_true, true_distance);
                    // Check if the object is a receiver type,
                    self.LoadMap(scratch, object);
                    self.CompareInstanceTypeAndJumpIf(
                        scratch,
                        FIRST_JS_RECEIVER_TYPE,
                        kLessThan,
                        is_false,
                        false_distance,
                    );
                    // ... and is not undefined (undetectable) nor callable.
                    self.Branch(
                        self.IsNotCallableNorUndetactable(scratch, scratch),
                        is_true,
                        true_distance,
                        fallthrough_when_true,
                        is_false,
                        false_distance,
                        fallthrough_when_false,
                    );
                    return;
                }
                interpreter::TestTypeOfFlags::LiteralFlag::kOther => {
                    if !fallthrough_when_false {
                        self.Jump(is_false, false_distance);
                    }
                    return;
                }
            }
            //UNREACHABLE();
        }
        fn CheckJSAnyIsStringAndBranch(
            &mut self,
            object: Register,
            is_true: &mut Label,
            true_distance: Label::Distance,
            fallthrough_when_true: bool,
            is_false: &mut Label,
            false_distance: Label::Distance,
            fallthrough_when_false: bool,
        ) {
            todo!()
        }

        fn BranchOnObjectType(
            &mut self,
            object: Register,
            symbol_type: ObjectType,
            is_true: &mut Label,
            true_distance: Label::Distance,
            fallthrough_when_true: bool,
            is_false: &mut Label,
            false_distance: Label::Distance,
            fallthrough_when_false: bool,
        ) {
            todo!()
        }
        fn IsCallableAndNotUndetectable(
            &mut self,
            scratch: Register,
            scratch1: Register,
        ) -> Condition {
            Condition::kEqual
        }
        fn IsNotCallableNorUndetactable(
            &mut self,
            scratch: Register,
            scratch1: Register,
        ) -> Condition {
            Condition::kEqual
        }

        fn JumpToDeferredIf<F>(
            &mut self,
            condition: Condition