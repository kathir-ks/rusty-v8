// Converted from V8 C++ source files:
// Header: regexp-bytecode-generator-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// From /home/kathirks_gc/v8_go/archive/codebase/src/regexp/regexp-bytecode-generator.h
pub struct RegExpBytecodeGenerator {
    pc_: i32,
    buffer_: Vec<u8>,
}

impl RegExpBytecodeGenerator {
    pub fn new() -> Self {
        RegExpBytecodeGenerator {
            pc_: 0,
            buffer_: Vec::new(),
        }
    }
    fn Emit32(&mut self, word: u32) {
        DCHECK!(self.pc_ <= self.buffer_.len() as i32);
        if self.pc_ + 3 >= self.buffer_.len() as i32 {
            self.ExpandBuffer();
        }
        let pc = self.pc_ as usize;
        let mut bytes: [u8; 4] = word.to_le_bytes();
        self.buffer_.splice(pc..pc, bytes.iter().cloned());
        self.pc_ += 4;
    }
    fn ExpandBuffer(&mut self) {
        self.buffer_.resize(self.buffer_.len() * 2 + 1, 0);
    }
    fn Emit(&mut self, byte: u32, twenty_four_bits: u32) {
        DCHECK!(is_uint24(twenty_four_bits));
        self.Emit32((twenty_four_bits << BYTECODE_SHIFT) | byte);
    }

    fn Emit_i32(&mut self, byte: u32, twenty_four_bits: i32) {
        DCHECK!(is_int24(twenty_four_bits));
        self.Emit32(((twenty_four_bits as u32) << BYTECODE_SHIFT) | byte);
    }

    fn Emit16(&mut self, word: u32) {
        DCHECK!(self.pc_ <= self.buffer_.len() as i32);
        if self.pc_ + 1 >= self.buffer_.len() as i32 {
            self.ExpandBuffer();
        }
        let pc = self.pc_ as usize;
        let mut bytes: [u8; 2] = (word as u16).to_le_bytes();
        self.buffer_.splice(pc..pc, bytes.iter().cloned());
        self.pc_ += 2;
    }

    fn Emit8(&mut self, word: u32) {
        DCHECK!(self.pc_ <= self.buffer_.len() as i32);
        if self.pc_ == self.buffer_.len() as i32 {
            self.ExpandBuffer();
        }
        let pc = self.pc_ as usize;
        self.buffer_.splice(pc..pc, [(word & 0xFF) as u8].iter().cloned());
        self.pc_ += 1;
    }
}

const BYTECODE_SHIFT: u32 = 8;
fn is_uint24(value: u32) -> bool {
    value < (1 << 24)
}

fn is_int24(value: i32) -> bool {
    value >= -(1 << 23) && value < (1 << 23)
}
macro_rules! DCHECK {
    ($condition:expr) => {
        if !$condition {
            panic!("DCHECK failed: {}", stringify!($condition));
        }
    };
}
