// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/codegen/reloc-info.h

pub mod reloc_info {
    use std::mem;
    use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
    //use crate::base::export_template::*; // Assuming a custom export-template
    //use crate::common::code_memory_access::*; // Assuming a custom code memory access
    //use crate::common::globals::*; // Assuming a custom globals definition
    //use crate::objects::code::*; // Assuming a custom code definition
    //use crate::objects::instruction_stream::*; // Assuming a custom instruction stream definition

    // Placeholder types and constants for the V8 API.  These need to be replaced with
    // actual Rust types that map to the V8 C++ API, but this is beyond the scope of
    // a simple conversion.
    pub type Address = usize;
    pub type intptr_t = isize;
    pub type uint32_t = u32;
    pub type uint8_t = u8;

    pub const kNullAddress: Address = 0;
    pub const kBitsPerByte: usize = 8;
    pub const kBitsPerInt: usize = mem::size_of::<usize>() * 8;
    pub const kSystemPointerSize: usize = mem::size_of::<usize>();

    // Placeholder. Define based on compilation target in build script.
    pub const COMPRESS_POINTERS_BOOL: bool = false;

    // Placeholder type for InstructionStream.
    #[derive(Debug, Copy, Clone)]
    pub struct InstructionStream {}
    impl InstructionStream {
        pub fn new() -> Self {
            InstructionStream{}
        }
    }

    // Placeholder type for Code.
    #[derive(Debug, Copy, Clone)]
    pub struct Code {}

    // Placeholder type for HeapObject.
    #[derive(Debug, Copy, Clone)]
    pub struct HeapObject {}

    #[derive(Debug, Copy, Clone)]
    pub struct Tagged<T>(T);

    impl Tagged<HeapObject> {
        pub fn new() -> Self {
            Tagged(HeapObject{})
        }
    }
    impl Tagged<InstructionStream> {
        pub fn new() -> Self {
            Tagged(InstructionStream{})
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct PtrComprCageBase {}

    // Placeholder
    pub type WriteBarrierMode = i32;
    pub const UPDATE_WRITE_BARRIER: WriteBarrierMode = 0;

    // Placeholder type for WasmCodePointer.
    #[derive(Debug, Copy, Clone)]
    pub struct WasmCodePointer {}

    // Placeholder for Assembler.
    pub struct Assembler {}

    // Placeholder for Builtin
    pub struct Builtin {}

    // Placeholder for JSDispatchHandle
    pub struct JSDispatchHandle {}

    // Placeholder for EmbeddedData.
    pub struct EmbeddedData {}

    // Placeholder
    pub type ObjectVisitor = i32;

    // Placeholder
    pub type Isolate = i32;

    // Placeholder
    pub type WritableJitAllocation = i32;

    /// Specifies whether to perform icache flush operations on RelocInfo updates.
    /// If `FLUSH_ICACHE_IF_NEEDED`, the icache will always be flushed if an
    /// instruction was modified. If `SKIP_ICACHE_FLUSH` the flush will always be
    /// skipped (only use this if you will flush the icache manually before it is
    /// executed).
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ICacheFlushMode {
        FLUSH_ICACHE_IF_NEEDED,
        SKIP_ICACHE_FLUSH,
    }

    mod detail {
        pub const kTagBits: usize = 2;
        pub const kTagMask: usize = (1 << kTagBits) - 1;
        pub const kLongTagBits: usize = 6;

        pub const kEmbeddedObjectTag: usize = 0;
        pub const kCodeTargetTag: usize = 1;
        pub const kWasmStubCallTag: usize = 2;
        pub const kDefaultTag: usize = 3;

        pub const kSmallPCDeltaBits: usize = super::kBitsPerByte - kTagBits;
        pub const kSmallPCDeltaMask: usize = (1 << kSmallPCDeltaBits) - 1;
    }

    /// Relocation information
    ///
    /// Relocation information consists of the address (pc) of the datum
    /// to which the relocation information applies, the relocation mode
    /// (rmode), and an optional data field. The relocation mode may be
    /// "descriptive" and not indicate a need for relocation, but simply
    /// describe a property of the datum. Such rmodes are useful for GC
    /// and nice disassembly output.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct RelocInfo {
        pc_: Address,
        rmode_: Mode,
        data_: intptr_t,
        constant_pool_: Address,
    }

