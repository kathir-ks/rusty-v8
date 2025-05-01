#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

// use std::rc::Rc;
// use std::cell::RefCell;

// Placeholder for V8_TARGET_ARCH_PPC64 check
const V8_TARGET_ARCH_PPC64: bool = true;

#[cfg(target_arch = "powerpc64")]
mod regexp_ppc {

    use std::ptr::null_mut;
    use std::mem::size_of;

    // use crate::codegen::macro_assembler::MacroAssembler; // Assuming a similar structure in Rust
    // use crate::codegen::ppc::assembler_ppc_inl::AssemblerPPC; // Assuming a similar structure in Rust
    // use crate::heap::factory::Factory; // Assuming a similar structure in Rust
    // use crate::logging::log::Log; // Assuming a similar structure in Rust
    // use crate::objects::code_inl::Code; // Assuming a similar structure in Rust
    // use crate::regexp::regexp_stack::RegExpStack; // Assuming a similar structure in Rust
    // use crate::snapshot::embedded::embedded_data_inl::EmbeddedData; // Assuming a similar structure in Rust
    // use crate::strings::string::String; // Assuming a similar structure in Rust

    //use v8::internal::wasm::AsmJsOffset;

    // Placeholder types, replace with actual implementations
    type Isolate = usize;
    type Zone = usize;
    type Handle<T> = *mut T;
    type ByteArray = u8;
    type DirectHandle<T> = *mut T;
    type RegExpFlags = u32;
    type AbstractCode = usize;
    type InstructionStream = usize;
    type Object = usize;
    type Code = usize;
    type RelocInfo = usize;
    type Address = usize;
    type Builtin = usize;
    type Operand = i64;
    type Register = u32;
    type Label = u32;
    type CRegister = u32;
    type Condition = u32;
    type ExternalReference = u32;
    type ZoneList<T> = Vec<T>;
    type CharacterRange = u32;
    type StackFrame = u32;
    type RegList = u64;

    const kSystemPointerSize: usize = 8;
    const kIntSize: usize = 4;

    macro_rules! DCHECK_EQ {
        ($left:expr, $right:expr) => {
            assert_eq!($left, $right);
        };
    }

    macro_rules! DCHECK_LE {
        ($left:expr, $right:expr) => {
            assert!($left <= $right);
        };
    }

    macro_rules! DCHECK_GT {
        ($left:expr, $right:expr) => {
            assert!($left > $right);
        };
    }

    // Placeholder enum
    #[derive(Debug, PartialEq)]
    enum Mode {
        LATIN1,
        UC16,
    }

    // Placeholder const
    const FAILURE: i32 = -1;
    const SUCCESS: i32 = 1;
    const EXCEPTION: i32 = -2;
    const FALLBACK_TO_EXPERIMENTAL: i32 = -3;
    const kTableSize: usize = 256;
    const kTableMask: usize = 255;

    // Placeholder function
    fn is_int16(value: i32) -> bool {
        value >= i16::MIN as i32 && value <= i16::MAX as i32
    }

    // Placeholder definitions
    const r0: Register = 0;
    const r3: Register = 3;
    const r4: Register = 4;
    const r5: Register = 5;
    const r6: Register = 6;
    const r7: Register = 7;
    const r8: Register = 8;
    const r9: Register = 9;
    const r10: Register = 10;
    const r12: Register = 12;
    const r25: Register = 25;
    const r26: Register = 26;
    const r27: Register = 27;
    const r28: Register = 28;
    const r29: Register = 29;
    const r30: Register = 30;
    const r31: Register = 31;
    const sp: Register = 1;
    const ip: Register = 12; // Instruction pointer

    const eq: Condition = 0;
    const ne: Condition = 1;
    const lt: Condition = 2;
    const gt: Condition = 3;
    const le: Condition = 4;
    const ge: Condition = 5;
    const al: Condition = 6; // Always

    const LeaveOE: u32 = 0;
    const SetRC: u32 = 1;

