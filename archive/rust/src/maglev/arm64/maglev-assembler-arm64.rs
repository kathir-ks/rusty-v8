// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Add appropriate Rust crates for any C++ libraries used
// TODO: For header files (.h, .hpp), create appropriate Rust module definitions and public interfaces

pub mod maglev {
    pub mod arm64 {
        pub mod maglev_assembler_arm64 {
            use std::ops::{Add, Sub};

            // Placeholder constants - replace with actual values from V8
            const kHeapObjectTag: i64 = 1;
            const kTaggedSize: i64 = 8;
            const kSystemPointerSize: i64 = 8;
            const kStringRepresentationMask: i64 = 3;
            const kSeqStringTag: i64 = 0;
            const kConsStringTag: i64 = 1;
            const kSlicedStringTag: i64 = 2;
            const kThinStringTag: i64 = 3;
            const kOneByteStringTag: i64 = 4;
            const OFFSET_OF_DATA_START_FIXED_ARRAY: i64 = 16;
            const OFFSET_OF_DATA_START_SEQ_ONE_BYTE_STRING: i64 = 12;
            const OFFSET_OF_DATA_START_SEQ_TWO_BYTE_STRING: i64 = 12;
            const FIRST_STRING_TYPE: i64 = 1;
            const LAST_STRING_TYPE: i64 = 2;

            // Placeholder enums - replace with actual enums from V8
            #[derive(Debug, PartialEq, Eq, Copy, Clone)]
            pub enum AllocationType {
                kOld,
            }

            #[derive(Debug, PartialEq, Eq, Copy, Clone)]
            pub enum AllocationAlignment {
                kTaggedAligned,
            }

            #[derive(Debug, PartialEq, Eq, Copy, Clone)]
            pub enum RootIndex {
                kSingleCharacterStringTable,
                kempty_string,
            }

            #[derive(Debug, PartialEq, Eq, Copy, Clone)]
            pub enum AbortReason {
                kOsrUnexpectedStackSize,
                kUnexpectedValue,
            }

            #[derive(Debug, PartialEq, Eq, Copy, Clone)]
            pub enum CharCodeMaskMode {
                kMustApplyMask,
            }

            #[derive(Debug, PartialEq, Eq, Copy, Clone)]
            pub enum CodeKind {
                MAGLEV,
            }

            #[derive(Debug, PartialEq, Eq, Copy, Clone)]
            pub enum StackFrame {
                MAGLEV,
            }

            #[derive(Debug, PartialEq, Eq, Copy, Clone)]
            pub enum Builtin {
                kDeoptimizationEntry_Eager,
                kDeoptimizationEntry_Lazy,
                kMaglevOptimizeCodeOrTailCallOptimizedCodeSlot,
                kDoubleToI,
            }

            // Placeholder structs/enums - replace with actual structs/enums from V8
            pub struct MaglevAssembler<'a> {
                isolate_: &'a Isolate,
                code_gen_state_: CodeGenState,
                eager_deopt_count_: usize,
                eager_deopt_entry_: Option<Label>,
                lazy_deopt_count_: usize,
                lazy_deopt_entry_: Option<Label>,
            }

            struct CodeGenState {
                stack_slots_: i32,
                entry_label_: Label,
            }

            impl CodeGenState {
                fn entry_label(&self) -> &Label {
                    &self.entry_label_
                }
            }

            pub struct Graph {
                is_osr_: bool,
                has_recursive_calls_: bool,
                tagged_stack_slots_: u32,
                untagged_stack_slots_: u32,
                min_maglev_stackslots_for_unoptimized_frame_size_: u32,
            }

            impl Graph {
                fn is_osr(&self) -> bool {
                    self.is_osr_
                }

                fn has_recursive_calls(&self) -> bool {
                    self.has_recursive_calls_
                }

                fn tagged_stack_slots(&self) -> u32 {
                    self.tagged_stack_slots_
                }