    impl RelocInfo {
        /// The minimum size of a comment is equal to two bytes for the extra tagged
        /// pc and `kSystemPointerSize` for the actual pointer to the comment.
        pub const kMinRelocCommentSize: usize = 2 + kSystemPointerSize;

        /// The maximum size for a call instruction including pc-jump.
        pub const kMaxCallSize: usize = 6;

        /// The maximum pc delta that will use the short encoding.
        pub const kMaxSmallPCDelta: usize = detail::kSmallPCDeltaMask;

        #[allow(non_camel_case_types)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum Mode {
            // Please note the order is important (see IsRealRelocMode, IsGCRelocMode,
            // and IsShareableRelocMode predicates below).

            NO_INFO,  // Never recorded value. Most common one, hence value 0.

            CODE_TARGET,
            // TODO(ishell): rename to NEAR_CODE_TARGET.
            RELATIVE_CODE_TARGET,  // LAST_CODE_TARGET_MODE
            COMPRESSED_EMBEDDED_OBJECT,
            FULL_EMBEDDED_OBJECT,  // LAST_GCED_ENUM

            WASM_CALL,  // FIRST_SHAREABLE_RELOC_MODE
            WASM_STUB_CALL,
            WASM_CODE_POINTER_TABLE_ENTRY,
            WASM_CANONICAL_SIG_ID,

            EXTERNAL_REFERENCE,  // The address of an external C++ function.
            INTERNAL_REFERENCE,  // An address inside the same function.

            // Encoded internal reference, used only on RISCV64, RISCV32, MIPS64
            // and PPC.
            INTERNAL_REFERENCE_ENCODED,

            // An integer JSDispatchHandle, referring to an entry in the
            // JSDispatchTable.
            JS_DISPATCH_HANDLE,

            // An off-heap instruction stream target. See http://goo.gl/Z2HUiM.
            // TODO(ishell): rename to BUILTIN_ENTRY.
            OFF_HEAP_TARGET,  // FIRST_BUILTIN_ENTRY_MODE
            // An un-embedded off-heap instruction stream target.
            // See http://crbug.com/v8/11527 for details.
            NEAR_BUILTIN_ENTRY,  // LAST_BUILTIN_ENTRY_MODE

            // Marks constant and veneer pools. Only used on ARM and ARM64.
            // They use a custom noncompact encoding.
            CONST_POOL,
            VENEER_POOL,

            DEOPT_SCRIPT_OFFSET,
            DEOPT_INLINING_ID,  // Deoptimization source position.
            DEOPT_REASON,       // Deoptimization reason index.
            DEOPT_ID,           // Deoptimization inlining id.
            DEOPT_NODE_ID,      // Id of the node that caused deoptimization. This
                                // information is only recorded in debug builds.

            // This is not an actual reloc mode, but used to encode a long pc jump that
            // cannot be encoded as part of another record.
            PC_JUMP,

            // Pseudo-types
            NUMBER_OF_MODES,

            LAST_CODE_TARGET_MODE,
            FIRST_REAL_RELOC_MODE,
            LAST_REAL_RELOC_MODE,
            FIRST_EMBEDDED_OBJECT_RELOC_MODE,
            LAST_EMBEDDED_OBJECT_RELOC_MODE,
            LAST_GCED_ENUM,
            FIRST_BUILTIN_ENTRY_MODE,
            LAST_BUILTIN_ENTRY_MODE,
            FIRST_SHAREABLE_RELOC_MODE,
        }

