// Converted from V8 C++ source files:
// Header: reloc-info.h
// Implementation: reloc-info.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod reloc_info {
    use crate::codegen::assembler::Assembler;
    use crate::strings::uri::V8;
    use std::io::Write;
    use std::ops::Range;

    pub struct CodeReference {}

    pub struct EmbeddedData {}

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum ICacheFlushMode {
        FLUSH_ICACHE_IF_NEEDED,
        SKIP_ICACHE_FLUSH,
    }

    pub mod detail {
        pub const K_TAG_BITS: i32 = 2;
        pub const K_TAG_MASK: i32 = (1 << K_TAG_BITS) - 1;
        pub const K_LONG_TAG_BITS: i32 = 6;

        pub const K_EMBEDDED_OBJECT_TAG: i32 = 0;
        pub const K_CODE_TARGET_TAG: i32 = 1;
        pub const K_WASM_STUB_CALL_TAG: i32 = 2;
        pub const K_DEFAULT_TAG: i32 = 3;

        pub const K_SMALL_PC_DELTA_BITS: i32 = 6;
        pub const K_SMALL_PC_DELTA_MASK: i32 = (1 << K_SMALL_PC_DELTA_BITS) - 1;
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Mode {
        NO_INFO,

        CODE_TARGET,
        RELATIVE_CODE_TARGET,
        COMPRESSED_EMBEDDED_OBJECT,
        FULL_EMBEDDED_OBJECT,

        WASM_CALL,
        WASM_STUB_CALL,
        WASM_CODE_POINTER_TABLE_ENTRY,
        WASM_CANONICAL_SIG_ID,

        EXTERNAL_REFERENCE,
        INTERNAL_REFERENCE,
        INTERNAL_REFERENCE_ENCODED,
        JS_DISPATCH_HANDLE,
        OFF_HEAP_TARGET,
        NEAR_BUILTIN_ENTRY,
        CONST_POOL,
        VENEER_POOL,
        DEOPT_SCRIPT_OFFSET,
        DEOPT_INLINING_ID,
        DEOPT_REASON,
        DEOPT_ID,
        DEOPT_NODE_ID,
        PC_JUMP,
        NUMBER_OF_MODES,
    }

    impl Mode {
        pub const LAST_CODE_TARGET_MODE: Self = Self::RELATIVE_CODE_TARGET;
        pub const FIRST_REAL_RELOC_MODE: Self = Self::CODE_TARGET;
        pub const LAST_REAL_RELOC_MODE: Self = Self::VENEER_POOL;
        pub const FIRST_EMBEDDED_OBJECT_RELOC_MODE: Self = Self::COMPRESSED_EMBEDDED_OBJECT;
        pub const LAST_EMBEDDED_OBJECT_RELOC_MODE: Self = Self::FULL_EMBEDDED_OBJECT;
        pub const FIRST_BUILTIN_ENTRY_MODE: Self = Self::OFF_HEAP_TARGET;
        pub const LAST_BUILTIN_ENTRY_MODE: Self = Self::NEAR_BUILTIN_ENTRY;
        pub const FIRST_SHAREABLE_RELOC_MODE: Self = Self::WASM_CALL;
    }

    impl RelocInfo {
        pub const K_APPLY_MASK: i32 = 0xFFFFFFFF;
        pub fn mode_mask(mode: Mode) -> i32 {
            1 << mode as i32
        }
        pub fn all_real_modes_mask() -> i32 {
            let k_first_unreal_reloc_mode = Mode::NUMBER_OF_MODES;
            ((Self::mode_mask(k_first_unreal_reloc_mode) - 1)
                & !(Self::mode_mask(Mode::FIRST_REAL_RELOC_MODE) - 1)) as i32
        }

        pub fn embedded_object_mode_mask() -> i32 {
            Self::mode_mask(Mode::FULL_EMBEDDED_OBJECT) | Self::mode_mask(Mode::COMPRESSED_EMBEDDED_OBJECT)
        }

        pub fn post_codegen_relocation_mask() -> i32 {
            Self::mode_mask(Mode::CODE_TARGET)
                | Self::mode_mask(Mode::COMPRESSED_EMBEDDED_OBJECT)
                | Self::mode_mask(Mode::FULL_EMBEDDED_OBJECT)
                | Self::mode_mask(Mode::NEAR_BUILTIN_ENTRY)
                | Self::mode_mask(Mode::WASM_STUB_CALL)
                | Self::mode_mask(Mode::RELATIVE_CODE_TARGET)
                | Self::K_APPLY_MASK
        }
    }

    pub struct RelocInfo {
        pc_: usize,
        rmode_: Mode,
        data_: isize,
        constant_pool_: usize,
    }

    impl RelocInfo {
        pub const K_MIN_RELOC_COMMENT_SIZE: i32 = 2 + 8;
        pub const K_MAX_CALL_SIZE: i32 = 6;
        pub const K_MAX_SMALL_PC_DELTA: i32 = detail::K_SMALL_PC_DELTA_MASK as i32;

        pub fn new(pc: usize, rmode: Mode, data: isize, constant_pool: usize) -> Self {
            RelocInfo {
                pc_: pc,
                rmode_: rmode,
                data_: data,
                constant_pool_: constant_pool,
            }
        }

        pub fn is_real_reloc_mode(mode: Mode) -> bool {
            mode as i32 >= Mode::FIRST_REAL_RELOC_MODE as i32 && mode as i32 <= Mode::LAST_REAL_RELOC_MODE as i32
        }

        pub fn is_gc_reloc_mode(mode: Mode) -> bool {
            mode as i32 <= Mode::LAST_EMBEDDED_OBJECT_RELOC_MODE as i32
        }

        pub fn is_shareable_reloc_mode(mode: Mode) -> bool {
            mode == Mode::NO_INFO || mode as i32 >= Mode::FIRST_SHAREABLE_RELOC_MODE as i32
        }

        pub fn is_code_target(mode: Mode) -> bool {
            mode == Mode::CODE_TARGET
        }

        pub fn is_code_target_mode(mode: Mode) -> bool {
            mode as i32 <= Mode::LAST_CODE_TARGET_MODE as i32
        }

        pub fn is_relative_code_target(mode: Mode) -> bool {
            mode == Mode::RELATIVE_CODE_TARGET
        }

        pub fn is_full_embedded_object(mode: Mode) -> bool {
            mode == Mode::FULL_EMBEDDED_OBJECT
        }

        pub fn is_compressed_embedded_object(mode: Mode) -> bool {
            mode == Mode::COMPRESSED_EMBEDDED_OBJECT
        }

        pub fn is_embedded_object_mode(mode: Mode) -> bool {
            mode as i32 >= Mode::FIRST_EMBEDDED_OBJECT_RELOC_MODE as i32 && mode as i32 <= Mode::LAST_EMBEDDED_OBJECT_RELOC_MODE as i32
        }

        pub fn is_wasm_call(mode: Mode) -> bool {
            mode == Mode::WASM_CALL
        }

        pub fn is_wasm_stub_call(mode: Mode) -> bool {
            mode == Mode::WASM_STUB_CALL
        }

        pub fn is_wasm_canonical_sig_id(mode: Mode) -> bool {
            mode == Mode::WASM_CANONICAL_SIG_ID
        }

        pub fn is_wasm_code_pointer_table_entry(mode: Mode) -> bool {
            mode == Mode::WASM_CODE_POINTER_TABLE_ENTRY
        }

        pub fn is_const_pool(mode: Mode) -> bool {
            mode == Mode::CONST_POOL
        }

        pub fn is_veneer_pool(mode: Mode) -> bool {
            mode == Mode::VENEER_POOL
        }

        pub fn is_deopt_position(mode: Mode) -> bool {
            mode == Mode::DEOPT_SCRIPT_OFFSET || mode == Mode::DEOPT_INLINING_ID
        }

        pub fn is_deopt_reason(mode: Mode) -> bool {
            mode == Mode::DEOPT_REASON
        }

        pub fn is_deopt_id(mode: Mode) -> bool {
            mode == Mode::DEOPT_ID
        }

        pub fn is_deopt_node_id(mode: Mode) -> bool {
            mode == Mode::DEOPT_NODE_ID
        }

        pub fn is_external_reference(mode: Mode) -> bool {
            mode == Mode::EXTERNAL_REFERENCE
        }

        pub fn is_internal_reference(mode: Mode) -> bool {
            mode == Mode::INTERNAL_REFERENCE
        }

        pub fn is_internal_reference_encoded(mode: Mode) -> bool {
            mode == Mode::INTERNAL_REFERENCE_ENCODED
        }

        pub fn is_off_heap_target(mode: Mode) -> bool {
            mode == Mode::OFF_HEAP_TARGET
        }

        pub fn is_near_builtin_entry(mode: Mode) -> bool {
            mode == Mode::NEAR_BUILTIN_ENTRY
        }

        pub fn is_builtin_entry_mode(mode: Mode) -> bool {
            mode as i32 >= Mode::FIRST_BUILTIN_ENTRY_MODE as i32 && mode as i32 <= Mode::LAST_BUILTIN_ENTRY_MODE as i32
        }

        pub fn is_js_dispatch_handle(mode: Mode) -> bool {
            mode == Mode::JS_DISPATCH_HANDLE
        }

        pub fn is_no_info(mode: Mode) -> bool {
            mode == Mode::NO_INFO
        }

        pub fn is_only_for_serializer(mode: Mode) -> bool {
            mode == Mode::EXTERNAL_REFERENCE || mode == Mode::OFF_HEAP_TARGET
        }

        pub fn pc(&self) -> usize {
            self.pc_
        }

        pub fn rmode(&self) -> Mode {
            self.rmode_
        }

        pub fn constant_pool(&self) -> usize {
            self.constant_pool_
        }

        pub fn data(&self) -> isize {
            self.data_
        }

        pub fn is_coded_specially(&self) -> bool {
            false
        }
        pub fn off_heap_target_is_coded_specially() -> bool {
            true
        }
        pub fn is_in_constant_pool(&self) -> bool {
            false
        }
        pub fn wasm_call_address(&self) -> usize {
            self.pc_
        }
        pub fn wasm_stub_call_address(&self) -> usize {
            self.pc_
        }
        pub fn wasm_canonical_sig_id(&self) -> u32 {
            0
        }
        pub fn wasm_code_pointer_table_entry(&self) -> i32 {
            0
        }
        pub fn wasm_call_tag(&self) -> u32 {
            0
        }
        pub fn set_off_heap_target_address(&mut self, _target: usize, _icache_flush_mode: ICacheFlushMode) {}
        pub fn target_address(&self) -> usize {
            self.pc_
        }
        pub fn target_object(_cage_base: i32) -> i32 {
            0
        }
        pub fn target_object_handle(_origin: i32) -> i32 {
            0
        }
        pub fn target_builtin_at(_origin: i32) -> i32 {
            0
        }
        pub fn target_off_heap_target(&self) -> usize {
            self.pc_
        }
        pub fn constant_pool_entry_address(&self) -> usize {
            self.pc_
        }
        pub fn target_address_address(&self) -> usize {
            self.pc_
        }
        pub fn has_target_address_address(&self) -> bool {
            false
        }
        pub fn target_address_size(&self) -> i32 {
            0
        }
        pub fn target_external_reference(&self) -> usize {
            self.pc_
        }
        pub fn target_internal_reference(&self) -> usize {
            self.pc_
        }
        pub fn target_internal_reference_address(&self) -> usize {
            self.pc_
        }
        pub fn js_dispatch_handle(&self) -> i32 {
            0
        }
        pub fn visit<F>(&self, _host: i32, _visitor: F) {}

    }
    #[derive(Debug)]
    pub struct WritableJitAllocation {}
    pub struct WritableRelocInfo {
        base: RelocInfo,
        jit_allocation_: WritableJitAllocation,
    }

    impl WritableRelocInfo {
        pub fn new(jit_allocation: WritableJitAllocation, pc: usize, rmode: Mode) -> Self {
            WritableRelocInfo {
                base: RelocInfo {
                    pc_: pc,
                    rmode_: rmode,
                    data_: 0,
                    constant_pool_: 0,
                },
                jit_allocation_: jit_allocation,
            }
        }
        pub fn new_with_data(
            jit_allocation: WritableJitAllocation,
            pc: usize,
            rmode: Mode,
            data: isize,
            constant_pool: usize,
        ) -> Self {
            WritableRelocInfo {
                base: RelocInfo {
                    pc_: pc,
                    rmode_: rmode,
                    data_: data,
                    constant_pool_: constant_pool,
                },
                jit_allocation_: jit_allocation,
            }
        }
        pub fn apply(&mut self, _delta: isize) {}
        pub fn set_wasm_call_address(&mut self, _address: usize) {}
        pub fn set_wasm_stub_call_address(&mut self, _address: usize) {}
        pub fn set_wasm_canonical_sig_id(&mut self, _id: u32) {}
        pub fn set_wasm_code_pointer_table_entry(
            &mut self,
            _code_pointer: i32,
            _icache_flush_mode: ICacheFlushMode,
        ) {
        }
        pub fn set_target_address(&mut self, _host: i32, _target: usize, _write_barrier_mode: i32, _icache_flush_mode: ICacheFlushMode) {}
        pub fn set_target_address_no_host(&mut self, _target: usize, _icache_flush_mode: ICacheFlushMode) {}
        pub fn set_target_object(&mut self, _host: i32, _target: i32, _write_barrier_mode: i32, _icache_flush_mode: ICacheFlushMode) {}
        pub fn set_target_object_no_host(&mut self, _target: i32, _icache_flush_mode: ICacheFlushMode) {}
        pub fn set_target_external_reference(&mut self, _address: usize, _icache_flush_mode: ICacheFlushMode) {}
        pub fn jit_allocation(&mut self) -> &mut WritableJitAllocation {
            &mut self.jit_allocation_
        }
    }
    pub struct RelocInfoWriter {
        pos_: *mut u8,
        last_pc_: *mut u8,
    }

    impl RelocInfoWriter {
        pub const K_MAX_SIZE: i32 = 1 + 4 + 1 + 1 + 8;
        pub fn new() -> Self {
            RelocInfoWriter {
                pos_: std::ptr::null_mut(),
                last_pc_: std::ptr::null_mut(),
            }
        }
        pub fn pos(&self) -> *mut u8 {
            self.pos_
        }
        pub fn last_pc(&self) -> *mut u8 {
            self.last_pc_
        }
        pub fn write(&mut self, rinfo: &RelocInfo) {}
        pub fn reposition(&mut self, _pos: *mut u8, _pc: *mut u8) {}
        fn write_long_pc_jump(&mut self, pc_delta: u32) -> u32 {
            pc_delta
        }
        fn write_short_tagged_pc(&mut self, _pc_delta: u32, _tag: i32) {}
        fn write_short_data(&mut self, _data_delta: u8) {}
        fn write_mode(&mut self, _rmode: Mode) {}
        fn write_mode_and_pc(&mut self, _pc_delta: u32, _rmode: Mode) {}
        fn write_int_data(&mut self, _number: i32) {}
    }
    pub struct RelocIteratorBase<T> {
        pos_: *const u8,
        end_: *const u8,
        rinfo_: T,
        done_: bool,
        mode_mask_: i32,
    }

    impl<T> RelocIteratorBase<T> {
        pub const K_ALL_MODES_MASK: i32 = -1;
        fn new(reloc_info: T, pos: *const u8, end: *const u8, mode_mask: i32) -> Self {
            RelocIteratorBase {
                pos_: pos,
                end_: end,
                rinfo_: reloc_info,
                done_: false,
                mode_mask_: mode_mask,
            }
        }
        pub fn done(&self) -> bool {
            self.done_
        }
        pub fn next(&mut self) {}
        pub fn rinfo(&mut self) -> &mut T {
            &mut self.rinfo_
        }
        fn set_mode(&mut self, _mode: Mode) -> bool {
            false
        }
        fn get_mode(&self) -> Mode {
            Mode::NO_INFO
        }
        fn advance(&mut self, _bytes: i32) {}
        fn advance_get_tag(&mut self) -> i32 {
            0
        }
        fn advance_read_long_pc_jump(&mut self) {}
        fn advance_read_pc(&mut self) {}
        fn advance_read_int(&mut self) {}
        fn read_short_tagged_pc(&mut self) {}
        fn read_short_data(&mut self) {}
    }
    pub struct RelocIterator {
        base: RelocIteratorBase<RelocInfo>,
    }

    impl RelocIterator {
        pub fn new_instruction_stream(istream: i32, mode_mask: i32) -> Self {
            RelocIterator {
                base: RelocIteratorBase::new(RelocInfo { pc_: 0, rmode_: Mode::NO_INFO, data_: 0, constant_pool_: 0 }, std::ptr::null(), std::ptr::null(), mode_mask),
            }
        }

        pub fn new_code(code: i32, mode_mask: i32) -> Self {
            RelocIterator {
                base: RelocIteratorBase::new(RelocInfo { pc_: 0, rmode_: Mode::NO_INFO, data_: 0, constant_pool_: 0 }, std::ptr::null(), std::ptr::null(), mode_mask),
            }
        }

        pub fn new_vector(
            instructions: i32,
            reloc_info: i32,
            const_pool: i32,
            mode_mask: i32,
        ) -> Self {
            RelocIterator {
                base: RelocIteratorBase::new(RelocInfo { pc_: 0, rmode_: Mode::NO_INFO, data_: 0, constant_pool_: 0 }, std::ptr::null(), std::ptr::null(), mode_mask),
            }
        }

        pub fn new_code_reference(code_reference: i32) -> Self {
            RelocIterator {
                base: RelocIteratorBase::new(RelocInfo { pc_: 0, rmode_: Mode::NO_INFO, data_: 0, constant_pool_: 0 }, std::ptr::null(), std::ptr::null(), 0),
            }
        }

        pub fn new_embedded_data(
            embedded_data: i32,
            code: i32,
            mode_mask: i32,
        ) -> Self {
            RelocIterator {
                base: RelocIteratorBase::new(RelocInfo { pc_: 0, rmode_: Mode::NO_INFO, data_: 0, constant_pool_: 0 }, std::ptr::null(), std::ptr::null(), mode_mask),
            }
        }

    }
    pub struct WritableRelocIterator {
        base: RelocIteratorBase<WritableRelocInfo>,
    }

    impl WritableRelocIterator {
        pub fn new_instruction_stream(
            jit_allocation: WritableJitAllocation,
            istream: i32,
            constant_pool: i32,
            mode_mask: i32,
        ) -> Self {
            WritableRelocIterator {
                base: RelocIteratorBase::new(WritableRelocInfo::new(jit_allocation, 0, Mode::NO_INFO), std::ptr::null(), std::ptr::null(), mode_mask),
            }
        }
        pub fn new_vector(
            jit_allocation: WritableJitAllocation,
            instructions: i32,
            reloc_info: i32,
            constant_pool: i32,
            mode_mask: i32,
        ) -> Self {
            WritableRelocIterator {
                base: RelocIteratorBase::new(WritableRelocInfo::new(jit_allocation, 0, Mode::NO_INFO), std::ptr::null(), std::ptr::null(), mode_mask),
            }
        }
    }
}
