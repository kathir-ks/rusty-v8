// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use std::os::raw::c_char;
//use std::ffi::CStr;
use std::rc::Rc;

// Placeholder for base::strings, which would require significant porting.
mod base {
    pub type uc16 = u16;
}

// Placeholder for builtins-regexp-gen.h and related functionality
mod builtins_regexp_gen {
    // Define a dummy trait or struct if needed
    pub struct RegExpMatchAllAssembler {}

    impl RegExpMatchAllAssembler {
        pub fn new() -> Self {
            RegExpMatchAllAssembler {}
        }
    }
}

// Placeholder for builtins-utils-gen.h and related functionality
mod builtins_utils_gen {}

// Placeholder for builtins.h and related functionality
mod builtins {
    pub enum Builtin {
        StringIndexOf,
        StringSubstring,
        StringAdd_CheckNone,
        RegExpReplace,
        RegExpSplit,
    }
}

// Placeholder for codegen/code-stub-assembler-inl.h
mod code_stub_assembler {
    // Dummy type for now. Needs proper definition based on V8's internal usage.
    pub struct CodeStubAssembler {}
    impl CodeStubAssembler {
        pub fn new() -> Self {
            CodeStubAssembler {}
        }
    }
}

// Placeholder for execution/protectors.h and related functionality
mod protectors {}

// Placeholder for heap/factory-inl.h and related functionality
mod factory_inl {
  pub struct Factory {

  }
  impl Factory {
    pub fn new() -> Self {
      Factory {}
    }
  }
}

// Placeholder for heap/heap-inl.h and related functionality
mod heap_inl {}

// Placeholder for logging/counters.h and related functionality
mod counters {}

// Placeholder for objects/instance-type.h
mod instance_type {
    pub const kStringRepresentationMask: i32 = 0x3;
    pub const kSeqStringTag: i32 = 0x0;
    pub const kIsIndirectStringMask: i32 = 0x4;
    pub const kUncachedExternalStringMask: i32 = 0x8;
    pub const kIsNotInternalizedMask: i32 = 0x10;
    pub const kInternalizedTag: i32 = 0x0;
    pub const kStringEncodingMask: i32 = 0x4;
    pub const kExternalStringTag: i32 = 0x20;
    pub const kOneByteStringTag: i32 = 0x0;
    pub const kTwoByteStringTag: i32 = 0x4;
    pub const kThinStringTagBit: i32 = 0x20;
    pub const kConsStringTag: i32 = 0x1;
}

// Placeholder for objects/objects.h
mod objects {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct String {
        pub length_: u32,
        pub hash_field_: i32,
        pub data: Vec<u8>,
        pub encoding: Encoding
    }

    impl String {
        pub const kEmptyHashField: i32 = 0;
        pub const kMaxLength: u32 = 1 << 28; // Example value
        pub const ONE_BYTE_ENCODING: Encoding = Encoding::OneByte;
        pub const TWO_BYTE_ENCODING: Encoding = Encoding::TwoByte;
        pub fn len(&self) -> usize {
          self.length_ as usize
        }
    }

    #[derive(Clone, Copy)]
    pub enum Encoding {
      OneByte,
      TwoByte
    }

    pub struct SeqOneByteString {
      pub string: String
    }
    pub struct SeqTwoByteString {
      pub string: String
    }
    pub struct ThinString {
        pub actual_: Rc<RefCell<String>>
    }
    pub struct ConsString {
        pub length_: u32,
        pub hash_field_: i32,
        pub first_: Rc<RefCell<String>>,
        pub second_: Rc<RefCell<String>>,
    }

    impl ConsString {
        pub const kMinLength: u32 = 16;
    }
    pub struct SlicedString {}
    pub struct JSArray {}
}

// Placeholder for objects/property-cell.h
mod property_cell {
    pub struct PropertyCell {
        pub value: i32, // Simulate value
    }
    impl PropertyCell {
        pub const kValueOffset: usize = 0;
    }
}

mod v8_internal {
    use super::*;

    use instance_type::*;
    use objects::*;
    use std::{cell::RefCell, rc::Rc};