                fn untagged_stack_slots(&self) -> u32 {
                    self.untagged_stack_slots_
                }

                fn min_maglev_stackslots_for_unoptimized_frame_size(&self) -> u32 {
                    self.min_maglev_stackslots_for_unoptimized_frame_size_
                }
            }

            pub struct Isolate {}
            pub struct RegisterSnapshot {}
            pub struct Label {}
            pub struct Register {}
            pub struct DoubleRegister {}
            pub struct Operand {}
            pub struct Immediate(i64);
            pub struct MemOperand {}

            impl MaglevAssembler<'_> {
                pub fn new(isolate: &Isolate, stack_slots: i32) -> MaglevAssembler {
                    MaglevAssembler {
                        isolate_: isolate,
                        code_gen_state_: CodeGenState {
                            stack_slots_: stack_slots,
                            entry_label_: Label {},
                        },
                        eager_deopt_count_: 0,
                        eager_deopt_entry_: None,
                        lazy_deopt_count_: 0,
                        lazy_deopt_entry_: None,
                    }
                }

                fn allow_allocate(&self) -> bool {
                    true
                }

                fn isolate_(&self) -> &Isolate {
                    self.isolate_
                }

                fn code_gen_state(&self) -> &CodeGenState {
                    &self.code_gen_state_
                }

                pub fn allocate(
                    &mut self,
                    register_snapshot: RegisterSnapshot,
                    object: Register,
                    size_in_bytes: i64,
                    alloc_type: AllocationType,
                    alignment: AllocationAlignment,
                ) {
                    allocate_raw(
                        self,
                        self.isolate_,
                        register_snapshot,
                        object,
                        size_in_bytes,
                        alloc_type,
                        alignment,
                    );
                }

                pub fn allocate_register(
                    &mut self,
                    register_snapshot: RegisterSnapshot,
                    object: Register,
                    size_in_bytes: Register,
                    alloc_type: AllocationType,
                    alignment: AllocationAlignment,
                ) {
                    allocate_raw_register(
                        self,
                        self.isolate_,
                        register_snapshot,
                        object,
                        size_in_bytes,
                        alloc_type,
                        alignment,
                    );
                }

                pub fn osr_prologue(&mut self, graph: &Graph) {
                    assert!(graph.is_osr());
                    assert!(!graph.has_recursive_calls());

                    let source_frame_size =
                        graph.min_maglev_stackslots_for_unoptimized_frame_size();

                    //static_assert(StandardFrameConstants::kFixedSlotCount % 2 == 1);
                    //TODO!("Handle StandardFrameConstants::kFixedSlotCount");
                    if source_frame_size % 2 == 0 {
                        //source_frame_size++;
                        //TODO!("fix this code")
                    }

                    //TODO!("Handle V8_ENABLE_SANDBOX_BOOL || v8_flags.debug_code");

                    let target_frame_size =
                        graph.tagged_stack_slots() + graph.untagged_stack_slots();
                    //CHECK_EQ(target_frame_size % 2, 1);
                    //CHECK_LE(source_frame_size, target_frame_size);
                    //TODO!("fix these asserts!");
                    // if source_frame_size < target_frame_size {
                    //     ASM_CODE_COMMENT_STRING(this, "Growing frame for OSR");
                    //     let additional_tagged =
                    //         if source_frame_size < graph.tagged_stack_slots() {
                    //             graph.tagged_stack_slots() - source_frame_size
                    //         } else {
                    //             0
                    //         };
                    //     let additional_tagged_double =
                    //         additional_tagged / 2 + additional_tagged % 2;
                    //     for i in 0..additional_tagged_double {
                    //         //Push(xzr, xzr);
                    //         //TODO!("fix push")
                    //     }
                    //     let size_so_far = source_frame_size + additional_tagged_double * 2;
                    //     //CHECK_LE(size_so_far, target_frame_size);
                    //     //TODO!("fix this asserts!");
                    //     // if size_so_far < target_frame_size {
                    //     //     Sub(sp, sp,
                    //     //         Immediate((target_frame_size - size_so_far) * kSystemPointerSize));
                    //     //     //TODO!("fix sub")
                    //     // }
                    // }
                    todo!()
                }