        const NO_INFO: Mode = Mode::NO_INFO;
        const CODE_TARGET: Mode = Mode::CODE_TARGET;
        const RELATIVE_CODE_TARGET: Mode = Mode::RELATIVE_CODE_TARGET;
        const COMPRESSED_EMBEDDED_OBJECT: Mode = Mode::COMPRESSED_EMBEDDED_OBJECT;
        const FULL_EMBEDDED_OBJECT: Mode = Mode::FULL_EMBEDDED_OBJECT;
        const WASM_CALL: Mode = Mode::WASM_CALL;
        const WASM_STUB_CALL: Mode = Mode::WASM_STUB_CALL;
        const WASM_CODE_POINTER_TABLE_ENTRY: Mode = Mode::WASM_CODE_POINTER_TABLE_ENTRY;
        const WASM_CANONICAL_SIG_ID: Mode = Mode::WASM_CANONICAL_SIG_ID;
        const EXTERNAL_REFERENCE: Mode = Mode::EXTERNAL_REFERENCE;
        const INTERNAL_REFERENCE: Mode = Mode::INTERNAL_REFERENCE;
        const INTERNAL_REFERENCE_ENCODED: Mode = Mode::INTERNAL_REFERENCE_ENCODED;
        const JS_DISPATCH_HANDLE: Mode = Mode::JS_DISPATCH_HANDLE;
        const OFF_HEAP_TARGET: Mode = Mode::OFF_HEAP_TARGET;
        const NEAR_BUILTIN_ENTRY: Mode = Mode::NEAR_BUILTIN_ENTRY;
        const CONST_POOL: Mode = Mode::CONST_POOL;
        const VENEER_POOL: Mode = Mode::VENEER_POOL;
        const DEOPT_SCRIPT_OFFSET: Mode = Mode::DEOPT_SCRIPT_OFFSET;
        const DEOPT_INLINING_ID: Mode = Mode::DEOPT_INLINING_ID;
        const DEOPT_REASON: Mode = Mode::DEOPT_REASON;
        const DEOPT_ID: Mode = Mode::DEOPT_ID;
        const DEOPT_NODE_ID: Mode = Mode::DEOPT_NODE_ID;
        const PC_JUMP: Mode = Mode::PC_JUMP;
        const NUMBER_OF_MODES: Mode = Mode::NUMBER_OF_MODES;
        const LAST_CODE_TARGET_MODE: Mode = Mode::RELATIVE_CODE_TARGET;
        const FIRST_REAL_RELOC_MODE: Mode = Mode::CODE_TARGET;
        const LAST_REAL_RELOC_MODE: Mode = Mode::VENEER_POOL;
        const FIRST_EMBEDDED_OBJECT_RELOC_MODE: Mode = Mode::COMPRESSED_EMBEDDED_OBJECT;
        const LAST_EMBEDDED_OBJECT_RELOC_MODE: Mode = Mode::FULL_EMBEDDED_OBJECT;
        const LAST_GCED_ENUM: Mode = Mode::LAST_EMBEDDED_OBJECT_RELOC_MODE;
        const FIRST_BUILTIN_ENTRY_MODE: Mode = Mode::OFF_HEAP_TARGET;
        const LAST_BUILTIN_ENTRY_MODE: Mode = Mode::NEAR_BUILTIN_ENTRY;
        const FIRST_SHAREABLE_RELOC_MODE: Mode = Mode::WASM_CALL;

        pub const kApplyMask: i32 = 0; // Placeholder - platform specific value

        pub const fn ModeMask(mode: Mode) -> i32 {
            1 << (mode as i32)
        }
        
        const_assert!(Mode::NUMBER_OF_MODES as usize <= kBitsPerInt);

        pub fn new(pc: Address, rmode: Mode, data: intptr_t, constant_pool: Address) -> Self {
            RelocInfo {
                pc_: pc,
                rmode_: rmode,
                data_: data,
                constant_pool_: constant_pool,
            }
        }

        // Convenience ctor.
        pub fn new_simple(pc: Address, rmode: Mode) -> Self {
            RelocInfo::new(pc, rmode, 0, kNullAddress)
        }