    //const kHeapObjectTag: usize = 0;
    const kTaggedSize: usize = 8;
    const kObjectAlignment: usize = 8;

    //const OFFSET_OF_DATA_START_SeqOneByteString: usize = 8;
    //const OFFSET_OF_DATA_START_SeqTwoByteString: usize = 8;

    // Placeholder types.  Need to be replaced with actual representations.
    pub type TNode<T> = Rc<RefCell<T>>;
    pub type TVariable<T> = Rc<RefCell<T>>;
    pub type IntPtrT = isize;
    pub type Int32T = i32;
    pub type Uint32T = u32;
    pub type Uint16T = u16;
    pub type Uint8T = u8;
    pub type Word32T = u32;
    pub type Word64T = u64;
    pub type WordT = usize;
    pub type RawPtrT = *mut u8;
    pub type JSAny = Rc<RefCell<String>>;
    pub type JSAnyNotSmi = Rc<RefCell<String>>;
    pub type Context = i32;
    pub type NativeContext = i32;
    pub type Object = Rc<RefCell<String>>;
    pub type BoolT = bool;
    pub type HeapObject = Rc<RefCell<String>>;
    pub type Smi = i32;
    pub type Number = f64;
    pub type JSArray = objects::JSArray;
    pub type FixedArray = i32;
    pub type Map = i32;
    pub type ContextOrEmptyContext = i32;

    //Dummy constant for StringComparison
    pub enum StringComparison {
      kLessThan,
      kLessThanOrEqual,
      kGreaterThan,
      kGreaterThanOrEqual,
      kCompare
    }

    pub enum UnicodeEncoding {
        UTF16,
        UTF32,
    }

    #[derive(Clone, Copy)]
    pub enum ElementsKind {
      PACKED_ELEMENTS,
      UINT8_ELEMENTS,
      UINT16_ELEMENTS
    }
    pub enum RootIndex {
      kreplace_symbol,
      kmatch_all_symbol,
      kTheHoleValue,
      ksplit_symbol
    }

    pub struct SingleCharacterStringTable {}
    pub struct DescriptorIndexNameValue {
      pub function_descriptor_index: i32,
      pub root_index: RootIndex,
      pub context_index: i32
    }