                pub fn prologue(&mut self, graph: &Graph) {
                    // TemporaryRegisterScope temps(this);
                    //  We add two extra registers to the scope. Ideally we could add all the
                    //  allocatable general registers, except Context, JSFunction, NewTarget and
                    //  ArgCount. Unfortunately, OptimizeCodeOrTailCallOptimizedCodeSlot and
                    //  LoadFeedbackVectorFlagsAndJumpIfNeedsProcessing pick random registers and
                    //  we could alias those.
                    // TODO(victorgomes): Fix these builtins to either use the scope or pass the
                    // used registers manually.
                    //temps.Include({x14, x15});
                    todo!()
                    //DCHECK(!graph->is_osr());
                    // assert!(!graph.is_osr());
                    //TODO!("fix this assert");

                    // CallTarget();
                    // BailoutIfDeoptimized();
                    //TODO!("fix these calls")

                    // if (graph->has_recursive_calls()) {
                    //     BindCallTarget(code_gen_state()->entry_label());
                    // }
                    //TODO!("fix this if")

                    // #ifndef V8_ENABLE_LEAPTIERING
                    // Tiering support.
                    //TODO!("Handle V8_ENABLE_LEAPTIERING")

                    // if (v8_flags.turbofan) {
                    //     using D = MaglevOptimizeCodeOrTailCallOptimizedCodeSlotDescriptor;
                    //     Register flags = D::GetRegisterParameter(D::kFlags);
                    //     Register feedback_vector = D::GetRegisterParameter(D::kFeedbackVector);
                    //     DCHECK(!AreAliased(flags, feedback_vector, kJavaScriptCallArgCountRegister,
                    //                        kJSFunctionRegister, kContextRegister,
                    //                        kJavaScriptCallNewTargetRegister,
                    //                        kJavaScriptCallDispatchHandleRegister));
                    //     DCHECK(!temps.Available().has(flags));
                    //     DCHECK(!temps.Available().has(feedback_vector));
                    //     Move(feedback_vector,
                    //          compilation_info()->toplevel_compilation_unit()->feedback().object());
                    //     Condition needs_processing =
                    //         LoadFeedbackVectorFlagsAndCheckIfNeedsProcessing(flags, feedback_vector,
                    //                                                          CodeKind::MAGLEV);
                    //     TailCallBuiltin(Builtin::kMaglevOptimizeCodeOrTailCallOptimizedCodeSlot,
                    //                     needs_processing);
                    // }
                    // #endif  // !V8_ENABLE_LEAPTIERING
                    // EnterFrame(StackFrame::MAGLEV);
                    //TODO!("fix these calls")

                    // Save arguments in frame.
                    // TODO(leszeks): Consider eliding this frame if we don't make any calls
                    // that could clobber these registers.
                    // Push the context and the JSFunction.
                    //Push(kContextRegister, kJSFunctionRegister);
                    // Push the actual argument count and a _possible_ stack slot.
                    //Push(kJavaScriptCallArgCountRegister, xzr);
                    //TODO!("fix these pushs")
                    // let remaining_stack_slots = code_gen_state().stack_slots() - 1;
                    // assert!(remaining_stack_slots >= 0);
                    //TODO!("fix this assert!")
                    // Initialize stack slots.
                    // if (graph->tagged_stack_slots() > 0) {
                    //     ASM_CODE_COMMENT_STRING(this, "Initializing stack slots");

                    //     // If tagged_stack_slots is divisible by 2, we overshoot and allocate one
                    //     // extra stack slot, otherwise we allocate exactly the right amount, since
                    //     // one stack has already been allocated.
                    //     let tagged_two_slots_count = graph->tagged_stack_slots() / 2;
                    //     remaining_stack_slots -= 2 * tagged_two_slots_count;

                    //     // Magic value. Experimentally, an unroll size of 8 doesn't seem any
                    //     // worse than fully unrolled pushes.
                    //     const int kLoopUnrollSize = 8;
                    //     if (tagged_two_slots_count < kLoopUnrollSize) {
                    //         for (int i = 0; i < tagged_two_slots_count; i++) {
                    //             Push(xzr, xzr);
                    //         }
                    //     } else {
                    //         TemporaryRegisterScope temps(this);
                    //         Register count = temps.AcquireScratch();
                    //         // Extract the first few slots to round to the unroll size.
                    //         let first_slots = tagged_two_slots_count % kLoopUnrollSize;
                    //         for (int i = 0; i < first_slots; ++i) {
                    //             Push(xzr, xzr);
                    //         }
                    //         Move(count, tagged_two_slots_count / kLoopUnrollSize);
                    //         // We enter the loop unconditionally, so make sure we need to loop at
                    //         // least once.
                    //         DCHECK_GT(tagged_two_slots_count / kLoopUnrollSize, 0);
                    //         Label loop;
                    //         bind(&loop);
                    //         for (int i = 0; i < kLoopUnrollSize; ++i) {
                    //             Push(xzr, xzr);
                    //         }
                    //         Subs(count, count, Immediate(1));
                    //         B(&loop, gt);
                    //     }
                    // }
                    // if (remaining_stack_slots > 0) {
                    //     // Round up.
                    //     remaining_stack_slots += (remaining_stack_slots % 2);
                    //     // Extend sp by the size of the remaining untagged part of the frame,
                    //     // no need to initialise these.
                    //     Sub(sp, sp, Immediate(remaining_stack_slots * kSystemPointerSize));
                    // }
                }