        pub const fn IsRealRelocMode(mode: Mode) -> bool {
            (mode as i32) >= (Self::FIRST_REAL_RELOC_MODE as i32) && (mode as i32) <= (Self::LAST_REAL_RELOC_MODE as i32)
        }
        // Is the relocation mode affected by GC?
        pub const fn IsGCRelocMode(mode: Mode) -> bool {
            (mode as i32) <= (Self::LAST_GCED_ENUM as i32)
        }
        pub const fn IsShareableRelocMode(mode: Mode) -> bool {
            mode == RelocInfo::NO_INFO || (mode as i32) >= (Self::FIRST_SHAREABLE_RELOC_MODE as i32)
        }
        pub const fn IsCodeTarget(mode: Mode) -> bool {
            mode == RelocInfo::CODE_TARGET
        }
        pub const fn IsCodeTargetMode(mode: Mode) -> bool {
            (mode as i32) <= (Self::LAST_CODE_TARGET_MODE as i32)
        }
        pub const fn IsRelativeCodeTarget(mode: Mode) -> bool {
            mode == RelocInfo::RELATIVE_CODE_TARGET
        }
        pub const fn IsFullEmbeddedObject(mode: Mode) -> bool {
            mode == RelocInfo::FULL_EMBEDDED_OBJECT
        }
        pub const fn IsCompressedEmbeddedObject(mode: Mode) -> bool {
            COMPRESS_POINTERS_BOOL && mode == RelocInfo::COMPRESSED_EMBEDDED_OBJECT
        }
        pub const fn IsEmbeddedObjectMode(mode: Mode) -> bool {
            Self::is_in_range(mode, RelocInfo::FIRST_EMBEDDED_OBJECT_RELOC_MODE,
                           RelocInfo::LAST_EMBEDDED_OBJECT_RELOC_MODE)
        }
        pub const fn IsWasmCall(mode: Mode) -> bool {
            mode == RelocInfo::WASM_CALL
        }
        pub const fn IsWasmStubCall(mode: Mode) -> bool {
            mode == RelocInfo::WASM_STUB_CALL
        }
        pub const fn IsWasmCanonicalSigId(mode: Mode) -> bool {
            mode == RelocInfo::WASM_CANONICAL_SIG_ID
        }
        pub const fn IsWasmCodePointerTableEntry(mode: Mode) -> bool {
            mode == RelocInfo::WASM_CODE_POINTER_TABLE_ENTRY
        }
        pub const fn IsConstPool(mode: Mode) -> bool {
            mode == RelocInfo::CONST_POOL
        }
        pub const fn IsVeneerPool(mode: Mode) -> bool {
            mode == RelocInfo::VENEER_POOL
        }
        pub const fn IsDeoptPosition(mode: Mode) -> bool {
            mode == RelocInfo::DEOPT_SCRIPT_OFFSET || mode == RelocInfo::DEOPT_INLINING_ID
        }
        pub const fn IsDeoptReason(mode: Mode) -> bool {
            mode == RelocInfo::DEOPT_REASON
        }
        pub const fn IsDeoptId(mode: Mode) -> bool {
            mode == RelocInfo::DEOPT_ID
        }
        pub const fn IsDeoptNodeId(mode: Mode) -> bool {
            mode == RelocInfo::DEOPT_NODE_ID
        }
        pub const fn IsExternalReference(mode: Mode) -> bool {
            mode == RelocInfo::EXTERNAL_REFERENCE
        }
        pub const fn IsInternalReference(mode: Mode) -> bool {
            mode == RelocInfo::INTERNAL_REFERENCE
        }
        pub const fn IsInternalReferenceEncoded(mode: Mode) -> bool {
            mode == RelocInfo::INTERNAL_REFERENCE_ENCODED
        }
        pub const fn IsOffHeapTarget(mode: Mode) -> bool {
            mode == RelocInfo::OFF_HEAP_TARGET
        }
        pub const fn IsNearBuiltinEntry(mode: Mode) -> bool {
            mode == RelocInfo::NEAR_BUILTIN_ENTRY
        }
        pub const fn IsBuiltinEntryMode(mode: Mode) -> bool {
            Self::is_in_range(mode, RelocInfo::FIRST_BUILTIN_ENTRY_MODE,
                           RelocInfo::LAST_BUILTIN_ENTRY_MODE)
        }
        pub const fn IsJSDispatchHandle(mode: Mode) -> bool {
            mode == RelocInfo::JS_DISPATCH_HANDLE
        }
        pub const fn IsNoInfo(mode: Mode) -> bool {
            mode == RelocInfo::NO_INFO
        }