    const kRegExpCalleeSaved: RegList = (1 << 25) | (1 << 26) | (1 << 27) | (1 << 28) | (1 << 29) | (1 << 30) | (1 << 31) | (1 << 12);

    const StackFrame_IRREGEXP: StackFrame = 1;

    fn StackFrame_TypeToMarker(frame_type: StackFrame) -> i32 {
        frame_type as i32
    }

    const OFFSET_OF_DATA_START_ByteArray: usize = 0; // Placeholder

    const kHeapObjectTag: usize = 0;

    // Offsets into the frame.
    const kFrameTypeOffset: i32 = -4;
    const kIsolateOffset: i32 = -8;
    const kDirectCallOffset: i32 = -12;
    const kStackAreaBaseOffset: i32 = -16;
    const kCaptureArraySizeOffset: i32 = -20;
    const kRegisterOutputOffset: i32 = -24;
    const kInputEndOffset: i32 = -28;
    const kInputStartOffset: i32 = -32;
    const kStartIndexOffset: i32 = -36;
    const kInputStringOffset: i32 = -40;
    const kSuccessfulCapturesOffset: i32 = -44;
    const kStringStartMinusOneOffset: i32 = -48;
    const kAtStartOffset: i32 = -52;
    const kRegisterZeroOffset: i32 = -56;
    const kBacktrackCountOffset: i32 = -56; //Fixme

    const kNumOutputRegistersOffset: i32 = 1234; // Dummy offset

    struct MacroAssembler {}
    impl MacroAssembler {
        fn new(_isolate: Isolate, _code_object_required: CodeObjectRequired, _buffer: AssemblerBuffer) -> Self {
            MacroAssembler{}
        }
    }

    #[derive(Debug, PartialEq)]
    enum CodeObjectRequired {
        kYes,
        kNo
    }

    struct AssemblerBuffer {}
    impl AssemblerBuffer {
        fn new(size: usize) -> Self {
            AssemblerBuffer{}
        }
    }

