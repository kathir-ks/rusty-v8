// Converted from V8 C++ source files:
// Header: baseline-assembler-s390-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod s390 {
use std::sync::{Arc, Mutex, RwLock};
  
use crate::baseline::baseline_assembler::CompressionMode;
use crate::baseline::baseline_assembler::ExternalReference;
use crate::baseline::baseline_assembler::FeedbackSlot;
use crate::baseline::baseline_assembler::Label;
use crate::baseline::baseline_assembler::ScratchRegisterScope;
use crate::codegen::interface_descriptors::BaselineLeaveFrameDescriptor;
use crate::codegen::s390::assembler_s390_inl::Assembler;
use crate::codegen::s390::assembler_s390_inl::Condition;
use crate::codegen::s390::assembler_s390_inl::MemOperand;
use crate::codegen::s390::assembler_s390_inl::Operand;
use crate::codegen::s390::assembler_s390_inl::Register;
use crate::init::bootstrapper::v8;
use crate::objects::literal_objects_inl::SmiValuesAre31Bits;
use crate::snapshot::references::SnapshotSpace;
use crate::init::bootstrapper::If;

    
pub mod baseline_assembler_s390_inl {
        
use crate::baseline::baseline_assembler::BaselineAssembler;
use crate::codegen::s390::assembler_s390_inl::Register;
use crate::codegen::s390::assembler_s390_inl::MemOperand;

        

        
pub struct detail {}
        
impl detail {
        
pub const K_SCRATCH_REGISTERS: [Register; 3] = [Register{code: 8}, Register{code: 13}, Register{code: 1}];
        
pub const K_NUM_SCRATCH_REGISTERS: usize = K_SCRATCH_REGISTERS.len();
        
#[cfg(debug_assertions)]
pub fn clobbers(target: Register, op: MemOperand) -> bool {
            op.rb() == target || op.rx() == target
        }
    }
        
impl BaselineAssembler {
    pub fn register_frame_operand(&self, interpreter_register: i32) -> MemOperand {
        MemOperand {
            base: Register { code: 0 },
            index: Register { code: 0 },
            scale: 0,
            displacement: interpreter_register * 8, // Assuming kSystemPointerSize = 8
        }
    }

    pub fn register_frame_address(&mut self, interpreter_register: i32, rscratch: Register) {
        // Assuming __AddS64 is available in the Rust port of MacroAssembler
        //self.__AddS64(rscratch, fp, interpreter_register * 8); // Assuming kSystemPointerSize = 8
    }

    pub fn feedback_vector_operand(&self) -> MemOperand {
        MemOperand {
            base: Register { code: 0 },
            index: Register { code: 0 },
            scale: 0,
            displacement: 0, //BaselineFrameConstants::kFeedbackVectorFromFp,
        }
    }

    pub fn feedback_cell_operand(&self) -> MemOperand {
        MemOperand {
            base: Register { code: 0 },
            index: Register { code: 0 },
            scale: 0,
            displacement: 0, //BaselineFrameConstants::kFeedbackCellFromFp,
        }
    }

    pub fn jump_target(&self) {
            // NOP on arm.
    }
}

        
        
pub fn jump_if_helper(
            assm: &mut Assembler,
            cc: Condition,
            lhs: Register,
            rhs: Register,
            target: &mut Label,
        ) {
        let width: i32 = 64;
            assert!(width == 64 || width == 32, "only support 64 and 32 bit compare");
        if width == 64 {
            if is_signed(cc) {
                assm.cmps64(lhs, rhs);
            } else {
                assm.cmpu64(lhs, rhs);
            }
        } else {
            if is_signed(cc) {
                assm.cmps32(lhs, rhs);
            } else {
                assm.cmpu32(lhs, rhs);
            }
        }
            assm.b(cc, target);
    }
        
        
pub fn is_signed(cc: Condition) -> bool {
            match cc {
                Condition::kEqual => false,
                Condition::kNotEqual => false,
                Condition::kLessThan => true,
                Condition::kGreaterThanOrEqual => true,
                Condition::kGreaterThan => true,
                Condition::kLessThanOrEqual => true,
                Condition::kUnsignedLessThan => false,
                Condition::kUnsignedGreaterThanOrEqual => false,
                Condition::kUnsignedGreaterThan => false,
                Condition::kUnsignedLessThanOrEqual => false,
                _ => false,
            }
        }
}
}
