// TODO: Add equivalent Rust crates for any C++ libraries used

// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// #include <assert.h>  // For assert
// #include <limits.h>  // For LONG_MIN, LONG_MAX.

// #if V8_TARGET_ARCH_PPC64

// #include <optional>

// #include "src/base/bits.h"
// #include "src/base/division-by-constant.h"
// #include "src/builtins/builtins-inl.h"
// #include "src/codegen/callable.h"
// #include "src/codegen/code-factory.h"
// #include "src/codegen/external-reference-table.h"
// #include "src/codegen/interface-descriptors-inl.h"
// #include "src/codegen/macro-assembler.h"
// #include "src/codegen/register-configuration.h"
// #include "src/codegen/register.h"
// #include "src/debug/debug.h"
// #include "src/deoptimizer/deoptimizer.h"
// #include "src/execution/frames-inl.h"
// #include "src/heap/mutable-page-metadata.h"
// #include "src/init/bootstrapper.h"
// #include "src/logging/counters.h"
// #include "src/runtime/runtime.h"
// #include "src/snapshot/snapshot.h"

// Satisfy cpplint check, but don't include platform-specific header. It is
// included recursively via macro-assembler.h.
// #if 0
// #include "src/codegen/ppc/macro-assembler-ppc.h"
// #endif

// #define __ ACCESS_MASM(masm)

// TODO: Define equivalents for macros

mod internal {
    // TODO: Implement internal modules and structs
}

// use internal::*;

// namespace v8 {
// namespace internal {

// namespace {

// Simd and Floating Pointer registers are not shared. For WebAssembly we save
// both registers, If we are not running Wasm, we can get away with only saving
// FP registers.
// #if V8_ENABLE_WEBASSEMBLY
// constexpr int kStackSavedSavedFPSizeInBytes =
//     (kNumCallerSavedDoubles * kSimd128Size) +
//     (kNumCallerSavedDoubles * kDoubleSize);
// #else
// constexpr int kStackSavedSavedFPSizeInBytes =
//     kNumCallerSavedDoubles * kDoubleSize;
// #endif  // V8_ENABLE_WEBASSEMBLY

// }  // namespace

// int MacroAssembler::RequiredStackSizeForCallerSaved(SaveFPRegsMode fp_mode,
//                                                     Register exclusion1,
//                                                     Register exclusion2,
//                                                     Register exclusion3) const {
//   int bytes = 0;

//   RegList exclusions = {exclusion1, exclusion2, exclusion3};
//   RegList list = kJSCallerSaved - exclusions;
//   bytes += list.Count() * kSystemPointerSize;

//   if (fp_mode == SaveFPRegsMode::kSave) {
//     bytes += kStackSavedSavedFPSizeInBytes;
//   }

//   return bytes;
// }

// int MacroAssembler::PushCallerSaved(SaveFPRegsMode fp_mode, Register scratch1,
//                                     Register scratch2, Register exclusion1,
//                                     Register exclusion2, Register exclusion3) {
//   int bytes = 0;

//   RegList exclusions = {exclusion1, exclusion2, exclusion3};
//   RegList list = kJSCallerSaved - exclusions;
//   MultiPush(list);
//   bytes += list.Count() * kSystemPointerSize;

//   if (fp_mode == SaveFPRegsMode::kSave) {
//     MultiPushF64AndV128(kCallerSavedDoubles, kCallerSavedSimd128s, scratch1,
//                         scratch2);
//     bytes += kStackSavedSavedFPSizeInBytes;
//   }

//   return bytes;
// }

// int MacroAssembler::PopCallerSaved(SaveFPRegsMode fp_mode, Register scratch1,
//                                    Register scratch2, Register exclusion1,
//                                    Register exclusion2, Register exclusion3) {
//   int bytes = 0;
//   if (fp_mode == SaveFPRegsMode::kSave) {
//     MultiPopF64AndV128(kCallerSavedDoubles, kCallerSavedSimd128s, scratch1,
//                        scratch2);
//     bytes += kStackSavedSavedFPSizeInBytes;
//   }

//   RegList exclusions = {exclusion1, exclusion2, exclusion3};
//   RegList list = kJSCallerSaved - exclusions;
//   MultiPop(list);
//   bytes += list.Count() * kSystemPointerSize;