                pub fn maybe_emit_deopt_builtins_call(
                    &mut self,
                    eager_deopt_count: usize,
                    eager_deopt_entry: &mut Option<Label>,
                    lazy_deopt_count: usize,
                    lazy_deopt_entry: &mut Option<Label>,
                ) {
                    // ForceConstantPoolEmissionWithoutJump();
                    //TODO!("fix this call")

                    // //DCHECK_GE(Deoptimizer::kLazyDeoptExitSize, Deoptimizer::kEagerDeoptExitSize);
                    // let deopt_count = eager_deopt_count + lazy_deopt_count;
                    // CheckVeneerPool(
                    //     false, false,
                    //     static_cast<int>(deopt_count) * Deoptimizer::kLazyDeoptExitSize);
                    //TODO!("fix these asserts and call!")

                    // TemporaryRegisterScope scope(this);
                    // Register scratch = scope.AcquireScratch();
                    // if (eager_deopt_count > 0) {
                    //     Bind(eager_deopt_entry);
                    //     LoadEntryFromBuiltin(Builtin::kDeoptimizationEntry_Eager, scratch);
                    //     MacroAssembler::Jump(scratch);
                    // }
                    // if (lazy_deopt_count > 0) {
                    //     Bind(lazy_deopt_entry);
                    //     LoadEntryFromBuiltin(Builtin::kDeoptimizationEntry_Lazy, scratch);
                    //     MacroAssembler::Jump(scratch);
                    // }
                }

                pub fn load_single_character_string(
                    &mut self,
                    result: Register,
                    char_code: Register,
                    scratch: Register,
                ) {
                    //DCHECK_NE(char_code, scratch);
                    //TODO!("fix this assert!")
                    // if (v8_flags.debug_code) {
                    //     Cmp(char_code, Immediate(String::kMaxOneByteCharCode));
                    //     Assert(ls, AbortReason::kUnexpectedValue);
                    // }
                    //TODO!("fix this if")
                    // Register table = scratch;
                    // LoadRoot(table, RootIndex::kSingleCharacterStringTable);
                    // LoadTaggedFieldByIndex(result, table, char_code, kTaggedSize,
                    //                         OFFSET_OF_DATA_START(FixedArray));
                }