    pub struct VariableList<'a> {
        vars: Vec<&'a TVariable<String>>
    }
    impl <'a> VariableList<'a>{
        pub fn new(vars: Vec<&'a TVariable<String>>) -> Self {
            VariableList {
                vars: vars
            }
        }
    }

    // Placeholder traits to simulate the TNode-based DSL.  These will likely need
    // more fidelity to the original V8 implementation.
    pub trait Node {
        //fn value(&self) -> Self;
    }

    pub trait CodeStubAssemblerMethods {
        fn word32_equal(&self, x: TNode<Word32T>, y: TNode<Word32T>) -> TNode<BoolT>;
        fn word_equal(&self, x: TNode<IntPtrT>, y: TNode<IntPtrT>) -> TNode<BoolT>;
        fn tagged_equal(&self, x: TNode<String>, y: TNode<String>) -> TNode<BoolT>;
        fn tagged_not_equal(&self, x: TNode<String>, y: TNode<String>) -> TNode<BoolT>;
        fn uint32_less_than(&self, x: TNode<Int32T>, y: TNode<Int32T>) -> TNode<BoolT>;
        fn intptr_equal(&self, x: TNode<IntPtrT>, y: TNode<IntPtrT>) -> TNode<BoolT>;
        fn is_string(&self, object: TNode<String>) -> TNode<BoolT>;
        fn is_heap_number(&self, object: TNode<String>) -> TNode<BoolT>;
        fn is_null_or_undefined(&self, object: TNode<String>) -> TNode<BoolT>;
        fn throw_type_error(&self, context: TNode<Context>, message: MessageTemplate);
        fn uint32_greater_than(&self, x: TNode<Uint32T>, y: TNode<Uint32T>) -> TNode<BoolT>;
        fn is_callable_map(&self, map: TNode<Map>) -> TNode<BoolT>;
        fn tagged_is_smi(&self, object: TNode<String>) -> TNode<BoolT>;
        fn word32_binary_not(&self, x: TNode<Int32T>) -> TNode<Int32T>;
        fn uintptr_greater_than(&self, x: TNode<IntPtrT>, y: TNode<IntPtrT>) -> TNode<BoolT>;
        fn uintptr_greater_than_or_equal(&self, x: TNode<IntPtrT>, y: TNode<IntPtrT>) -> TNode<BoolT>;
        fn intptr_less_than(&self, x: TNode<IntPtrT>, y: TNode<IntPtrT>) -> TNode<BoolT>;
        fn intptr_greater_than(&self, x: TNode<IntPtrT>, y: TNode<IntPtrT>) -> TNode<BoolT>;
        fn is_number_string_not_regexp_like_protector_cell_invalid(&self) -> TNode<BoolT>;
        fn branch(&self, condition: TNode<BoolT>, if_true: &dyn Fn(), if_false: &dyn Fn());
        fn goto_if(&self, condition: TNode<BoolT>, label: &Label);
        fn goto_if_not(&self, condition: TNode<BoolT>, label: &Label);
        fn call_builtin<T>(&self, builtin: builtins::Builtin, context: TNode<Context>, arg1: TNode<String>, arg2: TNode<String>, arg3: TNode<Smi>) -> TNode<T>;
    }

    pub struct Label {
        name: String
    }
    impl Label {
        pub fn new(name: String) -> Self {
            Label {
                name: name
            }
        }
    }

    pub enum MachineType {
        Uint8,
        Uint16,
        Uint32,
        Pointer,
        IntPtr,
        Word8,
        Word16,
        Word32,
        Word64,
    }
    impl MachineType {
      pub fn representation(&self) -> MachineRepresentation {
        match self {
          MachineType::Uint8 => MachineRepresentation::kWord8,
          MachineType::Uint16 => MachineRepresentation::kWord16,
          _ => MachineRepresentation::kWord32
        }
      }
    }

    #[derive(Clone, Copy)]
    pub enum MachineRepresentation {
      kWord8,
      kWord16,
      kWord32,
      kWord64
    }

    pub enum Descriptor {
        kLeft,
        kRight,
        kContext,
        kString,
        kFrom,
        kTo,
        kLength,
        kPosition,
        kReceiver,
        kSearch,
        kReplace,
        kRegexp
        kJSActualArgumentsCount
    }

    pub enum MessageTemplate {
        kRegExpGlobalInvokedOnNonGlobal,
        kStringMatchAllNullOrUndefinedFlags
    }
}

use v8_internal::*;

macro_rules! offset_of {
    ($struct:ty, $field:tt) => {
        unsafe {
            let base = std::ptr::null::<$struct>();
            &(*base).$field as *const _ as usize - base as *const _ as usize
        }
    };
}

//mod define_code_stub_assembler_macros {}

pub struct StringBuiltinsAssembler {
    state: code_stub_assembler::CodeStubAssembler
}

impl StringBuiltinsAssembler {
    pub fn new() -> Self {
        StringBuiltinsAssembler {
            state: code_stub_assembler::CodeStubAssembler::new()
        }
    }

