// Converted from V8 C++ source files:
// Header: regexp-macro-assembler.h
// Implementation: regexp-macro-assembler.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod regexp_macro_assembler {
    use std::rc::Rc;
    use std::sync::{Arc, Mutex, RwLock};

    use crate::base::strings::uc32;
    use crate::execution::frame_constants::StackFrameConstants;
    use crate::init::bootstrapper::Heap;
    use crate::logging::code_events::RegExpFlags;
    use crate::objects::fixed_array::FixedUInt16Array;
    use crate::regexp::regexp_ast::CharacterRange;
    use crate::regexp::regexp::RegExp;
    use crate::sandbox::code_pointer_table::Address;
    use crate::strings::string_case::DisallowGarbageCollection;
    use crate::strings::string_stream::ByteArray;
    use crate::strings::uri::String;
    use crate::strings::unicode_inl::is;
    use crate::torque::cfg::Label;
    use crate::v8::internal::Isolate;
    use crate::v8::internal::Zone;
    use crate::v8::internal::WriteIterator;
    use crate::v8::V8_EXPORT_PRIVATE;
    use crate::v8::{internal, Local};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use crate::objects::heap_object::HeapObject;
    use crate::codegen::instruction_stream::InstructionStream;
    use crate::objects::string::ConsString;
    use crate::objects::string::SlicedString;
    use crate::objects::string::ThinString;
    use crate::objects::string::StringShape;
    use crate::objects::string::IsExternalString;
    use crate::objects::string::IsSeqString;
    use crate::codegen::code::Code;
    use crate::regexp::regexp_data::IrRegExpData;
    use crate::objects::object::Object;
    use crate::execution::stack_limit::StackLimitCheck;
    use crate::objects::name::Name;
    use crate::codegen::generated_code::GeneratedCode;
    use crate::regexp::regexp_stack::RegExpStack;

    pub const K_LEAD_SURROGATE_START: uc32 = 0xd800;
    pub const K_LEAD_SURROGATE_END: uc32 = 0xdbff;
    pub const K_TRAIL_SURROGATE_START: uc32 = 0xdc00;
    pub const K_TRAIL_SURROGATE_END: uc32 = 0xdfff;
    pub const K_NON_BMP_START: uc32 = 0x10000;
    pub const K_NON_BMP_END: uc32 = 0x10ffff;

    pub const K_MAX_REGISTER_COUNT: i32 = 1 << 16;
    pub const K_MAX_REGISTER: i32 = K_MAX_REGISTER_COUNT - 1;
    pub const K_MAX_CAPTURES: i32 = (K_MAX_REGISTER - 1) / 2;
    pub const K_MAX_CP_OFFSET: i32 = (1 << 15) - 1;
    pub const K_MIN_CP_OFFSET: i32 = -(1 << 15);

    pub const K_TABLE_SIZE_BITS: i32 = 7;
    pub const K_TABLE_SIZE: i32 = 1 << K_TABLE_SIZE_BITS;
    pub const K_TABLE_MASK: i32 = K_TABLE_SIZE - 1;

    pub const K_USE_CHARACTERS_VALUE: i32 = -1;

    pub struct RegExpMacroAssembler {
        slow_safe_compiler_: bool,
        backtrack_limit_: u32,
        can_fallback_: bool,
        global_mode_: GlobalMode,
        isolate_: *mut Isolate,
        zone_: *mut Zone,
    }

    impl RegExpMacroAssembler {
        pub fn new(isolate: *mut Isolate, zone: *mut Zone) -> Self {
            RegExpMacroAssembler {
                slow_safe_compiler_: false,
                backtrack_limit_: crate::regexp::regexp::JSRegExp::K_NO_BACKTRACK_LIMIT,
                can_fallback_: false,
                global_mode_: GlobalMode::NOT_GLOBAL,
                isolate_: isolate,
                zone_: zone,
            }
        }

        pub fn has_backtrack_limit(&self) -> bool {
            self.backtrack_limit_ != crate::regexp::regexp::JSRegExp::K_NO_BACKTRACK_LIMIT
        }

        pub fn backtrack_limit(&self) -> u32 {
            self.backtrack_limit_
        }

        pub fn can_fallback(&self) -> bool {
            self.can_fallback_
        }

        pub fn set_slow_safe(&mut self, ssc: bool) {
            self.slow_safe_compiler_ = ssc;
        }

        pub fn slow_safe(&self) -> bool {
            self.slow_safe_compiler_
        }

        pub fn set_backtrack_limit(&mut self, backtrack_limit: u32) {
            self.backtrack_limit_ = backtrack_limit;
        }

        pub fn set_can_fallback(&mut self, val: bool) {
            self.can_fallback_ = val;
        }

        pub fn set_global_mode(&mut self, mode: GlobalMode) {
            self.global_mode_ = mode;
        }

        pub fn global_(&self) -> bool {
            self.global_mode_ != GlobalMode::NOT_GLOBAL
        }

        pub fn global_with_zero_length_check(&self) -> bool {
            self.global_mode_ == GlobalMode::GLOBAL || self.global_mode_ == GlobalMode::GLOBAL_UNICODE
        }

        pub fn global_unicode(&self) -> bool {
            self.global_mode_ == GlobalMode::GLOBAL_UNICODE
        }

        pub fn isolate(&self) -> *mut Isolate {
            self.isolate_
        }

        pub fn zone(&self) -> *mut Zone {
            self.zone_
        }

        pub fn check_not_in_surrogate_pair(&mut self, cp_offset: i32, on_failure: *mut Label) {
            let mut ok = Label::new();
            self.LoadCurrentCharacter(cp_offset, &mut ok, false, 1, K_USE_CHARACTERS_VALUE);
            self.CheckCharacterNotInRange(K_TRAIL_SURROGATE_START as u16, K_TRAIL_SURROGATE_END as u16, &mut ok);
            self.LoadCurrentCharacter(cp_offset - 1, &mut ok, false, 1, K_USE_CHARACTERS_VALUE);
            self.CheckCharacterInRange(K_LEAD_SURROGATE_START as u16, K_LEAD_SURROGATE_END as u16, unsafe { &mut *on_failure });
            self.Bind(&mut ok);
        }

        pub fn check_position(&mut self, cp_offset: i32, on_outside_input: *mut Label) {
            self.LoadCurrentCharacter(cp_offset, unsafe { &mut *on_outside_input }, true, 1, K_USE_CHARACTERS_VALUE);
        }

        pub fn LoadCurrentCharacter(&mut self, cp_offset: i32, on_end_of_input: *mut Label, check_bounds: bool, characters: i32, eats_at_least: i32) {
            let eats_at_least = if eats_at_least == K_USE_CHARACTERS_VALUE {
                characters
            } else {
                eats_at_least
            };

            self.LoadCurrentCharacterImpl(cp_offset, on_end_of_input, check_bounds, characters, eats_at_least);
        }

         // The implementation must be able to handle at least:
         // The maximal number of pushes between stack checks. Users must supply
         // kCheckStackLimit flag to push operations (instead of kNoStackLimitCheck)
         // at least once for every stack_limit() pushes that are executed.
         pub fn stack_limit_slack_slot_count(&mut self) -> i32 {
             0
         }
         pub fn AbortedCodeGeneration(&mut self) {}
         pub fn AdvanceCurrentPosition(&mut self, _by: i32) {}
         pub fn AdvanceRegister(&mut self, _reg: i32, _by: i32) {}
         // Continues execution from the position pushed on the top of the backtrack
         // stack by an earlier PushBacktrack(Label*).
         pub fn Backtrack(&mut self) {}
         pub fn Bind(&mut self, _label: &mut Label) {}
         // Dispatch after looking the current character up in a 2-bits-per-entry
         // map.  The destinations vector has up to 4 labels.
         pub fn CheckCharacter(&mut self, _c: u32, _on_equal: *mut Label) {}
         // Bitwise and the current character with the given constant and then
         // check for a match with c.
         pub fn CheckCharacterAfterAnd(&mut self, _c: u32, _and_with: u32, _on_equal: *mut Label) {}
         pub fn CheckCharacterGT(&mut self, _limit: u16, _on_greater: *mut Label) {}
         pub fn CheckCharacterLT(&mut self, _limit: u16, _on_less: *mut Label) {}
         pub fn CheckGreedyLoop(&mut self, _on_tos_equals_current_position: *mut Label) {}
         pub fn CheckAtStart(&mut self, _cp_offset: i32, _on_at_start: *mut Label) {}
         pub fn CheckNotAtStart(&mut self, _cp_offset: i32, _on_not_at_start: *mut Label) {}
         pub fn CheckNotBackReference(&mut self, _start_reg: i32, _read_backward: bool, _on_no_match: *mut Label) {}
         pub fn CheckNotBackReferenceIgnoreCase(&mut self, _start_reg: i32, _read_backward: bool, _unicode: bool, _on_no_match: *mut Label) {}
         // Check the current character for a match with a literal character.  If we
         // fail to match then goto the on_failure label.  End of input always
         // matches.  If the label is nullptr then we should pop a backtrack address
         // off the stack and go to that.
         pub fn CheckNotCharacter(&mut self, _c: u32, _on_not_equal: *mut Label) {}
         pub fn CheckNotCharacterAfterAnd(&mut self, _c: u32, _and_with: u32, _on_not_equal: *mut Label) {}
         // Subtract a constant from the current character, then and with the given
         // constant and then check for a match with c.
         pub fn CheckNotCharacterAfterMinusAnd(&mut self, _c: uc32, _minus: uc32, _and_with: uc32, _on_not_equal: *mut Label) {}
         pub fn CheckCharacterInRange(&mut self, _from: u16, _to: u16, _on_in_range: *mut Label) {}
         pub fn CheckCharacterNotInRange(&mut self, _from: u16, _to: u16, _on_not_in_range: *mut Label) {}
         // Returns true if the check was emitted, false otherwise.
         pub fn CheckCharacterInRangeArray(&mut self, _ranges: *const ZoneList<CharacterRange>, _on_in_range: *mut Label) -> bool {
             false
         }
         pub fn CheckCharacterNotInRangeArray(&mut self, _ranges: *const ZoneList<CharacterRange>, _on_not_in_range: *mut Label) -> bool {
             false
         }
         // The current character (modulus the kTableSize) is looked up in the byte
         // array, and if the found byte is non-zero, we jump to the on_bit_set label.
         pub fn CheckBitInTable(&mut self, _table: Handle<ByteArray>, _on_bit_set: *mut Label) {}
         pub fn SkipUntilBitInTable(&mut self, _cp_offset: i32, _table: Handle<ByteArray>, _nibble_table: Handle<ByteArray>, _advance_by: i32) {}
         pub fn SkipUntilBitInTableUseSimd(&mut self, _advance_by: i32) -> bool {
             false
         }
         // Check whether a standard/default character class matches the current
         // character. Returns false if the type of special character class does
         // not have custom support.
         // May clobber the current loaded character.
         pub fn CheckSpecialClassRanges(&mut self, _type: StandardCharacterSet, _on_no_match: *mut Label) -> bool {
             false
         }
         // Control-flow integrity:
         // Define a jump target and bind a label.
         pub fn BindJumpTarget(&mut self, _label: &mut Label) {}
         pub fn Fail(&mut self) {}
         pub fn GoTo(&mut self, _label: *mut Label) {}
         // Check whether a register is >= a given constant and go to a label if it
         // is.  Backtracks instead if the label is nullptr.
         pub fn IfRegisterGE(&mut self, _reg: i32, _comparand: i32, _if_ge: *mut Label) {}
         // Check whether a register is < a given constant and go to a label if it is.
         // Backtracks instead if the label is nullptr.
         pub fn IfRegisterLT(&mut self, _reg: i32, _comparand: i32, _if_lt: *mut Label) {}
         // Check whether a register is == to the current position and go to a
         // label if it is.
         pub fn IfRegisterEqPos(&mut self, _reg: i32, _if_eq: *mut Label) {}
         pub fn LoadCurrentCharacterImpl(&mut self, _cp_offset: i32, _on_end_of_input: *mut Label, _check_bounds: bool, _characters: i32, _eats_at_least: i32) {}
         pub fn PopCurrentPosition(&mut self) {}
         pub fn PopRegister(&mut self, _register_index: i32) {}
         // Pushes the label on the backtrack stack, so that a following Backtrack
         // will go to this label. Always checks the backtrack stack limit.
         pub fn PushBacktrack(&mut self, _label: *mut Label) {}
         pub fn PushCurrentPosition(&mut self) {}
         pub fn PushRegister(&mut self, _register_index: i32, _check_stack_limit: StackCheckFlag) {}
         pub fn ReadCurrentPositionFromRegister(&mut self, _reg: i32) {}
         pub fn ReadStackPointerFromRegister(&mut self, _reg: i32) {}
         pub fn SetCurrentPositionFromEnd(&mut self, _by: i32) {}
         pub fn SetRegister(&mut self, _register_index: i32, _to: i32) {}
         // Return whether the matching (with a global regexp) will be restarted.
         pub fn Succeed(&mut self) -> bool {
             false
         }
         pub fn WriteCurrentPositionToRegister(&mut self, _reg: i32, _cp_offset: i32) {}
         pub fn ClearRegisters(&mut self, _reg_from: i32, _reg_to: i32) {}
         pub fn WriteStackPointerToRegister(&mut self, _reg: i32) {}
         pub fn ImplementationToString(_impl: IrregexpImplementation) -> &'static str {
             "Implementation"
         }
         pub fn CaseInsensitiveCompareNonUnicode(_byte_offset1: Address, _byte_offset2: Address, _byte_length: usize, _isolate: *mut Isolate) -> i32 {
             0
         }
         pub fn CaseInsensitiveCompareUnicode(_byte_offset1: Address, _byte_offset2: Address, _byte_length: usize, _isolate: *mut Isolate) -> i32 {
             0
         }
         pub fn word_character_map_address() -> Address {
             0
         }
         pub fn IsCharacterInRangeArray(_current_char: u32, _raw_byte_array: Address) -> u32 {
             0
         }

         pub fn GetCode(&mut self, _source: DirectHandle<String>, _flags: RegExpFlags) -> Result<DirectHandle<HeapObject>, String> {
             Err("".to_string())
         }
         pub fn CanReadUnaligned(&self) -> bool {false}
         pub fn Implementation(&mut self) -> IrregexpImplementation {
             IrregexpImplementation::kBytecodeImplementation
         }
    }

    #[derive(PartialEq)]
    pub enum GlobalMode {
        NOT_GLOBAL,
        GLOBAL_NO_ZERO_LENGTH_CHECK,
        GLOBAL,
        GLOBAL_UNICODE,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum StackCheckFlag {
        K_NO_STACK_LIMIT_CHECK = 0,
        K_CHECK_STACK_LIMIT = 1,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum IrregexpImplementation {
        kIA32Implementation,
        kARMImplementation,
        kARM64Implementation,
        kMIPSImplementation,
        kLOONG64Implementation,
        kRISCVImplementation,
        kRISCV32Implementation,
        kS390Implementation,
        kPPCImplementation,
        kX64Implementation,
        kBytecodeImplementation,
    }

    pub struct NativeRegExpMacroAssembler {
        base: RegExpMacroAssembler,
        range_array_cache_: ZoneUnorderedMap<u32, IndirectHandle<FixedUInt16Array>>,
    }

    impl NativeRegExpMacroAssembler {
        pub fn new(isolate: *mut Isolate, zone: *mut Zone) -> Self {
            NativeRegExpMacroAssembler {
                base: RegExpMacroAssembler::new(isolate, zone),
                range_array_cache_: ZoneUnorderedMap::new(zone),
            }
        }

        pub fn GetOrAddRangeArray(&mut self, ranges: *const ZoneList<CharacterRange>) -> Handle<ByteArray> {
            let hash = {
                let ranges_ref = unsafe { &*ranges };
                let mut hasher = DefaultHasher::new();
                for i in 0..ranges_ref.length() {
                    let r = ranges_ref.at(i);
                    r.from().hash(&mut hasher);
                    r.to().hash(&mut hasher);
                }
                hasher.finish() as u32
            };

            if self.range_array_cache_.contains_key(&hash) {
                let range_array = self.range_array_cache_.get(&hash).unwrap();
                let ranges_ref = unsafe { &*ranges };
                let is_equal = {
                    let rhs = range_array;
                    let rhs_length = rhs.length();
                    let lhs = ranges_ref;

                    if rhs_length != range_array_length_for(lhs) {
                        false
                    } else {
                        (0..lhs.length()).all(|i| {
                            let r = lhs.at(i);
                            if rhs.get(i * 2 + 0) != r.from() {
                                return false;
                            }
                            if i * 2 + 1 == rhs_length {
                                return true;
                            }
                            if rhs.get(i * 2 + 1) != r.to() + 1 {
                                return false;
                            }
                            true
                        })
                    }
                };
                if is_equal {
                    return range_array.clone();
                }
            }

            let isolate = unsafe { &mut *self.base.isolate_ };
            let range_array = make_range_array(isolate, unsafe { &*ranges });
            self.range_array_cache_.insert(hash, range_array.clone());
            range_array
        }

        pub fn word_character_map_address() -> Address {
            unsafe { std::mem::transmute(&WORD_CHARACTER_MAP[0]) }
        }

        pub fn ExecuteForTesting(
            input: Tagged<String>,
            start_offset: i32,
            input_start: *const u8,
            input_end: *const u8,
            output: *mut i32,
            output_size: i32,
            isolate: *mut Isolate,
            regexp: Tagged<RegExp>,
        ) -> i32 {
            let data = regexp.data(unsafe {&mut *isolate});
            if let Some(ir_regexp_data) = data.try_cast::<IrRegExpData>() {
                return NativeRegExpMacroAssembler::Execute(
                    input,
                    start_offset,
                    input_start,
                    input_end,
                    output,
                    output_size,
                    isolate,
                    ir_regexp_data.unchecked_cast(),
                );
            } else {
                return FAILURE;
            }
        }
        // Returns a {Result} sentinel, or the number of successful matches.
        pub fn Match(
            regexp_data: DirectHandle<IrRegExpData>,
            subject: DirectHandle<String>,
            offsets_vector: *mut i32,
            offsets_vector_length: i32,
            previous_index: i32,
            isolate: *mut Isolate,
        ) -> i32 {
            if !subject.is_flat() {
                return FAILURE;
            }
            if previous_index < 0 || previous_index > subject.length() {
                return FAILURE;
            }

            let mut subject_ptr = *subject;
            let start_offset = previous_index;
            let mut char_length = subject_ptr.length() as i32 - start_offset;
            let mut slice_offset = 0;

            if subject_ptr.is_cons() {
                let cons_string = subject_ptr.unchecked_cast::<ConsString>();
                if cons_string.second().length() != 0 {
                    return FAILURE;
                }
                subject_ptr = cons_string.first().unchecked_cast();
            } else if subject_ptr.is_sliced() {
                let slice = subject_ptr.unchecked_cast::<SlicedString>();
                subject_ptr = slice.parent().unchecked_cast();
                slice_offset = slice.offset() as i32;
            }

            if subject_ptr.is_thin() {
                subject_ptr = subject_ptr.unchecked_cast::<ThinString>().actual().unchecked_cast();
            }
            let is_one_byte = subject_ptr.is_one_byte_representation();
            if !subject_ptr.is_external_string() && !subject_ptr.is_seq_string() {
                return FAILURE;
            }
            let char_size_shift = if is_one_byte { 0 } else { 1 };

            let no_gc = DisallowGarbageCollection::new();
            let input_start = subject_ptr.address_of_character_at((start_offset + slice_offset) as usize, &no_gc) as *const u8;
            let byte_length = char_length << char_size_shift;
            let input_end = unsafe { input_start.add(byte_length as usize) };

            let isolate_ref = unsafe { &mut *isolate };
            NativeRegExpMacroAssembler::Execute(
                subject_ptr,
                start_offset,
                input_start,
                input_end,
                unsafe { &mut *offsets_vector },
                offsets_vector_length,
                isolate,
                *regexp_data,
            )
        }
        // Returns a {Result} sentinel, or the number of successful matches.
        pub fn Execute(
            input: Tagged<String>,
            start_offset: i32,
            input_start: *const u8,
            input_end: *const u8,
            output: *mut i32,
            output_size: i32,
            isolate: *mut Isolate,
            regexp_data: Tagged<IrRegExpData>,
        ) -> i32 {
            let is_one_byte = input.is_one_byte_representation();
            let code = regexp_data.code(unsafe { &mut *isolate }, is_one_byte);
            let call_origin = RegExp::CallOrigin::KFromRuntime;

            type RegexpMatcherSig =
                unsafe extern "C" fn(Address, i32, *const u8, *const u8, *mut i32, i32, i32, *mut Isolate, Address) -> i32;

            let fn_ptr = code.get_regexp_matcher::<RegexpMatcherSig>(unsafe { &mut *isolate });

            let result = unsafe {
                fn_ptr(
                    input.ptr() as Address,
                    start_offset,
                    input_start,
                    input_end,
                    output,
                    output_size,
                    call_origin as i32,
                    isolate,
                    regexp_data.ptr() as Address,
                )
            };
            
            if result == EXCEPTION && !unsafe { &mut *isolate }.has_exception() {
                let allow_allocation = AllowGarbageCollection::new();
                unsafe { &mut *isolate }.stack_overflow();
            }

            result
        }

        pub fn GrowStack(isolate: *mut Isolate) -> Address {
            let no_gc = DisallowGarbageCollection::new();
            let regexp_stack = unsafe { &mut *isolate }.regexp_stack();
            let old_size = regexp_stack.memory_size();

            let new_stack_base = regexp_stack.EnsureCapacity(old_size * 2);
            if new_stack_base == 0 {
                return 0;
            }

            regexp_stack.stack_pointer() as Address
        }
        pub fn CheckStackGuardState(
            isolate: *mut Isolate,
            start_index: i32,
            call_origin: RegExp::CallOrigin,
            return_address: *mut Address,
            re_code: Tagged<InstructionStream>,
            subject: *mut Address,
            input_start: *mut *const u8,
            input_end: *mut *const u8,
            gap: usize,
        ) -> i32 {
            let no_gc = DisallowGarbageCollection::new();

            let old_pc = unsafe { PointerAuthentication::AuthenticatePC(return_address, 0) as Address };
            if !(re_code.instruction_start() <= old_pc && old_pc <= re_code.code(crate::codegen::code::CodeFlags::kAcquireLoad).instruction_end()) {
                println!("CheckStackGuardState::Error!");
                return EXCEPTION;
            }
            let mut check = StackLimitCheck::new(unsafe { &mut *isolate });
            let js_has_overflowed = check.JsHasOverflowed(gap);

            if call_origin == RegExp::CallOrigin::KFromJs {
                if js_has_overflowed {
                    return EXCEPTION;
                } else if check.InterruptRequested() {
                    return RETRY;
                } else {
                    return 0;
                }
            }

            let mut handles = HandleScope::new(unsafe { &mut *isolate });
            let code_handle = DirectHandle::new(re_code, unsafe { &mut *isolate });
            let subject_handle = DirectHandle::new(Tagged::<Object>::from_ptr(unsafe { *(*subject as *mut Object) }).unchecked_cast::<String>(), unsafe { &mut *isolate });
            let is_one_byte = subject_handle.is_one_byte_representation_underneath();
            let mut return_value = 0;
            {
                let no_gc_mole = DisableGCMole::new();
                if js_has_overflowed {
                    let yes_gc = AllowGarbageCollection::new();
                    unsafe { &mut *isolate }.stack_overflow();
                    return_value = EXCEPTION;
                } else if check.InterruptRequested() {
                    let yes_gc = AllowGarbageCollection::new();
                    let result = unsafe { &mut *isolate }.stack_guard().HandleInterrupts();
                    if result.is_exception(unsafe { &mut *isolate }) {
                        return_value = EXCEPTION;
                    }
                }

                if code_handle.address() != re_code.ptr() as Address {
                    let delta = code_handle.address() as isize - re_code.ptr() as isize;
                    let new_pc = old_pc as isize + delta;
                    unsafe { PointerAuthentication::ReplacePC(return_address, new_pc as Address, 0) };
                }
            }

            if return_value == 0 {
                if subject_handle.is_one_byte_representation_underneath() != is_one_byte {
                    return_value = RETRY;
                } else {
                    unsafe { *subject = subject_handle.ptr() as Address };
                    let byte_length = unsafe { *input_end as usize - *input_start as usize };
                    unsafe { *input_start = subject_handle.address_of_character_at(start_index as usize, &no_gc) as *const u8 };
                    unsafe { *input_end = (*input_start as usize + byte_length) as *const u8 };
                }
            }

            return return_value;
        }
    }
    impl std::ops::Deref for NativeRegExpMacroAssembler {
        type Target = RegExpMacroAssembler;

        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }
    impl std::ops::DerefMut for NativeRegExpMacroAssembler {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.base
        }
    }

    impl NativeRegExpMacroAssembler {
        pub fn LoadCurrentCharacterImpl(
            &mut self,
            cp_offset: i32,
            on_end_of_input: *mut Label,
            check_bounds: bool,
            characters: i32,
            eats_at_least: i32,
        ) {
            if eats_at_least < characters {
                return;
            }
            if !(K_MIN_CP_OFFSET <= cp_offset && cp_offset <= K_MAX_CP_OFFSET) {
                return;
            }
            if check_bounds {
                if cp_offset >= 0 {
                    self.check_position(cp_offset + eats_at_least - 1, on_end_of_input);
                } else {
                    self.check_position(cp_offset, on_end_of_input);
                }
            }
            self.LoadCurrentCharacterUnchecked(cp_offset, characters);
        }
        pub fn CanReadUnaligned(&self) -> bool {
            v8_flags::get_enable_regexp_unaligned_accesses() && !self.slow_safe()
        }
        pub fn LoadCurrentCharacterUnchecked(&mut self, _cp_offset: i32, _character_count: i32) {}

        pub fn Implementation(&mut self) -> IrregexpImplementation {
            IrregexpImplementation::kBytecodeImplementation
        }
    }

    const WORD_CHARACTER_MAP: [u8; 256] = [
        0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
        0xFFu8, 0xFFu8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0xFFu8, 0xFFu8,
        0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
        0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
        0xFFu8, 0xFFu8, 0xFFu8, 0