        pub fn IsOnlyForSerializer(mode: Mode) -> bool {
            // #[cfg(V8_TARGET_ARCH_IA32)]
            // {
                // On ia32, inlined off-heap trampolines must be relocated.
                //assert_ne!((Self::kApplyMask & Self::ModeMask(RelocInfo::OFF_HEAP_TARGET)), 0);
                //assert_eq!((Self::kApplyMask & Self::ModeMask(RelocInfo::EXTERNAL_REFERENCE)), 0);
                //return mode == RelocInfo::EXTERNAL_REFERENCE;
            // }
            // #[cfg(not(V8_TARGET_ARCH_IA32))]
            // {
                //assert_eq!((Self::kApplyMask & Self::ModeMask(RelocInfo::OFF_HEAP_TARGET)), 0);
                //assert_eq!((Self::kApplyMask & Self::ModeMask(RelocInfo::EXTERNAL_REFERENCE)), 0);
                return mode == RelocInfo::EXTERNAL_REFERENCE || mode == RelocInfo::OFF_HEAP_TARGET;
            // }
        }

        pub fn pc(&self) -> Address {
            self.pc_
        }
        pub fn rmode(&self) -> Mode {
            self.rmode_
        }
        pub fn constant_pool(&self) -> Address {
            self.constant_pool_
        }
        pub fn data(&self) -> intptr_t {
            self.data_
        }

        // Is the pointer this relocation info refers to coded like a plain pointer
        // or is it strange in some way (e.g. relative or patched into a series of
        // instructions).
        pub fn IsCodedSpecially(&self) -> bool {
            todo!()
        }

        // The static pendant to IsCodedSpecially, just for off-heap targets. Used
        // during deserialization, when we don't actually have a RelocInfo handy.
        pub fn OffHeapTargetIsCodedSpecially() -> bool {
            todo!()
        }

        // If true, the pointer this relocation info refers to is an entry in the
        // constant pool, otherwise the pointer is embedded in the instruction stream.
        pub fn IsInConstantPool(&self) -> bool {
            todo!()
        }

        pub fn wasm_call_address(&self) -> Address {
            todo!()
        }
        pub fn wasm_stub_call_address(&self) -> Address {
            todo!()
        }
        pub fn wasm_canonical_sig_id(&self) -> uint32_t {
            todo!()
        }
        // #[inline]
        pub fn wasm_code_pointer_table_entry(&self) -> WasmCodePointer {
            todo!()
        }

        pub fn wasm_call_tag(&self) -> uint32_t {
            todo!()
        }

        pub fn set_off_heap_target_address(&mut self, _target: Address, _icache_flush_mode: ICacheFlushMode) {
            todo!()
        }

        // this relocation applies to;
        // can only be called if IsCodeTarget(rmode_)
        // #[inline]
        pub fn target_address(&self) -> Address {
            todo!()
        }
        // Cage base value is used for decompressing compressed embedded references.
        // #[inline]
        pub fn target_object(&self, _cage_base: PtrComprCageBase) -> Tagged<HeapObject> {
            todo!()
        }

        pub fn target_object_handle(&self, _origin: &Assembler) -> ! {
            todo!()
        }

        // Decodes builtin ID encoded as a PC-relative offset. This encoding is used
        // during code generation of call/jump with NEAR_BUILTIN_ENTRY.
        // #[inline]
        pub fn target_builtin_at(&self, _origin: &Assembler) -> Builtin {
            todo!()
        }
        // #[inline]
        pub fn target_off_heap_target(&self) -> Address {
            todo!()
        }