                pub fn string_from_char_code(
                    &mut self,
                    register_snapshot: RegisterSnapshot,
                    char_code_fits_one_byte: &mut Option<Label>,
                    result: Register,
                    char_code: Register,
                    scratch: Register,
                    mask_mode: CharCodeMaskMode,
                ) {
                    // AssertZeroExtended(char_code);
                    // //DCHECK_NE(char_code, scratch);
                    //TODO!("fix this assert!")
                    // ZoneLabelRef done(this);
                    // if (mask_mode == CharCodeMaskMode::kMustApplyMask) {
                    //     And(char_code, char_code, Immediate(0xFFFF));
                    // }
                    //TODO!("fix this if")
                    // Cmp(char_code, Immediate(String::kMaxOneByteCharCode));
                    // JumpToDeferredIf(
                    //     hi,
                    //     [](MaglevAssembler* masm, RegisterSnapshot register_snapshot,
                    //        ZoneLabelRef done, Register result, Register char_code,
                    //        Register scratch) {
                    //         // Be sure to save {char_code}. If it aliases with {result}, use
                    //         // the scratch register.
                    //         // TODO(victorgomes): This is probably not needed any more, because
                    //         // we now ensure that results registers don't alias with inputs/temps.
                    //         // Confirm, and drop this check.
                    //         if (char_code.Aliases(result)) {
                    //             __ Move(scratch, char_code);
                    //             char_code = scratch;
                    //         }
                    //         DCHECK(!char_code.Aliases(result));
                    //         DCHECK(!register_snapshot.live_tagged_registers.has(char_code));
                    //         register_snapshot.live_registers.set(char_code);
                    //         __ AllocateTwoByteString(register_snapshot, result, 1);
                    //         __ Strh(
                    //             char_code.W(),
                    //             FieldMemOperand(result, OFFSET_OF_DATA_START(SeqTwoByteString)));
                    //         __ B(*done);
                    //     },
                    //     register_snapshot, done, result, char_code, scratch);
                    //TODO!("fix this call!")
                    // if (char_code_fits_one_byte != nullptr) {
                    //     bind(char_code_fits_one_byte);
                    // }
                    // LoadSingleCharacterString(result, char_code, scratch);
                    // bind(*done);
                }