    fn direct_string_data(&self, string: TNode<String>, string_instance_type: TNode<Word32T>) -> TNode<RawPtrT> {
        let var_data: TVariable<RawPtrT> = Rc::new(RefCell::new(std::ptr::null_mut())); // Initialized as null

        let if_sequential = Label::new("if_sequential".to_string());
        let if_external = Label::new("if_external".to_string());
        let if_join = Label::new("if_join".to_string());

        if self.word32_equal(Rc::new(RefCell::new((string_instance_type.borrow().clone() & instance_type::kStringRepresentationMask as u32) as u32)), Rc::new(RefCell::new(instance_type::kSeqStringTag as u32))){
          self.goto_if(Rc::new(RefCell::new(true)), &if_sequential);
        } else {
          self.goto_if(Rc::new(RefCell::new(true)), &if_external);
        }

        //Branch(Word32Equal(Word32And(string_instance_type,
        //                             Int32Constant(kStringRepresentationMask)),
        //                  Int32Constant(kSeqStringTag)),
        //    &if_sequential, &if_external);

        self.goto_if(Rc::new(RefCell::new(true)), &if_sequential);
        //BIND(&if_sequential);
        {
            assert_eq!(offset_of!(SeqOneByteString, string.data), offset_of!(SeqTwoByteString, string.data));
            //var_data = RawPtrAdd(ReinterpretCast<RawPtrT>(BitcastTaggedToWord(string)),
            //                     IntPtrConstant(OFFSET_OF_DATA_START(SeqOneByteString) -
            //                                    kHeapObjectTag));
            //Placeholder
            *var_data.borrow_mut() = 0x123 as RawPtrT;
            self.goto_if(Rc::new(RefCell::new(true)), &if_join);
        }

        //BIND(&if_external);
        self.goto_if(Rc::new(RefCell::new(true)), &if_external);
        {
            //var_data = LoadExternalStringResourceDataPtr(CAST(string));
            //Placeholder
            *var_data.borrow_mut() = 0x456 as RawPtrT;
            self.goto_if(Rc::new(RefCell::new(true)), &if_join);
        }

        //BIND(&if_join);
        self.goto_if(Rc::new(RefCell::new(true)), &if_join);
        return Rc::new(RefCell::new(var_data.borrow().clone()));
    }

    fn call_search_string_raw<SubjectChar, PatternChar>(
        &self,
        subject_ptr: TNode<RawPtrT>,
        subject_length: TNode<IntPtrT>,
        search_ptr: TNode<RawPtrT>,
        search_length: TNode<IntPtrT>,
        start_position: TNode<IntPtrT>,
    ) -> TNode<IntPtrT> {
        // TODO: Implement the C function call here
        // This is a placeholder, replace with actual C function call logic
        Rc::new(RefCell::new(0))
    }

    fn search_one_byte_string_in_two_byte_string(
        &self,
        subject_ptr: TNode<RawPtrT>,
        subject_length: TNode<IntPtrT>,
        search_ptr: TNode<RawPtrT>,
        search_length: TNode<IntPtrT>,
        start_position: TNode<IntPtrT>,
    ) -> TNode<IntPtrT> {
        self.call_search_string_raw::<base::uc16, u8>(
            subject_ptr,
            subject_length,
            search_ptr,
            search_length,
            start_position,
        )
    }

    fn search_one_byte_string_in_one_byte_string(
        &self,
        subject_ptr: TNode<RawPtrT>,
        subject_length: TNode<IntPtrT>,
        search_ptr: TNode<RawPtrT>,
        search_length: TNode<IntPtrT>,
        start_position: TNode<IntPtrT>,
    ) -> TNode<IntPtrT> {
        self.call_search_string_raw::<u8, u8>(
            subject_ptr,
            subject_length,
            search_ptr,
            search_length,
            start_position,
        )
    }

    fn search_two_byte_string_in_two_byte_string(
        &self,
        subject_ptr: TNode<RawPtrT>,
        subject_length: TNode<IntPtrT>,
        search_ptr: TNode<RawPtrT>,
        search_length: TNode<IntPtrT>,
        start_position: TNode<IntPtrT>,
    ) -> TNode<IntPtrT> {
        self.call_search_string_raw::<base::uc16, base::uc16>(
            subject_ptr,
            subject_length,
            search_ptr,
            search_length,
            start_position,
        )
    }

    fn search_two_byte_string_in_one_byte_string(
        &self,
        subject_ptr: TNode<RawPtrT>,
        subject_length: TNode<IntPtrT>,
        search_ptr: TNode<RawPtrT>,
        search_length: TNode<IntPtrT>,
        start_position: TNode<IntPtrT>,
    ) -> TNode<IntPtrT> {
        self.call_search_string_raw::<u8, base::uc16>(
            subject_ptr,
            subject_length,
            search_ptr,
            search_length,
            start_position,
        )
    }