        // Returns the address of the constant pool entry where the target address
        // is held.  This should only be called if IsInConstantPool returns true.
        // #[inline]
        pub fn constant_pool_entry_address(&self) -> Address {
            todo!()
        }

        // Read the address of the word containing the target_address in an
        // instruction stream.  What this means exactly is architecture-independent.
        // The only architecture-independent user of this function is the serializer.
        // The serializer uses it to find out how many raw bytes of instruction to
        // output before the next target.  Architecture-independent code shouldn't
        // dereference the pointer it gets back from this.
        // #[inline]
        pub fn target_address_address(&self) -> Address {
            todo!()
        }
        pub fn HasTargetAddressAddress(&self) -> bool {
            todo!()
        }

        // This indicates how much space a target takes up when deserializing a code
        // stream.  For most architectures this is just the size of a pointer.  For
        // an instruction like movw/movt where the target bits are mixed into the
        // instruction bits the size of the target will be zero, indicating that the
        // serializer should not step forwards in memory after a target is resolved
        // and written.  In this case the target_address_address function above
        // should return the end of the instructions to be patched, allowing the
        // deserializer to deserialize the instructions as raw bytes and put them in
        // place, ready to be patched with the target.
        // #[inline]
        pub fn target_address_size(&self) -> i32 {
            todo!()
        }

        // Read the reference in the instruction this relocation
        // applies to; can only be called if rmode_ is EXTERNAL_REFERENCE.
        // #[inline]
        pub fn target_external_reference(&self) -> Address {
            todo!()
        }

        // Read the reference in the instruction this relocation
        // applies to; can only be called if rmode_ is INTERNAL_REFERENCE.
        // #[inline]
        pub fn target_internal_reference(&self) -> Address {
            todo!()
        }

        // Return the reference address this relocation applies to;
        // can only be called if rmode_ is INTERNAL_REFERENCE.
        // #[inline]
        pub fn target_internal_reference_address(&self) -> Address {
            todo!()
        }

        // Return the JSDispatchHandle this relocation applies to;
        // can only be called if rmode_ is JS_DISPATCH_HANDLE.
        // #[inline]
        pub fn js_dispatch_handle(&self) -> JSDispatchHandle {
            todo!()
        }

        pub fn Visit(&self, _host: Tagged<InstructionStream>, _visitor: *mut ObjectVisitor) {
            todo!()
        }

        // Printing
        pub fn RelocModeName(_rmode: Mode) -> &'static str {
            todo!()
        }
        pub fn Print(&self, _isolate: *mut Isolate, _os: &mut std::ostream) {
            todo!()
        }

        pub fn Verify(&self, _isolate: *mut Isolate) {
            todo!()
        }

        pub const fn AllRealModesMask() -> i32 {
            let kFirstUnrealRelocMode =
                RelocInfo::LAST_REAL_RELOC_MODE as i32 + 1;
            ((Self::ModeMask(Mode::from_i32(kFirstUnrealRelocMode).unwrap()) - 1) &
            !(Self::ModeMask(RelocInfo::FIRST_REAL_RELOC_MODE) - 1)) as i32
        }

        pub fn EmbeddedObjectModeMask() -> i32 {
            Self::ModeMask(RelocInfo::FULL_EMBEDDED_OBJECT) |
            Self::ModeMask(RelocInfo::COMPRESSED_EMBEDDED_OBJECT)
        }

        // In addition to modes covered by the apply mask (which is applied at GC
        // time, among others), this covers all modes that are relocated by
        // InstructionStream::CopyFromNoFlush after code generation.
        pub fn PostCodegenRelocationMask() -> i32 {
            Self::ModeMask(RelocInfo::CODE_TARGET) |
            Self::ModeMask(RelocInfo::COMPRESSED_EMBEDDED_OBJECT) |
            Self::ModeMask(RelocInfo::FULL_EMBEDDED_OBJECT) |
            Self::ModeMask(RelocInfo::NEAR_BUILTIN_ENTRY) |
            Self::ModeMask(RelocInfo::WASM_STUB_CALL) |
            Self::ModeMask(RelocInfo::RELATIVE_CODE_TARGET) | Self::kApplyMask
        }