                pub fn string_char_code_or_code_point_at(
                    &mut self,
                    mode: BuiltinStringPrototypeCharCodeOrCodePointAtMode,
                    register_snapshot: &mut RegisterSnapshot,
                    result: Register,
                    string: Register,
                    index: Register,
                    scratch1: Register,
                    scratch2: Register,
                    result_fits_one_byte: &mut Option<Label>,
                ) {
                    // ZoneLabelRef done(this);
                    // Label seq_string;
                    // Label cons_string;
                    // Label sliced_string;
                    //TODO!("fix these labels")

                    // Label* deferred_runtime_call = MakeDeferredCode(
                    //     [](MaglevAssembler* masm,
                    //        BuiltinStringPrototypeCharCodeOrCodePointAt::Mode mode,
                    //        RegisterSnapshot register_snapshot, ZoneLabelRef done, Register result,
                    //        Register string, Register index) {
                    //         DCHECK(!register_snapshot.live_registers.has(result));
                    //         DCHECK(!register_snapshot.live_registers.has(string));
                    //         DCHECK(!register_snapshot.live_registers.has(index));
                    //         {
                    //             SaveRegisterStateForCall save_register_state(masm, register_snapshot);
                    //             __ SmiTag(index);
                    //             __ Push(string, index);
                    //             __ Move(kContextRegister, masm->native_context().object());
                    //             // This call does not throw nor can deopt.
                    //             if (mode ==
                    //                 BuiltinStringPrototypeCharCodeOrCodePointAt::kCodePointAt) {
                    //                 __ CallRuntime(Runtime::kStringCodePointAt);
                    //             } else {
                    //                 DCHECK_EQ(mode,
                    //                           BuiltinStringPrototypeCharCodeOrCodePointAt::kCharCodeAt);
                    //                 __ CallRuntime(Runtime::kStringCharCodeAt);
                    //             }
                    //             save_register_state.DefineSafepoint();
                    //             __ SmiUntag(kReturnRegister0);
                    //             __ Move(result, kReturnRegister0);
                    //         }
                    //         __ jmp(*done);
                    //     },
                    //     mode, register_snapshot, done, result, string, index);
                    //TODO!("fix these calls")

                    // // We might need to try more than one time for ConsString, SlicedString and
                    // // ThinString.
                    // Label loop;
                    // bind(&loop);
                    //TODO!("fix this label")

                    // if (v8_flags.debug_code) {
                    //     // Check if {string} is a string.
                    //     AssertObjectTypeInRange(string, FIRST_STRING_TYPE, LAST_STRING_TYPE,
                    //                             AbortReason::kUnexpectedValue);

                    //     Ldr(scratch1.W(), FieldMemOperand(string, offsetof(String, length_)));
                    //     Cmp(index.W(), scratch1.W());
                    //     Check(lo, AbortReason::kUnexpectedValue);
                    // }
                    //TODO!("fix these calls")

                    // #if V8_STATIC_ROOTS_BOOL
                    //     Register map = scratch1.W();
                    //     LoadMapForCompare(map, string);
                    // #else
                    //     Register instance_type = scratch1;
                    //     // Get instance type.
                    //     LoadInstanceType(instance_type, string);
                    // #endif
                    //TODO!("fix these calls")

                    // {
                    // #if V8_STATIC_ROOTS_BOOL
                    //         using StringTypeRange = InstanceTypeChecker::kUniqueMapRangeOfStringType;
                    //         // Check the string map ranges in dense increasing order, to avoid needing
                    //         // to subtract away the lower bound.
                    //         static_assert(StringTypeRange::kSeqString.first == 0);
                    //         CompareInt32AndJumpIf(map, StringTypeRange::kSeqString.second,
                    //                               kUnsignedLessThanEqual, &seq_string, Label::kNear);

                    //         static_assert(StringTypeRange::kSeqString.second + Map::kSize ==
                    //                       StringTypeRange::kExternalString.first);
                    //         CompareInt32AndJumpIf(map, StringTypeRange::kExternalString.second,
                    //                               kUnsignedLessThanEqual, deferred_runtime_call);
                    //         // TODO(victorgomes): Add fast path for external strings.

                    //         static_assert(StringTypeRange::kExternalString.second + Map::kSize ==
                    //                       StringTypeRange::kConsString.first);
                    //         CompareInt32AndJumpIf(map, StringTypeRange::kConsString.second,
                    //                               kUnsignedLessThanEqual, &cons_string, Label::kNear);

                    //         static_assert(StringTypeRange::kConsString.second + Map::kSize ==
                    //                       StringTypeRange::kSlicedString.first);
                    //         CompareInt32AndJumpIf(map, StringTypeRange::kSlicedString.second,
                    //                               kUnsignedLessThanEqual, &sliced_string, Label::kNear);

                    //         static_assert(StringTypeRange::kSlicedString.second + Map::kSize ==
                    //                       StringTypeRange::kThinString.first);
                    //         // No need to check for thin strings, they're the last string map.
                    //         static_assert(StringTypeRange::kThinString.second ==
                    //                       InstanceTypeChecker::kStringMapUpperBound);
                    //         // Fallthrough to thin string.
                    // #else
                    //         TemporaryRegisterScope temps(this);
                    //         Register representation = temps.AcquireScratch().W();

                    //         // TODO(victorgomes): Add fast path for external strings.
                    //         And(representation, instance_type.W(),
                    //             Immediate(kStringRepresentationMask));
                    //         CompareAndBranch(representation, Immediate(kSeqStringTag), kEqual,
                    //                          &seq_string);
                    //         CompareAndBranch(representation, Immediate(kConsStringTag), kEqual,
                    //                          &cons_string);
                    //         CompareAndBranch(representation, Immediate(kSlicedStringTag), kEqual,
                    //                          &sliced_string);
                    //         CompareAndBranch(representation, Immediate(kThinStringTag), kNotEqual,
                    //                          deferred_runtime_call);
                    //         // Fallthrough to thin string.
                    // #endif
                    //TODO!("fix these calls")
                    // }

                    // // Is a thin string.
                    // {
                    //     LoadTaggedField(string, string, offsetof(ThinString, actual_));
                    //     B(&loop);
                    // }
                    //TODO!("fix these calls")

                    // bind(&sliced_string);
                    // {
                    //     TemporaryRegisterScope temps(this);
                    //     Register offset = temps.AcquireScratch();

                    //     LoadAndUntagTaggedSignedField(offset, string,
                    //                                   offsetof(SlicedString, offset_));
                    //     LoadTaggedField(string, string, offsetof(SlicedString, parent_));
                    //     Add(index, index, offset);
                    //     B(&loop);
                    // }
                    //TODO!("fix these calls")

                    // bind(&cons_string);
                    // {
                    //     // Reuse {instance_type} register here, since CompareRoot requires a scratch
                    //     // register as well.
                    //     Register second_string = scratch1;
                    //     LoadTaggedFieldWithoutDecompressing(second_string, string,
                    //                                         offsetof(ConsString, second_));
                    //     CompareRoot(second_string, RootIndex::kempty_string);
                    //     B(deferred_runtime_call, ne);
                    //     LoadTaggedField(string, string, offsetof(ConsString, first_));
                    //     B(&loop);  // Try again with first string.
                    // }
                    //TODO!("fix these calls")

                    // bind(&seq_string);
                    // {
                    //     Label two_byte_string;
                    // #if V8_STATIC_ROOTS_BOOL
                    //     if (InstanceTypeChecker::kTwoByteStringMapBit == 0) {
                    //         TestInt32AndJumpIfAllClear(map,
                    //                                    InstanceTypeChecker::kStringMapEncodingMask,
                    //                                    &two_byte_string, Label::kNear);
                    //     } else {
                    //         TestInt32AndJumpIfAnySet(map, InstanceTypeChecker::kStringMapEncodingMask,
                    //                                  &two_byte_string, Label::kNear);
                    //     }
                    // #else
                    //     TestAndBranchIfAllClear(instance_type, kOneByteStringTag, &two_byte_string);
                    // #endif
                    //TODO!("fix these calls")

                    //     // The result of one-byte string will be the same for both modes
                    //     // (CharCodeAt/CodePointAt), since it cannot be the first half of a
                    //     // surrogate pair.
                    //     Add(index, index, OFFSET_OF_DATA_START(SeqOneByteString) - kHeapObjectTag);
                    //     Ldrb(result, MemOperand(string, index));
                    //     B(result_fits_one_byte);
                    //TODO!("fix these calls")

                    //     bind(&two_byte_string);
                    //     // {instance_type} is unused from this point, so we can use as scratch.
                    //     Register scratch = scratch1;
                    //     Lsl(scratch, index, 1);
                    //     Add(scratch, scratch,
                    //         OFFSET_OF_DATA_START(SeqTwoByteString) - kHeapObjectTag);
                    //TODO!("fix these calls")

                    //     if (mode == BuiltinStringPrototypeCharCodeOrCodePointAt::kCharCodeAt) {
                    //         Ldrh(result, MemOperand(string, scratch));
                    //     } else {
                    //         DCHECK_EQ(mode,
                    //                   BuiltinStringPrototypeCharCodeOrCodePointAt::kCodePointAt);
                    //         Register string_backup = string;
                    //         if (result == string) {
                    //             string_backup = scratch2;
                    //             Mov(string_backup, string);
                    //         }
                    //         Ldrh(result, MemOperand(string, scratch));
                    //TODO!("fix these calls")

                    //         Register first_code_point = scratch;
                    //         And(first_code_point.W(), result.W(), Immediate(0xfc00));
                    //         CompareAndBranch(first_code_point, Immediate(0xd800), kNotEqual, *done);
                    //TODO!("fix these calls")

                    //         Register length = scratch;
                    //         Ldr(length.W(),
                    //             FieldMemOperand(string_backup, offsetof(String, length_)));
                    //         Add(index.W(), index.W(), Immediate(1));
                    //         CompareAndBranch(index, length, kGreaterThanEqual, *done);
                    //TODO!("fix these calls")

                    //         Register second_code_point = scratch;
                    //         Lsl(index, index, 1);
                    //         Add(index, index,
                    //             OFFSET_OF_DATA_START(SeqTwoByteString) - kHeapObjectTag);
                    //         Ldrh(second_code_point, MemOperand(string_backup, index));
                    //TODO!("fix these calls")

                    //         // {index} is not needed at this point.
                    //         Register scratch2 = index;
                    //         And(scratch2.W(), second_code_point.W(), Immediate(0xfc00));
                    //         CompareAndBranch(scratch2, Immediate(0xdc00), kNotEqual, *done);
                    //TODO!("fix these calls")

                    //         int surrogate_offset = 0x10000 - (0xd800 << 10) - 0xdc00;
                    //         Add(second_code_point, second_code_point, Immediate(surrogate_offset));
                    //         Lsl(result, result, 10);
                    //         Add(result, result, second_code_point);
                    //     }
                    //TODO!("fix these calls")

                    //     // Fallthrough.
                    // }

                    // bind(*done);
                    //TODO!("fix this label")

                    // if (v8_flags.debug_code) {
                    //     // We make sure that the user of this macro is not relying in string and
                    //     // index to not be clobbered.
                    //     if (result != string) {
                    //         Mov(string, Immediate(0xdeadbeef));
                    //     }
                    //     if (result != index) {
                    //         Mov(index, Immediate(0xdeadbeef));
                    //     }
                    // }
                    //TODO!("fix these calls")
                }

