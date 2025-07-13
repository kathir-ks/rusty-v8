// Converted from V8 C++ source files:
// Header: fma-instr.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod fma_instr {

  macro_rules! define_fma_instructions {
    ($V:ident) => {
      $V!(vfmadd132sd, L128, 0x66, 0x0F, 0x38, W1, 0x99);
      $V!(vfmadd213sd, L128, 0x66, 0x0F, 0x38, W1, 0xA9);
      $V!(vfmadd231sd, L128, 0x66, 0x0F, 0x38, W1, 0xB9);
      $V!(vfmsub132sd, L128, 0x66, 0x0F, 0x38, W1, 0x9B);
      $V!(vfmsub213sd, L128, 0x66, 0x0F, 0x38, W1, 0xAB);
      $V!(vfmsub231sd, L128, 0x66, 0x0F, 0x38, W1, 0xBB);
      $V!(vfnmadd132sd, L128, 0x66, 0x0F, 0x38, W1, 0x9D);
      $V!(vfnmadd213sd, L128, 0x66, 0x0F, 0x38, W1, 0xAD);
      $V!(vfnmadd231sd, L128, 0x66, 0x0F, 0x38, W1, 0xBD);
      $V!(vfnmsub132sd, L128, 0x66, 0x0F, 0x38, W1, 0x9F);
      $V!(vfnmsub213sd, L128, 0x66, 0x0F, 0x38, W1, 0xAF);
      $V!(vfnmsub231sd, L128, 0x66, 0x0F, 0x38, W1, 0xBF);

      $V!(vfmadd132ss, LIG, 0x66, 0x0F, 0x38, W0, 0x99);
      $V!(vfmadd213ss, LIG, 0x66, 0x0F, 0x38, W0, 0xA9);
      $V!(vfmadd231ss, LIG, 0x66, 0x0F, 0x38, W0, 0xB9);
      $V!(vfmsub132ss, LIG, 0x66, 0x0F, 0x38, W0, 0x9B);
      $V!(vfmsub213ss, LIG, 0x66, 0x0F, 0x38, W0, 0xAB);
      $V!(vfmsub231ss, LIG, 0x66, 0x0F, 0x38, W0, 0xBB);
      $V!(vfnmadd132ss, LIG, 0x66, 0x0F, 0x38, W0, 0x9D);
      $V!(vfnmadd213ss, LIG, 0x66, 0x0F, 0x38, W0, 0xAD);
      $V!(vfnmadd231ss, LIG, 0x66, 0x0F, 0x38, W0, 0xBD);
      $V!(vfnmsub132ss, LIG, 0x66, 0x0F, 0x38, W0, 0x9F);
      $V!(vfnmsub213ss, LIG, 0x66, 0x0F, 0x38, W0, 0xAF);
      $V!(vfnmsub231ss, LIG, 0x66, 0x0F, 0x38, W0, 0xBF);

      $V!(vfmadd132ps, L128, 0x66, 0x0F, 0x38, W0, 0x98);
      $V!(vfmadd213ps, L128, 0x66, 0x0F, 0x38, W0, 0xA8);
      $V!(vfmadd231ps, L128, 0x66, 0x0F, 0x38, W0, 0xB8);
      $V!(vfnmadd132ps, L128, 0x66, 0x0F, 0x38, W0, 0x9C);
      $V!(vfnmadd213ps, L128, 0x66, 0x0F, 0x38, W0, 0xAC);
      $V!(vfnmadd231ps, L128, 0x66, 0x0F, 0x38, W0, 0xBC);

      $V!(vfmadd132pd, L128, 0x66, 0x0F, 0x38, W1, 0x98);
      $V!(vfmadd213pd, L128, 0x66, 0x0F, 0x38, W1, 0xA8);
      $V!(vfmadd231pd, L128, 0x66, 0x0F, 0x38, W1, 0xB8);
      $V!(vfnmadd132pd, L128, 0x66, 0x0F, 0x38, W1, 0x9C);
      $V!(vfnmadd213pd, L128, 0x66, 0x0F, 0x38, W1, 0xAC);
      $V!(vfnmadd231pd, L128, 0x66, 0x0F, 0x38, W1, 0xBC);
    };
  }

  #[derive(Debug, Copy, Clone)]
  pub enum Instruction {
    Vfmadd132sd,
    Vfmadd213sd,
    Vfmadd231sd,
    Vfmsub132sd,
    Vfmsub213sd,
    Vfmsub231sd,
    Vfnmadd132sd,
    Vfnmadd213sd,
    Vfnmadd231sd,
    Vfnmsub132sd,
    Vfnmsub213sd,
    Vfnmsub231sd,
    Vfmadd132ss,
    Vfmadd213ss,
    Vfmadd231ss,
    Vfmsub132ss,
    Vfmsub213ss,
    Vfmsub231ss,
    Vfnmadd132ss,
    Vfnmadd213ss,
    Vfnmadd231ss,
    Vfnmsub132ss,
    Vfnmsub213ss,
    Vfnmsub231ss,
    Vfmadd132ps,
    Vfmadd213ps,
    Vfmadd231ps,
    Vfnmadd132ps,
    Vfnmadd213ps,
    Vfnmadd231ps,
    Vfmadd132pd,
    Vfmadd213pd,
    Vfmadd231pd,
    Vfnmadd132pd,
    Vfnmadd213pd,
    Vfnmadd231pd,
  }

  #[derive(Debug, Copy, Clone)]
  pub enum LValue {
    L128,
    LIG,
  }

  #[derive(Debug, Copy, Clone)]
  pub enum WValue {
    W0,
    W1,
  }

  #[derive(Debug, Clone)]
  pub struct FmaInstructionData {
    pub instruction: Instruction,
    pub l_value: LValue,
    pub byte_66: u8,
    pub byte_0f: u8,
    pub byte_38: u8,
    pub w_value: WValue,
    pub opcode: u8,
  }

  impl FmaInstructionData {
    pub fn new(
      instruction: Instruction,
      l_value: LValue,
      byte_66: u8,
      byte_0f: u8,
      byte_38: u8,
      w_value: WValue,
      opcode: u8,
    ) -> Self {
      FmaInstructionData {
        instruction,
        l_value,
        byte_66,
        byte_0f,
        byte_38,
        w_value,
        opcode,
      }
    }
  }

  macro_rules! create_fma_instruction_data {
    ($define_instruction:ident, $l_value:ident, $byte_66:expr, $byte_0f:expr, $byte_38:expr, $w_value:ident, $opcode:expr) => {
      Instruction::$define_instruction => FmaInstructionData::new(
        Instruction::$define_instruction,
        LValue::$l_value,
        $byte_66,
        $byte_0f,
        $byte_38,
        WValue::$w_value,
        $opcode,
      ),
    };
  }

  pub fn get_fma_instruction_data(instruction: Instruction) -> FmaInstructionData {
    match instruction {
      Instruction::Vfmadd132sd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0x99,
      ),
      Instruction::Vfmadd213sd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0xA9,
      ),
      Instruction::Vfmadd231sd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0xB9,
      ),
      Instruction::Vfmsub132sd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0x9B,
      ),
      Instruction::Vfmsub213sd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0xAB,
      ),
      Instruction::Vfmsub231sd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0xBB,
      ),
      Instruction::Vfnmadd132sd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0x9D,
      ),
      Instruction::Vfnmadd213sd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0xAD,
      ),
      Instruction::Vfnmadd231sd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0xBD,
      ),
      Instruction::Vfnmsub132sd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0x9F,
      ),
      Instruction::Vfnmsub213sd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0xAF,
      ),
      Instruction::Vfnmsub231sd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0xBF,
      ),
      Instruction::Vfmadd132ss => FmaInstructionData::new(
        instruction,
        LValue::LIG,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0x99,
      ),
      Instruction::Vfmadd213ss => FmaInstructionData::new(
        instruction,
        LValue::LIG,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0xA9,
      ),
      Instruction::Vfmadd231ss => FmaInstructionData::new(
        instruction,
        LValue::LIG,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0xB9,
      ),
      Instruction::Vfmsub132ss => FmaInstructionData::new(
        instruction,
        LValue::LIG,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0x9B,
      ),
      Instruction::Vfmsub213ss => FmaInstructionData::new(
        instruction,
        LValue::LIG,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0xAB,
      ),
      Instruction::Vfmsub231ss => FmaInstructionData::new(
        instruction,
        LValue::LIG,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0xBB,
      ),
      Instruction::Vfnmadd132ss => FmaInstructionData::new(
        instruction,
        LValue::LIG,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0x9D,
      ),
      Instruction::Vfnmadd213ss => FmaInstructionData::new(
        instruction,
        LValue::LIG,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0xAD,
      ),
      Instruction::Vfnmadd231ss => FmaInstructionData::new(
        instruction,
        LValue::LIG,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0xBD,
      ),
      Instruction::Vfnmsub132ss => FmaInstructionData::new(
        instruction,
        LValue::LIG,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0x9F,
      ),
      Instruction::Vfnmsub213ss => FmaInstructionData::new(
        instruction,
        LValue::LIG,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0xAF,
      ),
      Instruction::Vfnmsub231ss => FmaInstructionData::new(
        instruction,
        LValue::LIG,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0xBF,
      ),
      Instruction::Vfmadd132ps => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0x98,
      ),
      Instruction::Vfmadd213ps => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0xA8,
      ),
      Instruction::Vfmadd231ps => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0xB8,
      ),
      Instruction::Vfnmadd132ps => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0x9C,
      ),
      Instruction::Vfnmadd213ps => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0xAC,
      ),
      Instruction::Vfnmadd231ps => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W0,
        0xBC,
      ),
      Instruction::Vfmadd132pd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0x98,
      ),
      Instruction::Vfmadd213pd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0xA8,
      ),
      Instruction::Vfmadd231pd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0xB8,
      ),
      Instruction::Vfnmadd132pd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0x9C,
      ),
      Instruction::Vfnmadd213pd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0xAC,
      ),
      Instruction::Vfnmadd231pd => FmaInstructionData::new(
        instruction,
        LValue::L128,
        0x66,
        0x0F,
        0x38,
        WValue::W1,
        0xBC,
      ),
    }
  }
}
