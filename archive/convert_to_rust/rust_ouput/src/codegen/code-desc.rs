// Converted from V8 C++ source files:
// Header: code-desc.h
// Implementation: code-desc.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct CodeDesc {
    pub buffer: *mut u8,
    pub buffer_size: i32,
    pub instr_size: i32,
    pub safepoint_table_offset: i32,
    pub safepoint_table_size: i32,
    pub handler_table_offset: i32,
    pub handler_table_size: i32,
    pub constant_pool_offset: i32,
    pub constant_pool_size: i32,
    pub code_comments_offset: i32,
    pub code_comments_size: i32,
    pub builtin_jump_table_info_offset: i32,
    pub builtin_jump_table_info_size: i32,
    pub reloc_offset: i32,
    pub reloc_size: i32,
    pub unwinding_info: *mut u8,
    pub unwinding_info_size: i32,
    pub origin: *mut Assembler,
}

impl CodeDesc {
    pub fn initialize(
        desc: &mut CodeDesc,
        assembler: *mut Assembler,
        safepoint_table_offset: i32,
        handler_table_offset: i32,
        constant_pool_offset: i32,
        code_comments_offset: i32,
        builtin_jump_table_info_offset: i32,
        reloc_info_offset: i32,
    ) {
        unsafe {
            desc.buffer = (*assembler).buffer_start();
            desc.buffer_size = (*assembler).buffer_size();
            desc.instr_size = (*assembler).instruction_size();

            desc.builtin_jump_table_info_offset = builtin_jump_table_info_offset;
            desc.builtin_jump_table_info_size =
                desc.instr_size - builtin_jump_table_info_offset;

            desc.code_comments_offset = code_comments_offset;
            desc.code_comments_size =
                desc.builtin_jump_table_info_offset - code_comments_offset;

            desc.constant_pool_offset = constant_pool_offset;
            desc.constant_pool_size = desc.code_comments_offset - constant_pool_offset;

            desc.handler_table_offset = handler_table_offset;
            desc.handler_table_size = desc.constant_pool_offset - handler_table_offset;

            desc.safepoint_table_offset = safepoint_table_offset;
            desc.safepoint_table_size =
                desc.handler_table_offset - safepoint_table_offset;

            desc.reloc_offset = reloc_info_offset;
            desc.reloc_size = desc.buffer_size - reloc_info_offset;

            desc.unwinding_info_size = 0;
            desc.unwinding_info = std::ptr::null_mut();

            desc.origin = assembler;

            CodeDesc::verify(desc);
        }
    }

    #[cfg(debug_assertions)]
    pub fn verify(desc: &CodeDesc) {
        assert!(desc.instr_size > 0);
        assert!(!desc.buffer.is_null());

        assert!(desc.safepoint_table_size >= 0);
        assert_eq!(
            desc.safepoint_table_size + desc.safepoint_table_offset,
            desc.handler_table_offset
        );
        assert!(desc.handler_table_size >= 0);
        assert_eq!(
            desc.handler_table_size + desc.handler_table_offset,
            desc.constant_pool_offset
        );
        assert!(desc.constant_pool_size >= 0);
        assert_eq!(
            desc.constant_pool_size + desc.constant_pool_offset,
            desc.code_comments_offset
        );
        assert!(desc.code_comments_size >= 0);
        assert_eq!(
            desc.code_comments_size + desc.code_comments_offset,
            desc.builtin_jump_table_info_offset
        );
        assert!(desc.builtin_jump_table_info_size >= 0);
        assert_eq!(
            desc.builtin_jump_table_info_size + desc.builtin_jump_table_info_offset,
            desc.instr_size
        );

        assert!(desc.reloc_offset >= 0);
        assert!(desc.reloc_size >= 0);
        assert!(desc.unwinding_info_size >= 0);
    }

    #[cfg(not(debug_assertions))]
    pub fn verify(desc: &CodeDesc) {}

    pub fn body_size(&self) -> i32 {
        self.instr_size + self.unwinding_info_size
    }

    pub fn instruction_size(&self) -> i32 {
        self.safepoint_table_offset
    }

    pub fn metadata_size(&self) -> i32 {
        self.body_size() - self.instruction_size()
    }

    pub fn safepoint_table_offset_relative(&self) -> i32 {
        self.safepoint_table_offset - self.instruction_size()
    }

    pub fn handler_table_offset_relative(&self) -> i32 {
        self.handler_table_offset - self.instruction_size()
    }

    pub fn constant_pool_offset_relative(&self) -> i32 {
        self.constant_pool_offset - self.instruction_size()
    }

    pub fn code_comments_offset_relative(&self) -> i32 {
        self.code_comments_offset - self.instruction_size()
    }

    pub fn builtin_jump_table_info_offset_relative(&self) -> i32 {
        self.builtin_jump_table_info_offset - self.instruction_size()
    }

    pub fn unwinding_info_offset_relative(&self) -> i32 {
        self.builtin_jump_table_info_offset_relative() + self.builtin_jump_table_info_size
    }
}

pub struct Assembler {
    buffer: Vec<u8>,
    instruction_offset: usize,
    buffer_size_val: i32,
}

impl Assembler {
    pub fn new(size: usize) -> Self {
        Assembler {
            buffer: vec![0u8; size],
            instruction_offset: 0,
            buffer_size_val: size as i32,
        }
    }
    pub fn buffer_start(&mut self) -> *mut u8 {
        self.buffer.as_mut_ptr()
    }

    pub fn buffer_size(&self) -> i32 {
        self.buffer_size_val
    }

    pub fn instruction_size(&self) -> i32 {
        self.instruction_offset as i32
    }

    pub fn emit(&mut self, byte: u8) {
        if self.instruction_offset < self.buffer.len() {
            self.buffer[self.instruction_offset] = byte;
            self.instruction_offset += 1;
        } else {
            eprintln!("Buffer overflow in Assembler::emit");
        }
    }
}