    fn search_one_byte_in_one_byte_string(
        &self,
        subject_ptr: TNode<RawPtrT>,
        subject_length: TNode<IntPtrT>,
        search_ptr: TNode<RawPtrT>,
        start_position: TNode<IntPtrT>,
    ) -> TNode<IntPtrT> {
        // TODO: Implement the C function call here
        // This is a placeholder, replace with actual C function call logic
        Rc::new(RefCell::new(0))
    }

    fn generate_string_equal(&self, left: TNode<String>, right: TNode<String>, length: TNode<IntPtrT>) {
        let var_left: TVariable<String> = Rc::new(RefCell::new(left.borrow().clone()));
        let var_right: TVariable<String> = Rc::new(RefCell::new(right.borrow().clone()));

        let if_equal = Label::new("if_equal".to_string());
        let if_notequal = Label::new("if_notequal".to_string());
        let if_indirect = Label::new("if_indirect".to_string());
        let start = Label::new("start".to_string());

        self.goto_if(Rc::new(RefCell::new(true)), &start);
        // Callers must handle the case where {lhs} and {rhs} refer to the same
        // String object.
        //CSA_DCHECK(this, TaggedNotEqual(left, right));
        //
        //CSA_DCHECK(this, IntPtrEqual(LoadStringLengthAsWord(left), length));
        //CSA_DCHECK(this, IntPtrEqual(LoadStringLengthAsWord(right), length));
        //
        //Goto(&start);
        //BIND(&start);
        //TNode<String> lhs = var_left.value();
        //TNode<String> rhs = var_right.value();

        let lhs = var_left.clone();
        let rhs = var_right.clone();

        //TNode<Uint16T> lhs_instance_type = LoadInstanceType(lhs);
        //TNode<Uint16T> rhs_instance_type = LoadInstanceType(rhs);
        let lhs_instance_type = Rc::new(RefCell::new(0 as u16)) as TNode<Uint16T>;
        let rhs_instance_type = Rc::new(RefCell::new(0 as u16)) as TNode<Uint16T>;

        //StringEqual_Core(lhs, lhs_instance_type, rhs, rhs_instance_type, length,
        //                 &if_equal, &if_notequal, &if_indirect);
        self.string_equal_core(lhs.clone(), lhs_instance_type, rhs.clone(), rhs_instance_type, length,
                             &if_equal, &if_notequal, &if_indirect);

        //BIND(&if_indirect);
        self.goto_if(Rc::new(RefCell::new(true)), &if_indirect);
        {
            let restart = Label::new("restart".to_string());
            //Try to unwrap indirect strings, restart the above attempt on success.
            //MaybeDerefIndirectStrings(&var_left, lhs_instance_type, &var_right,
            //                         rhs_instance_type, &restart);
            self.maybe_deref_indirect_strings(&var_left, lhs_instance_type, &var_right,
                                         rhs_instance_type, &restart);
            //TailCallRuntime(Runtime::kStringEqual, NoContextConstant(), lhs, rhs);

            //BIND(&restart);
            self.goto_if(Rc::new(RefCell::new(true)), &restart);
            //GotoIf(TaggedEqual(var_left.value(), var_right.value()), &if_equal);
            if self.tagged_equal(var_left.clone(), var_right.clone()){
              self.goto_if(Rc::new(RefCell::new(true)), &if_equal);
            }
            self.goto_if(Rc::new(RefCell::new(true)), &start);
        }

        //BIND(&if_equal);
        self.goto_if(Rc::new(RefCell::new(true)), &if_equal);
        //Return(TrueConstant());

        //BIND(&if_notequal);
        self.goto_if(Rc::new(RefCell::new(true)), &if_notequal);
        //Return(FalseConstant());
    }