//   return bytes;
// }

// void MacroAssembler::GetLabelAddress(Register dest, Label* target) {
//   // This should be just a
//   //    add(dest, pc, branch_offset(target));
//   // but current implementation of Assembler::bind_to()/target_at_put() add
//   // (InstructionStream::kHeaderSize - kHeapObjectTag) to a position of a label
//   // in a "linked" state and thus making it usable only for mov_label_offset().
//   // TODO(ishell): fix branch_offset() and re-implement
//   // RegExpMacroAssemblerARM::PushBacktrack() without mov_label_offset().
//   mov_label_offset(dest, target);
//   // mov_label_offset computes offset of the |target| relative to the "current
//   // InstructionStream object pointer" which is essentially pc_offset() of the
//   // label added with (InstructionStream::kHeaderSize - kHeapObjectTag).
//   // Compute "current InstructionStream object pointer" and add it to the
//   // offset in |lr| register.
//   int current_instr_code_object_relative_offset =
//       pc_offset() + kPcLoadDelta +
//       (InstructionStream::kHeaderSize - kHeapObjectTag);
//   LoadPC(r0);
//   // LoadPC emits 2 instructions, pc_offset() is pointing to it's first
//   // instruction but real pc will be pointing to it's second instruction, make
//   // an adjustment so they both point to the same offset.
//   current_instr_code_object_relative_offset -= kInstrSize;
//   AddS64(dest, r0, dest);
//   SubS64(dest, dest, Operand(current_instr_code_object_relative_offset));
// }

// void MacroAssembler::Jump(Register target) {
//   mtctr(target);
//   bctr();
// }

// void MacroAssembler::LoadFromConstantsTable(Register destination,
//                                             int constant_index) {
//   DCHECK(RootsTable::IsImmortalImmovable(RootIndex::kBuiltinsConstantsTable));

//   DCHECK_NE(destination, r0);
//   LoadRoot(destination, RootIndex::kBuiltinsConstantsTable);
//   LoadTaggedField(destination,
//                   FieldMemOperand(destination, FixedArray::OffsetOfElementAt(
//                                                    constant_index)),
//                   r0);
// }

// void MacroAssembler::LoadRootRelative(Register destination, int32_t offset) {
//   LoadU64(destination, MemOperand(kRootRegister, offset), r0);
// }

// void MacroAssembler::StoreRootRelative(int32_t offset, Register value) {
//   StoreU64(value, MemOperand(kRootRegister, offset));
// }

// void MacroAssembler::LoadRootRegisterOffset(Register destination,
//                                             intptr_t offset) {
//   if (offset == 0) {
//     mr(destination, kRootRegister);
//   } else {
//     AddS64(destination, kRootRegister, Operand(offset), destination);
//   }
// }

// MemOperand MacroAssembler::ExternalReferenceAsOperand(
//     ExternalReference reference, Register scratch) {
//   if (root_array_available()) {
//     if (reference.IsIsolateFieldId()) {
//       return MemOperand(kRootRegister, reference.offset_from_root_register());
//     }
//     if (options().enable_root_relative_access) {
//       intptr_t offset =
//           RootRegisterOffsetForExternalReference(isolate(), reference);
//       if (is_int32(offset)) {
//         return MemOperand(kRootRegister, static_cast<int32_t>(offset));
//       }
//     }
//     if (options().isolate_independent_code) {
//       if (IsAddressableThroughRootRegister(isolate(), reference)) {
//         // Some external references can be efficiently loaded as an offset from
//         // kRootRegister.
//         intptr_t offset =
//             RootRegisterOffsetForExternalReference(isolate(), reference);
//         CHECK(is_int32(offset));
//         return MemOperand(kRootRegister, static_cast<int32_t>(offset));
//       } else {
//         // Otherwise, do a memory load from the external reference table.
//         LoadU64(scratch,
//                 MemOperand(kRootRegister,
//                            RootRegisterOffsetForExternalReferenceTableEntry(
//                                isolate(), reference)));
//         return MemOperand(scratch, 0);
//       }
//     }
//   }
//   Move(scratch, reference);
//   return MemOperand(scratch, 0);
// }