                pub fn truncate_double_to_int32(&mut self, dst: Register, src: DoubleRegister) {
                    // if (CpuFeatures::IsSupported(JSCVT)) {
                    //     Fjcvtzs(dst.W(), src);
                    //     return;
                    // }
                    //TODO!("Handle cpu features")

                    // ZoneLabelRef done(this);
                    // // Try to convert with an FPU convert instruction. It's trivial to compute
                    // // the modulo operation on an integer register so we convert to a 64-bit
                    // // integer.
                    // //
                    // // Fcvtzs will saturate to INT64_MIN (0x800...00) or INT64_MAX (0x7FF...FF)
                    // // when the double is out of range. NaNs and infinities will be converted to 0
                    // // (as ECMA-262 requires).
                    // Fcvtzs(dst.X(), src);

                    // // The values INT64_MIN (0x800...00) or INT64_MAX (0x7FF...FF) are not
                    // // representable using a double, so if the result is one of those then we know
                    // // that saturation occurred, and we need to manually handle the conversion.
                    // //
                    // // It is easy to detect INT64_MIN and INT64_MAX because adding or subtracting
                    // // 1 will cause signed overflow.
                    // Cmp(dst.X(), 1);
                    // Ccmp(dst.X(), -1, VFlag, vc);

                    // JumpToDeferredIf(
                    //     vs,
                    //     [](MaglevAssembler* masm, DoubleRegister src, Register dst,
                    //        ZoneLabelRef