    fn string_equal_core(
        &self,
        lhs: TNode<String>,
        lhs_instance_type: TNode<Word32T>,
        rhs: TNode<String>,
        rhs_instance_type: TNode<Word32T>,
        length: TNode<IntPtrT>,
        if_equal: &Label,
        if_not_equal: &Label,
        if_indirect: &Label,
    ) {
        //CSA_DCHECK(this, WordEqual(LoadStringLengthAsWord(lhs), length));
        //CSA_DCHECK(this, WordEqual(LoadStringLengthAsWord(rhs), length));
        //
        //// Callers must handle the case where {lhs} and {rhs} refer to the same
        //// String object.
        //CSA_DCHECK(this, TaggedNotEqual(lhs, rhs));
        //
        //// Combine the instance types into a single 16-bit value, so we can check
        //// both of them at once.
        //TNode<Word32T> both_instance_types = Word32Or(
        //    lhs_instance_type, Word32Shl(rhs_instance_type, Int32Constant(8)));
        let both_instance_types: TNode<Word32T> = Rc::new(RefCell::new( (lhs_instance_type.borrow().clone() | (rhs_instance_type.borrow().clone() << 8)) as u32 ));

        // Check if both {lhs} and {rhs} are internalized. Since we already know
        // that they're not the same object, they're not equal in that case.
        //int const kBothInternalizedMask =
        //    kIsNotInternalizedMask | (kIsNotInternalizedMask << 8);
        //int const kBothInternalizedTag = kInternalizedTag | (kInternalizedTag << 8);

        let kBothInternalizedMask: i32 = instance_type::kIsNotInternalizedMask | (instance_type::kIsNotInternalizedMask << 8);
        let kBothInternalizedTag: i32 = instance_type::kInternalizedTag | (instance_type::kInternalizedTag << 8);

        if self.word32_equal(Rc::new(RefCell::new( ((both_instance_types.borrow().clone() & kBothInternalizedMask as u32)) as u32)), Rc::new(RefCell::new(kBothInternalizedTag as u32))) {
          self.goto_if(Rc::new(RefCell::new(true)), if_not_equal);
        }

        //GotoIf(Word32Equal(Word32And(both_instance_types,
        //                             Int32Constant(kBothInternalizedMask)),
        //                  Int32Constant(kBothInternalizedTag)),
        //     if_not_equal);
        //
        //// Check if both {lhs} and {rhs} are direct strings, and that in case of
        //// ExternalStrings the data pointer is cached.
        //static_assert(kUncachedExternalStringTag != 0);
        //static_assert(kIsIndirectStringTag != 0);
        //int const kBothDirectStringMask =
        //    kIsIndirectStringMask | kUncachedExternalStringMask |
        //    ((kIsIndirectStringMask | kUncachedExternalStringMask) << 8);
        let kBothDirectStringMask: i32 = instance_type::kIsIndirectStringMask | instance_type::kUncachedExternalStringMask |
            ((instance_type::kIsIndirectStringMask | instance_type::kUncachedExternalStringMask) << 8);
        if !self.word32_equal(Rc::new(RefCell::new( ((both_instance_types.borrow().clone() & kBothDirectStringMask as u32)) as u32)), Rc::new(RefCell::new(0 as u32))) {
          self.goto_if(Rc::new(RefCell::new(true)), if_indirect);
        }

        //GotoIfNot(Word32Equal(Word32And(both_instance_types,
        //                                 Int32Constant(kBothDirectStringMask)),
        //                       Int32Constant(0)),
        //          if_indirect);
        //
        let if_skip_fast_case = Label::new("if_skip_fast_case".to_string());
        let if_fast_case = Label::new("if_fast_case".to_string());
        let if_oneonebytestring = Label::new("if_oneonebytestring".to_string());
        let if_twotwobytestring = Label::new("if_twotwobytestring".to_string());
        let if_onetwobytestring = Label::new("if_onetwobytestring".to_string());
        let if_twoonebytestring = Label::new("if_twoonebytestring".to_string());
        //
        //// Dispatch based on the {lhs} and {rhs} string encoding.
        //int const kBothStringEncodingMask =
        //    kStringEncodingMask | (kStringEncodingMask << 8);
        //int const kBothExternalStringTag =
        //    kExternalStringTag | (kExternalStringTag << 8);
        //int const kOneOneByteStringTag = kOneByteStringTag | (kOneByteStringTag << 8);
        //int const kTwoTwoByteStringTag = kTwoByteStringTag | (kTwoByteStringTag << 8);
        //int const kOneTwoByteStringTag = kOneByteStringTag | (kTwoByteStringTag << 8);
        let kBothStringEncodingMask: i32 = instance_type::kStringEncodingMask | (instance_type::kStringEncodingMask << 8);
        let kBothExternalStringTag: i32 = instance_type::kExternalStringTag | (instance_type::kExternalStringTag << 8);
        let kOneOneByteStringTag: i32 = instance_type::kOneByteStringTag | (instance_type::kOneByteStringTag << 8);
        let kTwoTwoByteStringTag: i32 = instance_type::kTwoByteStringTag | (instance_type::kTwoByteStringTag << 8);
        let kOneTwoByteStringTag: i32 = instance_type::kOneByteStringTag | (instance_type::kTwoByteStringTag << 8);

        let masked_instance_types: TNode<Word32T> = Rc::new(RefCell::new(((both_instance_types.borrow().clone() & kBothStringEncodingMask as u32)) as u32));
        let both_are_one_byte: TNode<Word32T> = self.word32_equal(masked_instance_types.clone(), Rc::new(RefCell::new(kOneOneByteStringTag as u32)));
        let both_are_two_byte: TNode<Word32T> = self.word32_equal(masked_instance_types.clone(), Rc::new(RefCell::new(kTwoTwoByteStringTag as u32)));

        if self.word32_equal(Rc::new(RefCell::new( ((both_instance_types.borrow().clone() & kBothExternalStringTag as u32)) as u32)), Rc::new(RefCell::new(kBothExternalStringTag as u32))) {
          self.goto_if(Rc::new(RefCell::new(true)), &if_skip_fast_case);
        }
        //If both strings are not external we know that their payload length is
        // kTagged sized. When they have the same type we can compare in chunks. The
        // padding bytes are set to zero.
        //GotoIf(Word32And(both_instance_types, Int32Constant(kBothExternalStringTag)),
        //       &if_skip_fast_case);
        let byte_length: TVariable<IntPtrT> = Rc::new(RefCell::new(length.borrow().clone()));
        if self.word32_equal(both_are_one_byte.clone(), Rc::new(RefCell::new(1 as u32))) {
            self.goto_if(Rc::new(RefCell::new(true)), &if_fast_case);
        }
        //GotoIf(both_are_one_byte, &if_fast_case);
        //byte_length = WordShl(byte_length.value(), IntPtrConstant(1));
        self.goto_if(both_are_two_byte.clone(), &if_fast_case);
        //Branch(both_are_two_byte, &if_fast_case, &if_skip_fast_case);
        self.goto_if(Rc::new(RefCell::new(true)), &if_skip_fast_case);
        //BIND(&if_fast_case);
        //{
        //    StringEqual_FastLoop(lhs, lhs_instance_type, rhs, rhs_instance_type,
        //                         byte_length.value(), if_equal, if_not_equal);
        //}

        //BIND(&if_skip_fast_case);
        self.goto_if(Rc::new(RefCell::new(true)), &if_skip_fast_case);
        //{
        //    GotoIf(both_are_one_byte, &if_oneonebytestring);
        //    GotoIf(both_are_two_byte, &if_twotwobytestring);
        //    Branch(
        //        Word32Equal(masked_instance_types, Int32Constant(kOneTwoByteStringTag)),
        //        &if_onetwobytestring, &if_twoonebytestring);
        //}

        //BIND(&if_oneonebytestring);
        self.goto_if(Rc::new(RefCell::new(true)), &if_oneonebytestring);
        //{
        //    StringEqual_Loop(lhs, lhs_instance_type, MachineType::Uint8(), rhs,
        //                     rhs_instance_type, MachineType::Uint8(), length, if_equal,
        //                     if_not_equal);
        //}

        //BIND(&if_twotwobytestring);
        self.goto_if(Rc::new(RefCell::new(true)), &if_twotwobytestring);
        //{
        //    StringEqual_Loop(lhs, lhs_instance_type, MachineType::Uint16(), rhs,
        //                     rhs_instance_type, MachineType::Uint16(), length, if_equal,
        //                     if_not_equal);
        //}

        //