// void MacroAssembler::Jump(intptr_t target, RelocInfo::Mode rmode,
//                           Condition cond, CRegister cr) {
//   Label skip;

//   if (cond != al) b(NegateCondition(cond), &skip, cr);

//   mov(ip, Operand(target, rmode));
//   mtctr(ip);
//   bctr();

//   bind(&skip);
// }

// void MacroAssembler::Jump(Address target, RelocInfo::Mode rmode, Condition cond,
//                           CRegister cr) {
//   DCHECK(!RelocInfo::IsCodeTarget(rmode));
//   Jump(static_cast<intptr_t>(target), rmode, cond, cr);
// }

// void MacroAssembler::Jump(Handle<Code> code, RelocInfo::Mode rmode,
//                           Condition cond, CRegister cr) {
//   DCHECK(RelocInfo::IsCodeTarget(rmode));
//   DCHECK_IMPLIES(options().isolate_independent_code,
//                  Builtins::IsIsolateIndependentBuiltin(*code));

//   Builtin builtin = Builtin::kNoBuiltinId;
//   if (isolate()->builtins()->IsBuiltinHandle(code, &builtin)) {
//     TailCallBuiltin(builtin, cond, cr);
//     return;
//   }
//   int32_t target_index = AddCodeTarget(code);
//   Jump(static_cast<intptr_t>(target_index), rmode, cond, cr);
// }

// void MacroAssembler::Jump(const ExternalReference& reference) {
//   UseScratchRegisterScope temps(this);
//   Register scratch = temps.Acquire();
//   Move(scratch, reference);
//   if (ABI_USES_FUNCTION_DESCRIPTORS) {
//     // AIX uses a function descriptor. When calling C code be
//     // aware of this descriptor and pick up values from it.
//     LoadU64(ToRegister(ABI_TOC_REGISTER),
//             MemOperand(scratch, kSystemPointerSize));
//     LoadU64(scratch, MemOperand(scratch, 0));
//   }
//   Jump(scratch);
// }

// void MacroAssembler::Call(Register target) {
//   BlockTrampolinePoolScope block_trampoline_pool(this);
//   // branch via link register and set LK bit for return point
//   mtctr(target);
//   bctrl();
// }

// void MacroAssembler::CallJSEntry(Register target) {
//   CHECK(target == r5);
//   Call(target);
// }

// int MacroAssembler::CallSizeNotPredictableCodeSize(Address target,
//                                                    RelocInfo::Mode rmode,
//                                                    Condition cond) {
//   return (2 + kMovInstructionsNoConstantPool) * kInstrSize;
// }

// void MacroAssembler::Call(Address target, RelocInfo::Mode rmode,
//                           Condition cond) {
//   BlockTrampolinePoolScope block_trampoline_pool(this);
//   DCHECK(cond == al);

//   // This can likely be optimized to make use of bc() with 24bit relative
//   //
//   // RecordRelocInfo(x.rmode_, x.immediate);
//   // bc( BA, .... offset, LKset);
//   //

//   mov(ip, Operand(target, rmode));
//   mtctr(ip);
//   bctrl();
// }

// void MacroAssembler::Call(Handle<Code> code, RelocInfo::Mode rmode,
//                           Condition cond) {
//   BlockTrampolinePoolScope block_trampoline_pool(this);
//   DCHECK(RelocInfo::IsCodeTarget(rmode));
//   DCHECK_IMPLIES(options().isolate_independent_code,
//                  Builtins::IsIsolateIndependentBuiltin(*code));

//   Builtin builtin = Builtin::kNoBuiltinId;
//   if (isolate()->builtins()->IsBuiltinHandle(code, &builtin)) {
//     CallBuiltin(builtin, cond);
//     return;
//   }
//   int32_t target_index = AddCodeTarget(code);
//   Call(static_cast<Address>(target_index), rmode, cond);
// }