    struct NoRootArrayScope<'a> {
        masm_: &'a MacroAssembler,
    }

    impl<'a> NoRootArrayScope<'a> {
        fn new(masm: &'a MacroAssembler) -> Self {
            NoRootArrayScope { masm_: masm }
        }
    }

    struct FrameScope<'a> {
        masm_: &'a MacroAssembler,
        _frame_type: StackFrame,
    }
    impl<'a> FrameScope<'a> {
        fn new(masm: &'a MacroAssembler, frame_type: StackFrame) -> Self {
            FrameScope { masm_: masm, _frame_type: frame_type }
        }
    }

    struct MemOperand {
        base: Register,
        offset: i32,
    }

    impl MemOperand {
        fn new(base: Register, offset: i32) -> Self {
            MemOperand { base, offset }
        }
    }

    macro_rules! ACCESS_MASM {
        ($masm_:expr) => {
             $masm_
        };
    }

    pub struct RegExpMacroAssemblerPPC {
        masm_: Box<MacroAssembler>,
        no_root_array_scope_: NoRootArrayScope<'static>,
        mode_: Mode,
        num_registers_: i32,
        num_saved_registers_: i32,
        entry_label_: Label,
        start_label_: Label,
        success_label_: Label,
        backtrack_label_: Label,
        exit_label_: Label,
        internal_failure_label_: Label,
        check_preempt_label_: Label,
        stack_overflow_label_: Label,
        fallback_label_: Label,
        isolate_: Isolate,
        zone_: Zone,
        backtrack_limit_: i32,
        has_backtrack_limit_: bool,
        global_: bool,
        global_with_zero_length_check_: bool,
        global_unicode_: bool,
        can_fallback_: bool,
    }

    impl RegExpMacroAssemblerPPC {
        pub const kRegExpCodeSize: usize = 4096; // Example size

        pub fn new(isolate: Isolate, zone: Zone, mode: Mode, registers_to_save: i32) -> Self {
            DCHECK_EQ!(0, registers_to_save % 2);
            let mut masm = Box::new(MacroAssembler::new(isolate, CodeObjectRequired::kYes, AssemblerBuffer::new(Self::kRegExpCodeSize)));
            let no_root_array_scope_ = NoRootArrayScope::new(&masm);

            let mut result = RegExpMacroAssemblerPPC {
                masm_: masm,
                no_root_array_scope_: no_root_array_scope_,
                mode_: mode,
                num_registers_: registers_to_save,
                num_saved_registers_: registers_to_save,
                entry_label_: 0, // Replace with actual label initialization
                start_label_: 0, // Replace with actual label initialization
                success_label_: 0, // Replace with actual label initialization
                backtrack_label_: 0, // Replace with actual label initialization
                exit_label_: 0, // Replace with actual label initialization
                internal_failure_label_: 0, // Replace with actual label initialization
                check_preempt_label_: 0,
                stack_overflow_label_: 0,
                fallback_label_: 0,
                isolate_: isolate,
                zone_: zone,
                backtrack_limit_: 0,
                has_backtrack_limit_: false,
                global_: false,
                global_with_zero_length_check_: false,
                global_unicode_: false,
                can_fallback_: false,
            };

            let masm_ref = &mut result.masm_;
            //__ACCESS_MASM!(masm_ref).b(&result.entry_label_);
            result.b(&result.entry_label_, masm_ref); // Corrected usage

            //__ACCESS_MASM!(masm_ref).bind(&result.internal_failure_label_);
            result.bind(&result.internal_failure_label_, masm_ref); // Corrected usage
            //__ACCESS_MASM!(masm_ref).li(r3, Operand(FAILURE));
            result.li(r3, Operand(FAILURE as i64), masm_ref);  // Corrected usage
            //__ACCESS_MASM!(masm_ref).Ret();
            result.Ret(masm_ref);  // Corrected usage
            //__ACCESS_MASM!(masm_ref).bind(&result.start_label_);
            result.bind(&result.start_label_, masm_ref); // Corrected usage

            result
        }

        fn b(&mut self, label: &Label, masm_: &mut Box<MacroAssembler>) {
            // Implementation for branching
        }

        fn bind(&mut self, label: &Label, masm_: &mut Box<MacroAssembler>) {
            // Implementation for binding a label
        }

        fn li(&mut self, reg: Register, operand: Operand, masm_: &mut Box<MacroAssembler>) {
            // Implementation for loading immediate value
        }

        fn Ret(&mut self, masm_: &mut Box<MacroAssembler>) {
            // Implementation for return
        }

        fn stack_limit_slack_slot_count(&self) -> i32 {
            //RegExpStack::kStackLimitSlackSlotCount
            4 //Dummy Value
        }

        fn AdvanceCurrentPosition(&mut self, by: i32, masm_: &mut Box<MacroAssembler>) {
            if by != 0 {
                if is_int16(by * self.char_size()) {
                   // __ACCESS_MASM!(masm_).addi(self.current_input_offset(), self.current_input_offset(), Operand(by * self.char_size()));
                } else {
                    //__ACCESS_MASM!(masm_).mov(r0, Operand(by * self.char_size()));
                    //__ACCESS_MASM!(masm_).add(self.current_input_offset(), r0, self.current_input_offset());
                }
            }
        }

        fn AdvanceRegister(&mut self, reg: i32, by: i32, masm_: &mut Box<MacroAssembler>) {
            DCHECK_LE!(0, reg);
            DCHECK_GT!(self.num_registers_, reg);
            if by != 0 {
               // __ACCESS_MASM!(masm_).LoadU64(r3, self.register_location(reg), r0);
               // __ACCESS_MASM!(masm_).mov(r0, Operand(by));
               // __ACCESS_MASM!(masm_).add(r3, r3, r0);
               // __ACCESS_MASM!(masm_).StoreU64(r3, self.register_location(reg), r0);
            }
        }

        fn Backtrack(&mut self, masm_: &mut Box<MacroAssembler>) {
            self.CheckPreemption(masm_);
            if self.has_backtrack_limit() {
                //Label next;
               // __ACCESS_MASM!(masm_).LoadU64(r3, MemOperand(self.frame_pointer(), kBacktrackCountOffset), r0);
               // __ACCESS_MASM!(masm_).addi(r3, r3, Operand(1));
               // __ACCESS_MASM!(masm_).StoreU64(r3, MemOperand(self.frame_pointer(), kBacktrackCountOffset), r0);
               // __ACCESS_MASM!(masm_).mov(r0, Operand(self.backtrack_limit()));
               // __ACCESS_MASM!(masm_).CmpS64(r3, r0);
               // __ACCESS_MASM!(masm_).bne(&next);

                // Backtrack limit exceeded.
                if self.can_fallback() {
                    //__ACCESS_MASM!(masm_).b(&self.fallback_label_);
                } else {
                    // Can't fallback, so we treat it as a failed match.
                    self.Fail(masm_);
                }

                //__ACCESS_MASM!(masm_).bind(&next);
            }
            // Pop InstructionStream offset from backtrack stack, add InstructionStream
            // and jump to location.
            //self.Pop(r3, masm_);
           // __ACCESS_MASM!(masm_).add(r3, r3, self.code_pointer());
           // __ACCESS_MASM!(masm_).Jump(r3);
        }

        fn CheckCharacter(&mut self, c: u32, on_equal: &Label, masm_: &mut Box<MacroAssembler>) {
           // __ACCESS_MASM!(masm_).CmpU64(self.current_character(), Operand(c), r0);
           // self.BranchOrBacktrack(eq, on_equal, masm_, 0);
        }

        fn CheckCharacterGT(&mut self, limit: u16, on_greater: &Label, masm_: &mut Box<MacroAssembler>) {
           // __ACCESS_MASM!(masm_).CmpU64(self.current_character(), Operand(limit as i64), r0);
           // self.BranchOrBacktrack(gt, on_greater, masm_, 0);
        }

        fn CheckAtStart(&mut self, cp_offset: i32, on_at_start: &Label, masm_: &mut Box<MacroAssembler>) {
           // __ACCESS_MASM!(masm_).LoadU64(r4, MemOperand(self.frame_pointer(), kStringStartMinusOneOffset), r0);
           // __ACCESS_MASM!(masm_).addi(r3, self.current_input_offset(),
           //         Operand(-self.char_size() + cp_offset * self.char_size()));
           // __ACCESS_MASM!(masm_).CmpS64(r3, r4);
           // self.BranchOrBacktrack(eq, on_at_start, masm_, 0);
        }

        fn CheckNotAtStart(&mut self, cp_offset: i32, on_not_at_start: &Label, masm_: &mut Box<MacroAssembler>) {
            //__ACCESS_MASM!(masm_).LoadU64(r4, MemOperand(self.frame_pointer(), kStringStartMinusOneOffset), r0);
            //__ACCESS_MASM!(masm_).addi(r3, self.current_input_offset(),
            //        Operand(-self.char_size() + cp_offset * self.char_size()));
            //__ACCESS_MASM!(masm_).CmpS64(r3, r4);
            //self.BranchOrBacktrack(ne, on_not_at_start, masm_, 0);
        }

        fn CheckCharacterLT(&mut self, limit: u16, on_less: &Label, masm_: &mut Box<MacroAssembler>) {
            //__ACCESS_MASM!(masm_).CmpU64(self.current_character(), Operand(limit as i64), r0);
            //self.BranchOrBacktrack(lt, on_less, masm_, 0);
        }

        fn CheckGreedyLoop(&mut self, on_equal: &Label, masm_: &mut Box<MacroAssembler>) {
           // Label backtrack_non_equal;
           // __ACCESS_MASM!(masm_).LoadU64(r3, MemOperand(self.backtrack_stackpointer(), 0), r0);
           // __ACCESS_MASM!(masm_).CmpS64(self.current_input_offset(), r3);
           // __ACCESS_MASM!(masm_).bne(&backtrack_non_equal);
           // __ACCESS_MASM!(masm_).addi(self.backtrack_stackpointer(), self.backtrack_stackpointer(),
           //         Operand(kSystemPointerSize as i64));

           // __ACCESS_MASM!(masm_).bind(&backtrack_non_equal);
           // self.BranchOrBacktrack(eq, on_equal, masm_, 0);
        }

        fn CheckNotBackReferenceIgnoreCase(&mut self, start_reg: i32, read_backward: bool, unicode: bool, on_no_match: &Label, masm_: &mut Box<MacroAssembler>) {
           // Label fallthrough;
           // __ACCESS_MASM!(masm_).LoadU64(r3, self.register_location(start_reg),
           //         r0);  // Index of start of capture
           // __ACCESS_MASM!(masm_).LoadU64(r4, self.register_location(start_reg + 1), r0);  // Index of end
           // __ACCESS_MASM!(masm_).sub(r4, r4, r3, LeaveOE, SetRC);                  // Length of capture.

            // At this point, the capture registers are either both set or both cleared.
            // If the capture length is zero, then the capture is either empty or cleared.
            // Fall through in both cases.
            //__ACCESS_MASM!(masm_).beq(&fallthrough, cr0);

            // Check that there are enough characters left in the input.
            if read_backward {
               // __ACCESS_MASM!(masm_).LoadU64(r6, MemOperand(self.frame_pointer(), kStringStartMinusOneOffset), r0);
               // __ACCESS_MASM!(masm_).add(r6, r6, r4);
               // __ACCESS_MASM!(masm_).CmpS64(self.current_input_offset(), r6);
               // self.BranchOrBacktrack(le, on_no_match, masm_, 0);
            } else {
               // __ACCESS_MASM!(masm_).add(r0, r4, self.current_input_offset(), LeaveOE, SetRC);
               // self.BranchOrBacktrack(gt, on_no_match, masm_, cr0);
            }

            if self.mode_ == Mode::LATIN1 {
                //Label success;
                //Label fail;
                //Label loop_check;

                // r3 - offset of start of capture
                // r4 - length of capture
               // __ACCESS_MASM!(masm_).add(r3, r3, self.end_of_input_address());
               // __ACCESS_MASM!(masm_).add(r5, self.end_of_input_address(), self.current_input_offset());
                if read_backward {
                   // __ACCESS_MASM!(masm_).sub(r5, r5, r4);  // Offset by length when matching backwards.
                }
               // __ACCESS_MASM!(masm_).add(r4, r3, r4);

                // r3 - Address of start of capture.
                // r4 - Address of end of capture
                // r5 - Address of current input position.

                //Label loop;
               // __ACCESS_MASM!(masm_).bind(&loop);
               // __ACCESS_MASM!(masm_).lbz(r6, MemOperand(r3));
               // __ACCESS_MASM!(masm_).addi(r3, r3, Operand(self.char_size() as i64));
               // __ACCESS_MASM!(masm_).lbz(r25, MemOperand(r5));
               // __ACCESS_MASM!(masm_).addi(r5, r5, Operand(self.char_size() as i64));
               // __ACCESS_MASM!(masm_).CmpS64(r25, r6);
               // __ACCESS_MASM!(masm_).beq(&loop_check);

                // Mismatch, try case-insensitive match (converting letters to lower-case).
               // __ACCESS_MASM!(masm_).ori(r6, r6, Operand(0x20));  // Convert capture character to lower-case.
               // __ACCESS_MASM!(masm_).ori(r25, r25, Operand(0x20));  // Also convert input character.
               // __ACCESS_MASM!(masm_).CmpS64(r25, r6);
               // __ACCESS_MASM!(masm_).bne(&fail);
               // __ACCESS_MASM!(masm_).subi(r6, r6, Operand('a' as i64));
               // __ACCESS_MASM!(masm_).cmpli(r6, Operand(('z' as i32 - 'a' as i32) as i64));  // Is r6 a lowercase letter?
               // __ACCESS_MASM!(masm_).ble(&loop_check);               // In range 'a'-'z'.
                // Latin-1: Check for values in range [224,254] but not 247.
               // __ACCESS_MASM!(masm_).subi(r6, r6, Operand((224 - 'a' as i32) as i64));
               // __ACCESS_MASM!(masm_).cmpli(r6, Operand((254 - 224) as i64));
               // __ACCESS_MASM!(masm_).bgt(&fail);                    // Weren't Latin-1 letters.
               // __ACCESS_MASM!(masm_).cmpi(r6, Operand((247 - 224) as i64));  // Check for 247.
               // __ACCESS_MASM!(masm_).beq(&fail);

               // __ACCESS_MASM!(masm_).bind(&loop_check);
               // __ACCESS_MASM!(masm_).CmpS64(r3, r4);
               // __ACCESS_MASM!(masm_).blt(&loop);
               // __ACCESS_MASM!(masm_).b(&success);

               // __ACCESS_MASM!(masm_).bind(&fail);
               // self.BranchOrBacktrack(al, on_no_match, masm_, 0);

               // __ACCESS_MASM!(masm_).bind(&success);
                // Compute new value of character position after the matched part.
               // __ACCESS_MASM!(masm_).sub(self.current_input_offset(), r5, self.end_of_input_address());
                if read_backward {
                   // __ACCESS_MASM!(masm_).LoadU64(r3,
                   //         self.register_location(start_reg), r0);  // Index of start of capture
                   // __ACCESS_MASM!(masm_).LoadU64(r4,
                   //         self.register_location(start_reg + 1), r0);  // Index of end of capture
                   // __ACCESS_MASM!(masm_).add(self.current_input_offset(), self.current_input_offset(), r3);
                   // __ACCESS_MASM!(masm_).sub(self.current_input_offset(), self.current_input_offset(), r4);
                }
            } else {
                DCHECK_EQ!(self.mode_, Mode::UC16);
                let argument_count = 4;
                //__ACCESS_MASM!(masm_).PrepareCallCFunction(argument_count, r5);

                // r3 - offset of start of capture
                // r4 - length of capture

                // Put arguments into arguments registers.
                // Parameters are
                //   r3: Address byte_offset1 - Address captured substring's start.
                //   r4: Address byte_offset2 - Address of current character position.
                //   r5: size_t byte_length - length of capture in bytes(!)
                //   r6: Isolate* isolate.

                // Address of start of capture.
               // __ACCESS_MASM!(masm_).add(r3, r3, self.end_of_input_address());
                // Length of capture.
               // __ACCESS_MASM!(masm_).mr(r5, r4);
                // Save length in callee-save register for use on return.
               // __ACCESS_MASM!(masm_).mr(r25, r4);
                // Address of current input position.
               // __ACCESS_MASM!(masm_).add(r4, self.current_input_offset(), self.end_of_input_address());
                if read_backward {
                   // __ACCESS_MASM!(masm_).sub(r4, r4, r25);
                }
                // Isolate.
               // __ACCESS_MASM!(masm_).mov(r6, Operand(ExternalReference::isolate_address(self.isolate_)));

                //{
                //    AllowExternalCallThatCantCauseGC scope(masm_.get());
                //    ExternalReference function =
                //        unicode
                //            ? ExternalReference::re_case_insensitive_compare_unicode()
                //            : ExternalReference::re_case_insensitive_compare_non_unicode();
                //    CallCFunctionFromIrregexpCode(function, argument_count);
                //}

                // Check if function returned non-zero for success or zero for failure.
               // __ACCESS_MASM!(masm_).cmpi(r3, Operand::Zero());
               // self.BranchOrBacktrack(eq, on_no_match, masm_, 0);

                // On success, advance position by length of capture.
                if read_backward {
                   // __ACCESS_MASM!(masm_).sub(self.current_input_offset(), self.current_input_offset(), r25);
                } else {
                   // __ACCESS_MASM!(masm_).add(self.current_input_offset(), self.current_input_offset(), r25);
                }
            }

           // __ACCESS_MASM!(masm_).bind(&fallthrough);
        }

        fn CheckNotBackReference(&mut self, start_reg: i32, read_backward: bool, on_no_match: &Label, masm_: &mut Box<MacroAssembler>) {
            //Label fallthrough;

            // Find length of back-referenced capture.
            //__ACCESS_MASM!(masm_).LoadU64(r3, self.register_location(start_reg), r0);
            //__ACCESS_MASM!(masm_).LoadU64(r4, self.register_location(start_reg + 1), r0);
            //__ACCESS_MASM!(masm_).sub(r4, r4, r3, LeaveOE, SetRC);  // Length to check.

            // At this point, the capture registers are either both set or both cleared.
            // If the capture length is zero, then the capture is either empty or cleared.
            // Fall through in both cases.
            //__ACCESS_MASM!(masm_).beq(&fallthrough, cr0);

            // Check that there are enough characters left in the input.
            if read_backward {
                //__ACCESS_MASM!(masm_).LoadU64(r6, MemOperand(self.frame_pointer(), kStringStartMinusOneOffset), r0);
                //__ACCESS_MASM!(masm_).add(r6, r6, r4);
                //__ACCESS_MASM!(masm_).CmpS64(self.current_input_offset(), r6);
                //self.BranchOrBacktrack(le, on_no_match, masm_, 0);
            } else {
                //__ACCESS_MASM!(masm_).add(r0, r4, self.current_input_offset(), LeaveOE, SetRC);
                //self.BranchOrBacktrack(gt, on_no_match, masm_, cr0);
            }

            // r3 - offset of start of capture
            // r4 - length of capture
            //__ACCESS_MASM!(masm_).add(r3, r3, self.end_of_input_address());
            //__ACCESS_MASM!(masm_).add(r5, self.end_of_input_address(), self.current_input_offset());
            if read_backward {
                //__ACCESS_MASM!(masm_).sub(r5, r5, r4);  // Offset by length when matching backwards.
            }
            //__ACCESS_MASM!(masm_).add(r4, r4, r3);

            //Label loop;
            //__ACCESS_MASM!(masm_).bind(&loop);
            if self.mode_ == Mode::LATIN1 {
                //__ACCESS_MASM!(masm_).lbz(r6, MemOperand(r3));
                //__ACCESS_MASM!(masm_).addi(r3, r3, Operand(self.char_size() as i64));
                //__ACCESS_MASM!(masm_).lbz(r25, MemOperand(r5));
                //__ACCESS_MASM!(masm_).addi(r5, r5, Operand(self.char_size() as i64));
            } else {
                DCHECK_EQ!(self.mode_, Mode::UC16);
                //__ACCESS_MASM!(masm_).lhz(r6, MemOperand(r3));
                //__ACCESS_MASM!(masm_).addi(r3, r3, Operand(self.char_size() as i64));
                //__ACCESS_MASM!(masm_).lhz(r25, MemOperand(r5));
                //__ACCESS_MASM!(masm_).addi(r5, r5, Operand(self.char_size() as i64));
            }
            //__ACCESS_MASM!(masm_).CmpS64(r6, r25);
            //self.BranchOrBacktrack(ne, on_no_match, masm_, 0);
            //__ACCESS_MASM!(masm_).CmpS64(r3, r4);
            //__ACCESS_MASM!(masm_).blt(&loop);

            // Move current character position to position after match.
            //__ACCESS_MASM!(masm_).sub(self.current_input_offset(), r5, self.end_of_input_address());
            if read_backward {
                //__ACCESS_MASM!(masm_).LoadU64(r3, self.register_location(start_reg), r0);  // Index of start of capture
                //__ACCESS_MASM!(masm_).LoadU64(r4,
                //        self.register_location(start_reg + 1), r0);  // Index of end of capture
                //__ACCESS_MASM!(masm_).add(self.current_input_offset