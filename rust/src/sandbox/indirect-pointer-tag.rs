// Copyright 2023 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod indirect_pointer_tag {
    use std::convert::TryFrom;

    // TODO: Replace `InstanceType` with the actual Rust type.
    // This is a placeholder, replace with the equivalent Rust enum from `src/objects/instance-type.h`
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum InstanceType {
        CODE_TYPE,
        BYTECODE_ARRAY_TYPE,
        INTERPRETER_DATA_TYPE,
        UNCOMPILED_DATA_WITHOUT_PREPARSE_DATA_TYPE,
        UNCOMPILED_DATA_WITH_PREPARSE_DATA_TYPE,
        UNCOMPILED_DATA_WITHOUT_PREPARSE_DATA_WITH_JOB_TYPE,
        UNCOMPILED_DATA_WITH_PREPARSE_DATA_AND_JOB_TYPE,
        ATOM_REG_EXP_DATA_TYPE,
        IR_REG_EXP_DATA_TYPE,
        WASM_DISPATCH_TABLE_TYPE,
        WASM_TRUSTED_INSTANCE_DATA_TYPE,
        WASM_INTERNAL_FUNCTION_TYPE,
        WASM_FUNCTION_DATA_TYPE,
        WASM_EXPORTED_FUNCTION_DATA_TYPE,
        WASM_JS_FUNCTION_DATA_TYPE,
        WASM_CAPI_FUNCTION_DATA_TYPE,
    }

    pub const K_INDIRECT_POINTER_TAG_SHIFT: i32 = 48;
    pub const K_INDIRECT_POINTER_TAG_MASK: u64 = 0x7fff_0000_0000_0000;
    pub const K_TRUSTED_POINTER_TABLE_MARK_BIT: u64 = 0x8000_0000_0000_0000;
    pub const K_TRUSTED_POINTER_TABLE_FREE_ENTRY_BIT: u64 = 0x0080_0000_0000_0000;
    pub const K_INDIRECT_POINTER_TAG_MASK_WITHOUT_FREE_ENTRY_BIT: u64 = 0x7f7f_0000_0000_0000;

    pub const K_ALL_TAGS_FOR_AND_BASED_TYPE_CHECKING: [u64; 64] = [
        0b00001111, 0b00010111, 0b00011011, 0b00011101, 0b00011110, 0b00100111,
        0b00101011, 0b00101101, 0b00101110, 0b00110011, 0b00110101, 0b00110110,
        0b00111001, 0b00111010, 0b00111100, 0b01000111, 0b01001011, 0b01001101,
        0b01001110, 0b01010011, 0b01010101, 0b01010110, 0b01011001, 0b01011010,
        0b01011100, 0b01100011, 0b01100101, 0b01100110, 0b01101001, 0b01101010,
        0b01101100, 0b01110001, 0b01110010, 0b01110100, 0b01111000, 0b10000111,
        0b10001011, 0b10001101, 0b10001110, 0b10010011, 0b10010101, 0b10010110,
        0b10011001, 0b10011010, 0b10011100, 0b10100011, 0b10100101, 0b10100110,
        0b10101001, 0b10101010, 0b10101100, 0b10110001, 0b10110010, 0b10110100,
        0b10111000, 0b11000011, 0b11000101, 0b11000110, 0b11001001, 0b11001010,
        0b11001100, 0b11010001, 0b11010010, 0b11010100, 0b11011000, 0b11100001,
        0b11100010, 0b11100100, 0b11101000, 0b11110000,
    ];

    macro_rules! make_tag {
        ($i:expr) => {
            K_ALL_TAGS_FOR_AND_BASED_TYPE_CHECKING[$i] << K_INDIRECT_POINTER_TAG_SHIFT
        };
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u64)]
    pub enum IndirectPointerTag {
        // The null tag. Usually used to express the lack of a valid tag, for example
        // in non-sandbox builds.
        IndirectPointerNullTag = 0,

        // This tag can be used when an indirect pointer field can legitimately refer
        // to objects of different types.
        // NOTE: this tag effectively disables the built-in type-checking mechanism.
        // As such, in virtually all cases the caller needs to perform runtime-type
        // checks (i.e. IsXyzObject(obj))` afterwards which need to be able to
        // correctly handle unexpected types. The last point is worth stressing
        // further. As an example, the following code is NOT correct:
        //
        //     auto obj = LoadTrustedPointerField<kUnknownIndirectPointerTag>(...);
        //     if (IsFoo(obj)) {
        //         Cast<Foo>(obj)->foo();
        //     } else if (IsBar(obj)) {
        //         Cast<Bar>(obj)->bar();
        //     } else {
        //         // Potential type confusion here!
        //         Cast<Baz>(obj)->baz();
        //     }
        //
        // This is because an attacker can swap trusted pointers and thereby cause an
        // object of a different/unexpected type to be returned. Instead, in this
        // case a CHECK can for example be used to make the code correct:
        //
        //     // ...
        //     } else {
        //         // Must be a Baz object
        //         CHECK(IsBaz(obj));
        //         Cast<Baz>(obj)->baz();
        //    }
        kUnknownIndirectPointerTag = K_INDIRECT_POINTER_TAG_MASK_WITHOUT_FREE_ENTRY_BIT,

        // Tag used internally by the trusted pointer table to mark free entries.
        // See also the comment above kTrustedPointerTableFreeEntryBit for why this
        // uses a dedicated bit.
        kFreeTrustedPointerTableEntryTag = K_TRUSTED_POINTER_TABLE_FREE_ENTRY_BIT,

        // "Regular" tags. One per supported instance type.
        kFirstSharedTrustedTag = make_tag!(1),
        kLastSharedTrustedTag = make_tag!(1),
        kFirstPerIsolateTrustedTag = make_tag!(6),
        kCodeIndirectPointerTag = make_tag!(6),
        kBytecodeArrayIndirectPointerTag = make_tag!(7),
        kInterpreterDataIndirectPointerTag = make_tag!(8),
        kUncompiledDataIndirectPointerTag = make_tag!(9),
        kRegExpDataIndirectPointerTag = make_tag!(10),
        #[cfg(feature = "wasm")]
        kWasmTrustedInstanceDataIndirectPointerTag = make_tag!(11),
        #[cfg(feature = "wasm")]
        kWasmInternalFunctionIndirectPointerTag = make_tag!(12),
        #[cfg(feature = "wasm")]
        kWasmFunctionDataIndirectPointerTag = make_tag!(13),
        #[cfg(feature = "wasm")]
        kWasmDispatchTableIndirectPointerTag = make_tag!(14),
        kLastPerIsolateTrustedTag = make_tag!(14),
        kUnpublishedIndirectPointerTag = make_tag!(34),
    }

    impl IndirectPointerTag {
        pub fn is_shared_trusted_pointer_type(self) -> bool {
            IndirectPointerTag::kFirstSharedTrustedTag <= self && self <= IndirectPointerTag::kLastSharedTrustedTag
        }

        pub fn is_per_isolate_trusted_pointer_type(self) -> bool {
            IndirectPointerTag::kFirstPerIsolateTrustedTag <= self && self <= IndirectPointerTag::kLastPerIsolateTrustedTag
        }

        pub fn is_valid_indirect_pointer_tag(self) -> bool {
            self.is_per_isolate_trusted_pointer_type() || self.is_shared_trusted_pointer_type()
        }

        pub const fn is_trusted_space_migration_in_progress_for_objects_with_tag(self) -> bool {
            false
        }

        pub fn from_instance_type(instance_type: InstanceType) -> Self {
            match instance_type {
                InstanceType::CODE_TYPE => IndirectPointerTag::kCodeIndirectPointerTag,
                InstanceType::BYTECODE_ARRAY_TYPE => IndirectPointerTag::kBytecodeArrayIndirectPointerTag,
                InstanceType::INTERPRETER_DATA_TYPE => IndirectPointerTag::kInterpreterDataIndirectPointerTag,
                InstanceType::UNCOMPILED_DATA_WITHOUT_PREPARSE_DATA_TYPE |
                InstanceType::UNCOMPILED_DATA_WITH_PREPARSE_DATA_TYPE |
                InstanceType::UNCOMPILED_DATA_WITHOUT_PREPARSE_DATA_WITH_JOB_TYPE |
                InstanceType::UNCOMPILED_DATA_WITH_PREPARSE_DATA_AND_JOB_TYPE => {
                    IndirectPointerTag::kUncompiledDataIndirectPointerTag
                }
                InstanceType::ATOM_REG_EXP_DATA_TYPE |
                InstanceType::IR_REG_EXP_DATA_TYPE => {
                    IndirectPointerTag::kRegExpDataIndirectPointerTag
                }
                #[cfg(feature = "wasm")]
                InstanceType::WASM_DISPATCH_TABLE_TYPE => IndirectPointerTag::kWasmDispatchTableIndirectPointerTag,
                #[cfg(feature = "wasm")]
                InstanceType::WASM_TRUSTED_INSTANCE_DATA_TYPE => IndirectPointerTag::kWasmTrustedInstanceDataIndirectPointerTag,
                #[cfg(feature = "wasm")]
                InstanceType::WASM_INTERNAL_FUNCTION_TYPE => IndirectPointerTag::kWasmInternalFunctionIndirectPointerTag,
                #[cfg(feature = "wasm")]
                InstanceType::WASM_FUNCTION_DATA_TYPE |
                InstanceType::WASM_EXPORTED_FUNCTION_DATA_TYPE |
                InstanceType::WASM_JS_FUNCTION_DATA_TYPE |
                InstanceType::WASM_CAPI_FUNCTION_DATA_TYPE => {
                    IndirectPointerTag::kWasmFunctionDataIndirectPointerTag
                }
                _ => panic!("UNREACHABLE"),
            }
        }

        // TODO: Properly handle the Result and error type
        pub fn instance_type_from_indirect_pointer_tag(tag: IndirectPointerTag) -> InstanceType {
            assert!(tag.is_valid_indirect_pointer_tag());
            match tag {
                IndirectPointerTag::kCodeIndirectPointerTag => InstanceType::CODE_TYPE,
                IndirectPointerTag::kBytecodeArrayIndirectPointerTag => InstanceType::BYTECODE_ARRAY_TYPE,
                IndirectPointerTag::kInterpreterDataIndirectPointerTag => InstanceType::INTERPRETER_DATA_TYPE,
                IndirectPointerTag::kUncompiledDataIndirectPointerTag => InstanceType::UNCOMPILED_DATA_WITHOUT_PREPARSE_DATA_TYPE,
                IndirectPointerTag::kRegExpDataIndirectPointerTag => InstanceType::ATOM_REG_EXP_DATA_TYPE,
                #[cfg(feature = "wasm")]
                IndirectPointerTag::kWasmDispatchTableIndirectPointerTag => InstanceType::WASM_DISPATCH_TABLE_TYPE,
                #[cfg(feature = "wasm")]
                IndirectPointerTag::kWasmTrustedInstanceDataIndirectPointerTag => InstanceType::WASM_TRUSTED_INSTANCE_DATA_TYPE,
                #[cfg(feature = "wasm")]
                IndirectPointerTag::kWasmInternalFunctionIndirectPointerTag => InstanceType::WASM_INTERNAL_FUNCTION_TYPE,
                #[cfg(feature = "wasm")]
                IndirectPointerTag::kWasmFunctionDataIndirectPointerTag => InstanceType::WASM_FUNCTION_DATA_TYPE,
                _ => panic!("UNREACHABLE"),
            }
        }
    }

    impl TryFrom<u64> for IndirectPointerTag {
        type Error = ();

        fn try_from(value: u64) -> Result<Self, Self::Error> {
            match value {
                x if x == IndirectPointerTag::IndirectPointerNullTag as u64 => Ok(IndirectPointerTag::IndirectPointerNullTag),
                x if x == IndirectPointerTag::kUnknownIndirectPointerTag as u64 => Ok(IndirectPointerTag::kUnknownIndirectPointerTag),
                x if x == IndirectPointerTag::kFreeTrustedPointerTableEntryTag as u64 => Ok(IndirectPointerTag::kFreeTrustedPointerTableEntryTag),
                x if x == IndirectPointerTag::kFirstSharedTrustedTag as u64 => Ok(IndirectPointerTag::kFirstSharedTrustedTag),
                x if x == IndirectPointerTag::kLastSharedTrustedTag as u64 => Ok(IndirectPointerTag::kLastSharedTrustedTag),
                x if x == IndirectPointerTag::kFirstPerIsolateTrustedTag as u64 => Ok(IndirectPointerTag::kFirstPerIsolateTrustedTag),
                x if x == IndirectPointerTag::kCodeIndirectPointerTag as u64 => Ok(IndirectPointerTag::kCodeIndirectPointerTag),
                x if x == IndirectPointerTag::kBytecodeArrayIndirectPointerTag as u64 => Ok(IndirectPointerTag::kBytecodeArrayIndirectPointerTag),
                x if x == IndirectPointerTag::kInterpreterDataIndirectPointerTag as u64 => Ok(IndirectPointerTag::kInterpreterDataIndirectPointerTag),
                x if x == IndirectPointerTag::kUncompiledDataIndirectPointerTag as u64 => Ok(IndirectPointerTag::kUncompiledDataIndirectPointerTag),
                x if x == IndirectPointerTag::kRegExpDataIndirectPointerTag as u64 => Ok(IndirectPointerTag::kRegExpDataIndirectPointerTag),
                #[cfg(feature = "wasm")]
                x if x == IndirectPointerTag::kWasmTrustedInstanceDataIndirectPointerTag as u64 => Ok(IndirectPointerTag::kWasmTrustedInstanceDataIndirectPointerTag),
                #[cfg(feature = "wasm")]
                x if x == IndirectPointerTag::kWasmInternalFunctionIndirectPointerTag as u64 => Ok(IndirectPointerTag::kWasmInternalFunctionIndirectPointerTag),
                #[cfg(feature = "wasm")]
                x if x == IndirectPointerTag::kWasmFunctionDataIndirectPointerTag as u64 => Ok(IndirectPointerTag::kWasmFunctionDataIndirectPointerTag),
                #[cfg(feature = "wasm")]
                x if x == IndirectPointerTag::kWasmDispatchTableIndirectPointerTag as u64 => Ok(IndirectPointerTag::kWasmDispatchTableIndirectPointerTag),
                x if x == IndirectPointerTag::kLastPerIsolateTrustedTag as u64 => Ok(IndirectPointerTag::kLastPerIsolateTrustedTag),
                x if x == IndirectPointerTag::kUnpublishedIndirectPointerTag as u64 => Ok(IndirectPointerTag::kUnpublishedIndirectPointerTag),
                _ => Err(()),
            }
        }
    }

    const _: () = assert!(IndirectPointerTag::kIndirectPointerNullTag as u64 & K_INDIRECT_POINTER_TAG_MASK == IndirectPointerTag::IndirectPointerNullTag as u64);
    const _: () = assert!(IndirectPointerTag::kIndirectPointerNullTag as u64 & K_INDIRECT_POINTER_TAG_MASK_WITHOUT_FREE_ENTRY_BIT == IndirectPointerTag::IndirectPointerNullTag as u64);

    const _: () = assert!(IndirectPointerTag::kUnknownIndirectPointerTag as u64 & K_INDIRECT_POINTER_TAG_MASK == IndirectPointerTag::kUnknownIndirectPointerTag as u64);
    const _: () = assert!(IndirectPointerTag::kUnknownIndirectPointerTag as u64 & K_INDIRECT_POINTER_TAG_MASK_WITHOUT_FREE_ENTRY_BIT == IndirectPointerTag::kUnknownIndirectPointerTag as u64);

    const _: () = assert!(IndirectPointerTag::kFreeTrustedPointerTableEntryTag as u64 & K_INDIRECT_POINTER_TAG_MASK == IndirectPointerTag::kFreeTrustedPointerTableEntryTag as u64);
    const _: () = assert!(IndirectPointerTag::kFreeTrustedPointerTableEntryTag as u64 & K_INDIRECT_POINTER_TAG_MASK_WITHOUT_FREE_ENTRY_BIT == 0);

    const _: () = assert!(!IndirectPointerTag::IndirectPointerNullTag.is_valid_indirect_pointer_tag());

    const _: () = assert!(IndirectPointerTag::kFirstSharedTrustedTag.is_shared_trusted_pointer_type());
    const _: () = assert!(IndirectPointerTag::kLastSharedTrustedTag.is_shared_trusted_pointer_type());

    const _: () = assert!(!IndirectPointerTag::kFirstPerIsolateTrustedTag.is_shared_trusted_pointer_type());
    const _: () = assert!(!IndirectPointerTag::kCodeIndirectPointerTag.is_shared_trusted_pointer_type());
    const _: () = assert!(!IndirectPointerTag::kBytecodeArrayIndirectPointerTag.is_shared_trusted_pointer_type());
    const _: () = assert!(!IndirectPointerTag::kInterpreterDataIndirectPointerTag.is_shared_trusted_pointer_type());
    const _: () = assert!(!IndirectPointerTag::kUncompiledDataIndirectPointerTag.is_shared_trusted_pointer_type());
    const _: () = assert!(!IndirectPointerTag::kRegExpDataIndirectPointerTag.is_shared_trusted_pointer_type());
    #[cfg(feature = "wasm")]
    const _: () = assert!(!IndirectPointerTag::kWasmTrustedInstanceDataIndirectPointerTag.is_shared_trusted_pointer_type());
    #[cfg(feature = "wasm")]
    const _: () = assert!(!IndirectPointerTag::kWasmInternalFunctionIndirectPointerTag.is_shared_trusted_pointer_type());
    #[cfg(feature = "wasm")]
    const _: () = assert!(!IndirectPointerTag::kWasmFunctionDataIndirectPointerTag.is_shared_trusted_pointer_type());
    #[cfg(feature = "wasm")]
    const _: () = assert!(!IndirectPointerTag::kWasmDispatchTableIndirectPointerTag.is_shared_trusted_pointer_type());
    const _: () = assert!(!IndirectPointerTag::kLastPerIsolateTrustedTag.is_shared_trusted_pointer_type());
}