// void MacroAssembler::CallBuiltin(Builtin builtin, Condition cond) {
//   ASM_CODE_COMMENT_STRING(this, CommentForOffHeapTrampoline("call", builtin));
//   // Use ip directly instead of using UseScratchRegisterScope, as we do not
//   // preserve scratch registers across calls.
//   switch (options().builtin_call_jump_mode) {
//     case BuiltinCallJumpMode::kAbsolute: {
//       Label skip;
//       mov(ip, Operand(BuiltinEntry(builtin), RelocInfo::OFF_HEAP_TARGET));
//       if (cond != al) b(NegateCondition(cond), &skip);
//       Call(ip);
//       bind(&skip);
//       break;
//     }
//     case BuiltinCallJumpMode::kPCRelative:
//       UNREACHABLE();
//     case BuiltinCallJumpMode::kIndirect: {
//       Label skip;
//       LoadU64(ip, EntryFromBuiltinAsOperand(builtin), r0);
//       if (cond != al) b(NegateCondition(cond), &skip);
//       Call(ip);
//       bind(&skip);
//       break;
//     }
//     case BuiltinCallJumpMode::kForMksnapshot: {
//       if (options().use_pc_relative_calls_and_jumps_for_mksnapshot) {
//         Handle<Code> code = isolate()->builtins()->code_handle(builtin);
//         int32_t code_target_index = AddCodeTarget(code);
//         Call(static_cast<Address>(code_target_index), RelocInfo::CODE_TARGET,
//              cond);
//       } else {
//         Label skip;
//         LoadU64(ip, EntryFromBuiltinAsOperand(builtin), r0);
//         if (cond != al) b(NegateCondition(cond), &skip);
//         Call(ip);
//         bind(&skip);
//       }
//       break;
//     }
//   }
// }

// void MacroAssembler::TailCallBuiltin(Builtin builtin, Condition cond,
//                                      CRegister cr) {
//   ASM_CODE_COMMENT_STRING(this,
//                           CommentForOffHeapTrampoline("tail call", builtin));
//   // Use ip directly instead of using UseScratchRegisterScope, as we do not
//   // preserve scratch registers across calls.
//   switch (options().builtin_call_jump_mode) {
//     case BuiltinCallJumpMode::kAbsolute: {
//       Label skip;
//       mov(ip, Operand(BuiltinEntry(builtin), RelocInfo::OFF_HEAP_TARGET));
//       if (cond != al) b(NegateCondition(cond), &skip, cr);
//       Jump(ip);
//       bind(&skip);
//       break;
//     }
//     case BuiltinCallJumpMode::kPCRelative:
//       UNREACHABLE();
//     case BuiltinCallJumpMode::kIndirect: {
//       Label skip;
//       LoadU64(ip, EntryFromBuiltinAsOperand(builtin), r0);
//       if (cond != al) b(NegateCondition(cond), &skip, cr);
//       Jump(ip);
//       bind(&skip);
//       break;
//     }
//     case BuiltinCallJumpMode::kForMksnapshot: {
//       if (options().use_pc_relative_calls_and_jumps_for_mksnapshot) {
//         Handle<Code> code = isolate()->builtins()->code_handle(builtin);
//         int32_t code_target_index = AddCodeTarget(code);
//         Jump(static_cast<intptr_t>(code_target_index), RelocInfo::CODE_TARGET,
//              cond, cr);
//       } else {
//         Label skip;
//         LoadU64(ip, EntryFromBuiltinAsOperand(builtin), r0);
//         if (cond != al) b(NegateCondition(cond), &skip, cr);
//         Jump(ip);
//         bind(&skip);
//       }
//       break;
//     }
//   }
// }

// void MacroAssembler::Drop(int count) {
//   if (count > 0) {
//     AddS64(sp, sp, Operand(count * kSystemPointerSize), r0);
//   }
// }

// void MacroAssembler::Drop(Register count, Register scratch) {
//   ShiftLeftU64(scratch, count, Operand(kSystemPointerSizeLog2));
//   add(sp, sp, scratch);
// }

// Enforce alignment of sp.
// void MacroAssembler::EnforceStackAlignment() {
//   int frame_alignment = ActivationFrameAlignment();
//   DCHECK(base::bits::IsPowerOfTwo(frame_alignment));

//   uint64_t frame_alignment_mask = ~(static_cast<uint64_t>(frame_alignment) - 1);
//   AndU64(sp, sp, Operand(frame_alignment_mask));
// }