        const fn is_in_range(mode: Mode, first: Mode, last: Mode) -> bool {
            (mode as i32) >= (first as i32) && (mode as i32) <= (last as i32)
        }
    }

    impl Default for RelocInfo {
        fn default() -> Self {
            RelocInfo {
                pc_: kNullAddress,
                rmode_: Mode::NO_INFO,
                data_: 0,
                constant_pool_: kNullAddress,
            }
        }
    }

    impl From<i32> for RelocInfo::Mode {
        fn from(value: i32) -> Self {
            match value {
                0 => RelocInfo::Mode::NO_INFO,
                1 => RelocInfo::Mode::CODE_TARGET,
                2 => RelocInfo::Mode::RELATIVE_CODE_TARGET,
                3 => RelocInfo::Mode::COMPRESSED_EMBEDDED_OBJECT,
                4 => RelocInfo::Mode::FULL_EMBEDDED_OBJECT,
                5 => RelocInfo::Mode::WASM_CALL,
                6 => RelocInfo::Mode::WASM_STUB_CALL,
                7 => RelocInfo::Mode::WASM_CODE_POINTER_TABLE_ENTRY,
                8 => RelocInfo::Mode::WASM_CANONICAL_SIG_ID,
                9 => RelocInfo::Mode::EXTERNAL_REFERENCE,
                10 => RelocInfo::Mode::INTERNAL_REFERENCE,
                11 => RelocInfo::Mode::INTERNAL_REFERENCE_ENCODED,
                12 => RelocInfo::Mode::JS_DISPATCH_HANDLE,
                13 => RelocInfo::Mode::OFF_HEAP_TARGET,
                14 => RelocInfo::Mode::NEAR_BUILTIN_ENTRY,
                15 => RelocInfo::Mode::CONST_POOL,
                16 => RelocInfo::Mode::VENEER_POOL,
                17 => RelocInfo::Mode::DEOPT_SCRIPT_OFFSET,
                18 => RelocInfo::Mode::DEOPT_INLINING_ID,
                19 => RelocInfo::Mode::DEOPT_REASON,
                20 => RelocInfo::Mode::DEOPT_ID,
                21 => RelocInfo::Mode::DEOPT_NODE_ID,
                22 => RelocInfo::Mode::PC_JUMP,
                23 => RelocInfo::Mode::NUMBER_OF_MODES,
                _ => panic!("Invalid RelocInfo::Mode value: {}", value),
            }
        }
    }

    impl RelocInfo::Mode {
        fn from_i32(value: i32) -> Option<Self> {
            match value {
                 0 => Some(RelocInfo::Mode::NO_INFO),
                 1 => Some(RelocInfo::Mode::CODE_TARGET),
                 2 => Some(RelocInfo::Mode::RELATIVE_CODE_TARGET),
                 3 => Some(RelocInfo::Mode::COMPRESSED_EMBEDDED_OBJECT),
                 4 => Some(RelocInfo::Mode::FULL_EMBEDDED_OBJECT),
                 5 => Some(RelocInfo::Mode::WASM_CALL),
                 6 => Some(RelocInfo::Mode::WASM_STUB_CALL),
                 7 => Some(RelocInfo::Mode::WASM_CODE_POINTER_TABLE_ENTRY),
                 8 => Some(RelocInfo::Mode::WASM_CANONICAL_SIG_ID),
                 9 => Some(RelocInfo::Mode::EXTERNAL_REFERENCE),
                10 => Some(RelocInfo::Mode::INTERNAL_REFERENCE),
                11 => Some(RelocInfo::Mode::INTERNAL_REFERENCE_ENCODED),
                12 => Some(RelocInfo::Mode::JS_DISPATCH_HANDLE),
                13 => Some(RelocInfo::Mode::OFF_HEAP_TARGET),
                14 => Some(RelocInfo::Mode::NEAR_BUILTIN_ENTRY),
                15 => Some(RelocInfo::Mode::CONST_POOL),
                16 => Some(RelocInfo::Mode::VENEER_POOL),
                17 => Some(RelocInfo::Mode::DEOPT_SCRIPT_OFFSET),
                18 => Some(RelocInfo::Mode::DEOPT_INLINING_ID),
                19 => Some(RelocInfo::Mode::DEOPT_REASON),
                20 => Some(RelocInfo::Mode::DEOPT_ID),
                21 => Some(RelocInfo::Mode::DEOPT_NODE_ID),
                22 => Some(RelocInfo::Mode::PC_JUMP),
                23 => Some(RelocInfo::Mode::NUMBER_OF_MODES),
                _ => None,
            }
        }
    }

    #[derive(Debug)]
    pub struct WritableRelocInfo {
        base: RelocInfo,
        jit_allocation_: WritableJitAllocation,
    }

    impl WritableRelocInfo {
        pub fn new(jit_allocation: WritableJitAllocation, pc: Address, rmode: Mode) -> Self {
            WritableRelocInfo {
                base: RelocInfo::new_simple(pc, rmode),
                jit_allocation_: jit_allocation,
            }
        }
        pub fn new_full(jit_allocation: WritableJitAllocation, pc: Address, rmode: Mode, data: intptr_t, constant_pool: Address) -> Self {
            WritableRelocInfo {
                base: RelocInfo::new(pc, rmode, data, constant_pool),
                jit_allocation_: jit_allocation,
            }
        }

        // Apply a relocation by delta bytes. When the code object is moved, PC
        // relative addresses have to be updated as well as absolute addresses
        // inside the code (internal references).
        // Do not forget to flush the icache afterwards!
        // #[inline]
        pub fn apply(&mut self, _delta: intptr_t) {
            todo!()
        }

        pub fn set_wasm_call_address(&mut self, _address: Address) {
            todo!()
        }
        pub fn set_wasm_stub_call_address(&mut self, _address: Address) {
            todo!()
        }
        pub fn set_wasm_canonical_sig_id(&mut self, _id: uint32_t) {
            todo!()
        }
        // #[inline]
        pub fn set_wasm_code_pointer_table_entry(&mut self, _wasm_code_pointer: WasmCodePointer, _icache_flush_mode: ICacheFlushMode) {
            todo!()
        }

        pub fn set_target_address(&mut self, _host: Tagged<InstructionStream>, _target: Address, _write_barrier_mode: WriteBarrierMode, _icache_flush_mode: ICacheFlushMode) {
            todo!()
        }
        // Use this overload only when an InstructionStream host is not available.
        pub fn set_target_address_no_host(&mut self, _target: Address, _icache_flush_mode: ICacheFlushMode) {
            todo!()
        }

        // #[inline]
        pub fn set_target_object(&mut self, _host: Tagged<InstructionStream>, _target: Tagged<HeapObject>, _write_barrier_mode: WriteBarrierMode, _icache_flush_mode: ICacheFlushMode) {
            todo!()
        }
        // Use this overload only when an InstructionStream host is not available.
        // #[inline]
        pub fn set_target_object_no_host(&mut self, _target: Tagged<HeapObject>, _icache_flush_mode: ICacheFlushMode) {
            todo!()
        }

        // #[inline]
        pub fn set_target_external_reference(&mut self, _address: Address, _icache_flush_mode: ICacheFlushMode) {
            todo!()
        }

        pub fn jit_allocation(&mut self) -> &mut WritableJitAllocation {
            &mut self.jit_allocation_
        }
    }

    /// `RelocInfoWriter` serializes a stream of relocation info. It writes towards
    /// lower addresses.
    pub struct RelocInfoWriter {
        pos_: *mut uint8_t,
        last_pc_: *mut uint8_t,
    }

    impl RelocInfoWriter {
        pub fn new() -> Self {
            RelocInfoWriter {
                pos_: std::ptr::null_mut(),
                last_pc_: std::ptr::null_mut(),
            }
        }

        pub fn