// void MacroAssembler::TestCodeIsMarkedForDeoptimization(Register code,
//                                                        Register scratch1,
//                                                        Register scratch2) {
//   LoadU32(scratch1, FieldMemOperand(code, Code::kFlagsOffset), scratch2);
//   TestBit(scratch1, Code::kMarkedForDeoptimizationBit, scratch2);
// }

// Operand MacroAssembler::ClearedValue() const {
//   return Operand(static_cast<int32_t>(i::ClearedValue(isolate()).ptr()));
// }

// void MacroAssembler::Call(Label* target) { b(target, SetLK); }

// void MacroAssembler::Push(Handle<HeapObject> handle) {
//   mov(r0, Operand(handle));
//   push(r0);
// }

// void MacroAssembler::Push(Tagged<Smi> smi) {
//   mov(r0, Operand(smi));
//   push(r0);
// }

// void MacroAssembler::PushArray(Register array, Register size, Register scratch,
//                                Register scratch2, PushArrayOrder order) {
//   Label loop, done;

//   if (order == kNormal) {
//     cmpi(size, Operand::Zero());
//     beq(&done);
//     ShiftLeftU64(scratch, size, Operand(kSystemPointerSizeLog2));
//     add(scratch, array, scratch);
//     mtctr(size);

//     bind(&loop);
//     LoadU64WithUpdate(scratch2, MemOperand(scratch, -kSystemPointerSize));
//     StoreU64WithUpdate(scratch2, MemOperand(sp, -kSystemPointerSize));
//     bdnz(&loop);

//     bind(&done);
//   } else {
//     cmpi(size, Operand::Zero());
//     beq(&done);

//     mtctr(size);
//     subi(scratch, array, Operand(kSystemPointerSize));

//     bind(&loop);
//     LoadU64WithUpdate(scratch2, MemOperand(scratch, kSystemPointerSize));
//     StoreU64WithUpdate(scratch2, MemOperand(sp, -kSystemPointerSize));
//     bdnz(&loop);
//     bind(&done);
//   }
// }

// void MacroAssembler::Move(Register dst, Handle<HeapObject> value,
//                           RelocInfo::Mode rmode) {
//   // TODO(jgruber,v8:8887): Also consider a root-relative load when generating
//   // non-isolate-independent code. In many cases it might be cheaper than
//   // embedding the relocatable value.
//   if (root_array_available_ && options().isolate_independent_code) {
//     IndirectLoadConstant(dst, value);
//     return;
//   } else if (RelocInfo::IsCompressedEmbeddedObject(rmode)) {
//     EmbeddedObjectIndex index = AddEmbeddedObject(value);
//     DCHECK(is_uint32(index));
//     mov(dst, Operand(static_cast<int>(index), rmode));
//   } else {
//     DCHECK(RelocInfo::IsFullEmbeddedObject(rmode));
//     mov(dst, Operand(value.address(), rmode));
//   }
// }

// void MacroAssembler::Move(Register dst, ExternalReference reference) {
//   if (root_array_available()) {
//     if (reference.IsIsolateFieldId()) {
//       AddS64(dst, kRootRegister,
//              Operand(reference.offset_from_root_register()));
//       return;
//     }
//     if (options().isolate_independent_code) {
//       IndirectLoadExternalReference(dst, reference);
//       return;
//     }
//   }

//   // External references should not get created with IDs if
//   // `!root_array_available()`.
//   CHECK(!reference.IsIsolateFieldId());
//   mov(dst, Operand(reference));
// }

// void MacroAssembler::LoadIsolateField(Register dst, IsolateFieldId id) {
//   Move(dst, ExternalReference::Create(id));
// }

// void MacroAssembler::Move(Register dst, Register src, Condition cond) {
//   DCHECK(cond == al);
//   if (dst != src) {
//     mr(dst, src);
//   }
// }

// void MacroAssembler::Move(DoubleRegister dst, DoubleRegister src) {
//   if (dst != src) {
//     fmr(dst, src);
//   }
// }

// void MacroAssembler::MultiPush(RegList regs, Register location) {
//   int16_t num_to_push = regs.Count();
//   int16_t stack_offset = num_to_push * kSystemPointerSize;

//   subi(location, location, Operand(stack_offset));
//   for (int16_t i = Register::kNumRegisters - 1; i >= 0; i--) {
//     if ((regs.bits() & (1 << i)) != 0) {
//       stack_offset -= kSystemPointerSize;
//       StoreU64(ToRegister(i), MemOperand(location, stack_offset));
//     }
//   }
// }

// void MacroAssembler::MultiPop(RegList regs, Register location) {
//   int16_t stack_offset = 0;

//   for (int16_t i = 0; i < Register::kNumRegisters; i++) {
//     if ((regs.bits() & (1 << i)) != 0) {
//       LoadU64(ToRegister(i), MemOperand(location, stack_offset));
//       stack_offset += kSystemPointerSize;
//     }
//   }
//   addi(location, location, Operand(stack_offset));
// }

// void MacroAssembler::MultiPushDoubles(DoubleRegList dregs, Register location) {
//   int16_t num_to_push = dregs.Count();
//   int16_t stack_offset = num_to_push * kDoubleSize;

//   subi(location, location, Operand(stack_offset));
//   for (int16_t i = DoubleRegister::kNumRegisters - 1; i >= 0; i--) {
//     if ((dregs.bits() & (1 << i)) != 0) {
//       DoubleRegister dreg = DoubleRegister::from_code(i);
//       stack_offset -= kDoubleSize;
//       stfd(dreg, MemOperand(location, stack_offset));
//     }
//   }
// }

// void MacroAssembler::MultiPushV128(Simd128RegList simd_regs, Register scratch,
//                                    Register location) {
//   int16_t num_to_push = simd_regs.Count();
//   int16_t stack_offset = num_to_push * kSimd128Size;

//   subi(location, location, Operand(stack_offset));
//   for (int16_t i = Simd128Register::kNumRegisters - 1; i >= 0; i--) {
//     if ((simd_regs.bits() & (1 << i)) != 0) {
//       Simd128Register simd_reg = Simd128Register::from_code(i);
//       stack_offset -= kSimd128Size;
//       StoreSimd128(simd_reg, MemOperand(location, stack_offset), scratch);
//     }
//   }
// }

// void MacroAssembler::MultiPopDoubles(DoubleRegList dregs, Register location) {
//   int16_t stack_offset = 0;

//   for (int16_t i = 0; i < DoubleRegister::kNumRegisters; i++) {
//     if ((dregs.bits() & (1 << i)) != 0) {
//       DoubleRegister dreg = DoubleRegister::from_code(i);
//       lfd(dreg, MemOperand(location, stack_offset));
//       stack_offset += kDoubleSize;
//     }
//   }
//   addi(location, location, Operand(stack_offset));
// }

// void MacroAssembler::MultiPopV128(Simd128RegList simd_regs, Register scratch,
//                                   Register location) {
//   int16_t stack_offset = 0;

//   for (int16_t i = 0; i < Simd128Register::kNumRegisters; i++) {
//     if ((simd_regs.bits() & (1 << i)) != 0) {
//       Simd128Register simd_reg = Simd128Register::from_code(i);
//       LoadSimd128(simd_reg, MemOperand(location, stack_offset), scratch);
//       stack_offset += kSimd128Size;
//     }
//   }
//   addi(location, location, Operand(stack_offset));
// }

// void MacroAssembler::MultiPushF64AndV128(DoubleRegList dregs,
//                                          Simd128RegList simd_regs,
//                                          Register scratch1, Register scratch2,
//                                          Register location) {
//   MultiPushDoubles(dregs);
// #if V8_ENABLE_WEBASSEMBLY
//   bool generating_bultins =
//       isolate() && isolate()->IsGeneratingEmbeddedBuiltins();
//   if (generating_bultins) {
//     // V8 uses the same set of fp param registers as Simd param registers.
//     // As these registers are two different sets on ppc we must make
//     // sure to also save them when Simd is enabled.
//     // Check the comments under crrev.com/c/2645694 for more details.
//     Label push_empty_simd, simd_pushed;
//     Move(scratch1, ExternalReference::supports_wasm_simd_128_address());
//     LoadU8(scratch1, MemOperand(scratch1), scratch2);
//     cmpi(scratch1, Operand::Zero());  // If > 0 then simd is available.
//     ble(&push_empty_simd);
//     MultiPushV128(simd_regs, scratch1);
//     b(&simd_pushed);
//     bind(&push_empty_simd);
//     // We still need to allocate empty space on the stack even if we
//     // are not pushing Simd registers (see kFixedFrameSizeFromFp).
//     addi(sp, sp,
//          Operand(-static_cast<int8_t>(simd_regs.Count()) * kSimd128Size));
//     bind(&simd_pushed);
//   } else {
//     if (CpuFeatures::SupportsWasmSimd128()) {
//       MultiPushV128(simd_regs, scratch1);
//     } else {
//       addi(sp, sp,
//            Operand(-static_cast<int8_t>(simd_regs.Count()) * kSimd128Size));
//     }
//   }
// #endif
// }

// void MacroAssembler::MultiPopF64AndV128(DoubleRegList dregs,
//                                         Simd128RegList simd_regs,
//                                         Register scratch1, Register scratch2,
//                                         Register location) {
// #if V8_ENABLE_WEBASSEMBLY
//   bool generating_bultins =
//       isolate() && isolate()->IsGeneratingEmbeddedBuiltins();
//   if (generating_bultins) {
//     Label pop_empty_simd, simd_popped;
//     Move(scratch1, ExternalReference::supports_wasm_simd_128_address());
//     LoadU8(scratch1, MemOperand(scratch1), scratch2);
//     cmpi(scratch1, Operand::Zero());  // If > 0 then simd is available.
//     ble(&pop_empty_simd);
//     MultiPopV128(simd_regs, scratch1);
//     b(&simd_popped);
//     bind(&pop_empty_simd);
//     addi(sp, sp,
//          Operand(static_cast<int8_t>(simd_regs.Count()) * kSimd128Size));
//     bind(&simd_popped);
//   } else {
//     if (CpuFeatures::SupportsWasmSimd128()) {
//       MultiPopV128(simd_regs, scratch1);
//     } else {
//       addi(sp, sp,
//            Operand(static_cast<int8_t>(simd_regs.Count()) * kSimd128Size));
//     }
//   }
// #endif
//   MultiPopDoubles(dregs);
// }

// void MacroAssembler::LoadTaggedRoot(Register destination, RootIndex index) {
//   ASM_CODE_COMMENT(this);
//   if (CanBeImmediate(index)) {
//     mov(destination, Operand(ReadOnlyRootPtr(index), RelocInfo::Mode::NO_INFO));
//     return;
//   }
//   LoadRoot(destination, index);
// }

// void MacroAssembler::LoadRoot(Register destination, RootIndex index,
//                               Condition cond) {
//   DCHECK(cond == al);
//   if (CanBeImmediate(index)) {
//     DecompressTagged(destination, ReadOnlyRootPtr(index));
//     return;
//   }
//   LoadU64(destination,
//           MemOperand(kRootRegister, RootRegisterOffsetForRootIndex(index)), r0);
// }

// void MacroAssembler::LoadTaggedField(const Register& destination,
//                                      const MemOperand& field_operand,
//                                      const Register& scratch) {
//   if (COMPRESS_POINTERS_BOOL) {
//     DecompressTagged(destination, field_operand);
//   } else {
//     LoadU64(destination, field_operand, scratch);
//   }
// }

// void MacroAssembler::SmiUntag(Register dst, const MemOperand& src, RCBit rc,
//                               Register scratch) {
//   if (SmiValuesAre31Bits()) {
//     LoadU32(dst, src, scratch);
//   } else {
//     LoadU64(dst, src, scratch);
//   }

//   SmiUntag(dst, rc);
// }

// void MacroAssembler::StoreTaggedField(const Register& value,
//                                       const MemOperand& dst_field_operand,
//                                       const Register& scratch) {
//   if (COMPRESS_POINTERS_BOOL) {
//     RecordComment("[ StoreTagged");
//     StoreU32(value, dst_field_operand, scratch);
//     RecordComment("]");
//   } else {
//     StoreU64(value, dst_field_operand, scratch);
//   }
// }

// void MacroAssembler::DecompressTaggedSigned(