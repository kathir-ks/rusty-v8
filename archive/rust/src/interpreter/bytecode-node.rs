// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod bytecode_node {
    use std::cmp;
    use std::fmt;
    use crate::interpreter::bytecode_source_info::BytecodeSourceInfo;
    use crate::interpreter::bytecodes::{Bytecode, Bytecodes, OperandScale};
    use crate::interpreter::bytecodes::OperandType;
    use crate::interpreter::bytecodes::ImplicitRegisterUse;
    use crate::interpreter::bytecodes::BytecodeOperands;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct BytecodeNode {
        bytecode_: Bytecode,
        operands_: [u32; Bytecodes::K_MAX_OPERANDS],
        operand_count_: i32,
        operand_scale_: OperandScale,
        source_info_: BytecodeSourceInfo,
    }

    impl BytecodeNode {
        #[inline]
        pub fn new(bytecode: Bytecode, source_info: BytecodeSourceInfo) -> Self {
            let node = BytecodeNode {
                bytecode_: bytecode,
                operand_count_: 0,
                operand_scale_: OperandScale::kSingle,
                source_info_: source_info,
                operands_: [0; Bytecodes::K_MAX_OPERANDS],
            };
            debug_assert_eq!(Bytecodes::number_of_operands(bytecode), node.operand_count());
            node
        }

        #[inline]
        pub fn new1(bytecode: Bytecode, operand0: u32, source_info: BytecodeSourceInfo) -> Self {
            let mut node = BytecodeNode {
                bytecode_: bytecode,
                operand_count_: 1,
                operand_scale_: OperandScale::kSingle,
                source_info_: source_info,
                operands_: [0; Bytecodes::K_MAX_OPERANDS],
            };
            debug_assert_eq!(Bytecodes::number_of_operands(bytecode), node.operand_count());
            node.set_operand(0, operand0);
            node
        }

        #[inline]
        pub fn new2(bytecode: Bytecode, operand0: u32, operand1: u32, source_info: BytecodeSourceInfo) -> Self {
            let mut node = BytecodeNode {
                bytecode_: bytecode,
                operand_count_: 2,
                operand_scale_: OperandScale::kSingle,
                source_info_: source_info,
                operands_: [0; Bytecodes::K_MAX_OPERANDS],
            };
            debug_assert_eq!(Bytecodes::number_of_operands(bytecode), node.operand_count());
            node.set_operand(0, operand0);
            node.set_operand(1, operand1);
            node
        }

        #[inline]
        pub fn new3(bytecode: Bytecode, operand0: u32, operand1: u32, operand2: u32, source_info: BytecodeSourceInfo) -> Self {
            let mut node = BytecodeNode {
                bytecode_: bytecode,
                operand_count_: 3,
                operand_scale_: OperandScale::kSingle,
                source_info_: source_info,
                operands_: [0; Bytecodes::K_MAX_OPERANDS],
            };
            debug_assert_eq!(Bytecodes::number_of_operands(bytecode), node.operand_count());
            node.set_operand(0, operand0);
            node.set_operand(1, operand1);
            node.set_operand(2, operand2);
            node
        }

        #[inline]
        pub fn new4(bytecode: Bytecode, operand0: u32, operand1: u32, operand2: u32, operand3: u32, source_info: BytecodeSourceInfo) -> Self {
            let mut node = BytecodeNode {
                bytecode_: bytecode,
                operand_count_: 4,
                operand_scale_: OperandScale::kSingle,
                source_info_: source_info,
                operands_: [0; Bytecodes::K_MAX_OPERANDS],
            };
            debug_assert_eq!(Bytecodes::number_of_operands(bytecode), node.operand_count());
            node.set_operand(0, operand0);
            node.set_operand(1, operand1);
            node.set_operand(2, operand2);
            node.set_operand(3, operand3);
            node
        }

        #[inline]
        pub fn new5(bytecode: Bytecode, operand0: u32, operand1: u32, operand2: u32, operand3: u32, operand4: u32, source_info: BytecodeSourceInfo) -> Self {
            let mut node = BytecodeNode {
                bytecode_: bytecode,
                operand_count_: 5,
                operand_scale_: OperandScale::kSingle,
                source_info_: source_info,
                operands_: [0; Bytecodes::K_MAX_OPERANDS],
            };
            debug_assert_eq!(Bytecodes::number_of_operands(bytecode), node.operand_count());
            node.set_operand(0, operand0);
            node.set_operand(1, operand1);
            node.set_operand(2, operand2);
            node.set_operand(3, operand3);
            node.set_operand(4, operand4);
            node
        }

        // The following macro expansion cannot be easily converted to rust due to the static linking of `Create` methods to `Bytecode::k##Name`.
        // Rust does not easily support this level of dynamic dispatch based on enum variants at compile time.
        // A hand-written function for each bytecode type would be a more idiomatic approach, but would require a complete listing of all bytecodes.
        // macro_rules! define_bytecode_node_creator {
        //     ($Name:ident, $($rest:tt)*) => {
        //         impl BytecodeNode {
        //             #[inline]
        //             pub fn $Name<Operands: ...>(source_info: BytecodeSourceInfo, operands: Operands) -> Self {
        //                 Self::create::<Bytecode::k##Name, __VA_ARGS__>(source_info, operands)
        //             }
        //         }
        //     };
        // }
        //
        // BYTECODE_LIST!(DEFINE_BYTECODE_NODE_CREATOR, DEFINE_BYTECODE_NODE_CREATOR);


        /// Prints the bytecode node to the given output stream.
        pub fn print(&self, os: &mut dyn std::io::Write) -> std::io::Result<()> {
            write!(os, "{:?}", self)
        }

        /// Returns the bytecode of this node.
        pub fn bytecode(&self) -> Bytecode {
            self.bytecode_
        }

        /// Returns the operand at the given index. Panics if the index is out of bounds.
        pub fn operand(&self, i: usize) -> u32 {
            assert!(i < self.operand_count() as usize);
            self.operands_[i]
        }

        /// Returns a pointer to the array of operands.
        pub fn operands(&self) -> &[u32; Bytecodes::K_MAX_OPERANDS] {
            &self.operands_
        }

        /// Updates the first operand.
        pub fn update_operand0(&mut self, operand0: u32) {
            self.set_operand(0, operand0);
        }

        /// Returns the number of operands.
        pub fn operand_count(&self) -> i32 {
            self.operand_count_
        }

        /// Returns the operand scale.
        pub fn operand_scale(&self) -> OperandScale {
            self.operand_scale_
        }

        /// Returns the source information.
        pub fn source_info(&self) -> &BytecodeSourceInfo {
            &self.source_info_
        }

        /// Sets the source information.
        pub fn set_source_info(&mut self, source_info: BytecodeSourceInfo) {
            self.source_info_ = source_info;
        }

        // Equality implementation
        pub fn eq(&self, other: &Self) -> bool {
            if self.bytecode_ != other.bytecode_ ||
               self.operand_count_ != other.operand_count_ ||
               self.operand_scale_ != other.operand_scale_ ||
               self.source_info_ != other.source_info_ {
                return false;
            }

            for i in 0..self.operand_count_ as usize {
                if self.operands_[i] != other.operands_[i] {
                    return false;
                }
            }

            true
        }

        fn create0<const BYTECODE: Bytecode, const ACCUM_USE: ImplicitRegisterUse>(source_info: BytecodeSourceInfo) -> BytecodeNode {
            BytecodeNode::new(BYTECODE, source_info)
        }

        fn create1<const BYTECODE: Bytecode, const ACCUM_USE: ImplicitRegisterUse, const OPERAND0_TYPE: OperandType>(source_info: BytecodeSourceInfo, operand0: u32) -> BytecodeNode {
            debug_assert_eq!(Bytecodes::get_operand_type(BYTECODE, 0), OPERAND0_TYPE);
            let mut scale = OperandScale::kSingle;
            scale = cmp::max(scale, Self::scale_for_operand::<OPERAND0_TYPE>(operand0));
            BytecodeNode {
                bytecode_: BYTECODE,
                operand_count_: 1,
                operand_scale_: scale,
                source_info_: source_info,
                operands_: [operand0, 0, 0, 0, 0],
            }
        }

        fn create2<const BYTECODE: Bytecode, const ACCUM_USE: ImplicitRegisterUse, const OPERAND0_TYPE: OperandType, const OPERAND1_TYPE: OperandType>(source_info: BytecodeSourceInfo, operand0: u32, operand1: u32) -> BytecodeNode {
            debug_assert_eq!(Bytecodes::get_operand_type(BYTECODE, 0), OPERAND0_TYPE);
            debug_assert_eq!(Bytecodes::get_operand_type(BYTECODE, 1), OPERAND1_TYPE);
            let mut scale = OperandScale::kSingle;
            scale = cmp::max(scale, Self::scale_for_operand::<OPERAND0_TYPE>(operand0));
            scale = cmp::max(scale, Self::scale_for_operand::<OPERAND1_TYPE>(operand1));

            BytecodeNode {
                bytecode_: BYTECODE,
                operand_count_: 2,
                operand_scale_: scale,
                source_info_: source_info,
                operands_: [operand0, operand1, 0, 0, 0],
            }
        }

       fn create3<const BYTECODE: Bytecode, const ACCUM_USE: ImplicitRegisterUse, const OPERAND0_TYPE: OperandType, const OPERAND1_TYPE: OperandType, const OPERAND2_TYPE: OperandType>(source_info: BytecodeSourceInfo, operand0: u32, operand1: u32, operand2: u32) -> BytecodeNode {
            debug_assert_eq!(Bytecodes::get_operand_type(BYTECODE, 0), OPERAND0_TYPE);
            debug_assert_eq!(Bytecodes::get_operand_type(BYTECODE, 1), OPERAND1_TYPE);
            debug_assert_eq!(Bytecodes::get_operand_type(BYTECODE, 2), OPERAND2_TYPE);
            let mut scale = OperandScale::kSingle;
            scale = cmp::max(scale, Self::scale_for_operand::<OPERAND0_TYPE>(operand0));
            scale = cmp::max(scale, Self::scale_for_operand::<OPERAND1_TYPE>(operand1));
            scale = cmp::max(scale, Self::scale_for_operand::<OPERAND2_TYPE>(operand2));

            BytecodeNode {
                bytecode_: BYTECODE,
                operand_count_: 3,
                operand_scale_: scale,
                source_info_: source_info,
                operands_: [operand0, operand1, operand2, 0, 0],
            }
        }

        fn create4<const BYTECODE: Bytecode, const ACCUM_USE: ImplicitRegisterUse, const OPERAND0_TYPE: OperandType, const OPERAND1_TYPE: OperandType, const OPERAND2_TYPE: OperandType, const OPERAND3_TYPE: OperandType>(source_info: BytecodeSourceInfo, operand0: u32, operand1: u32, operand2: u32, operand3: u32) -> BytecodeNode {
            debug_assert_eq!(Bytecodes::get_operand_type(BYTECODE, 0), OPERAND0_TYPE);
            debug_assert_eq!(Bytecodes::get_operand_type(BYTECODE, 1), OPERAND1_TYPE);
            debug_assert_eq!(Bytecodes::get_operand_type(BYTECODE, 2), OPERAND2_TYPE);
            debug_assert_eq!(Bytecodes::get_operand_type(BYTECODE, 3), OPERAND3_TYPE);
            let mut scale = OperandScale::kSingle;
            scale = cmp::max(scale, Self::scale_for_operand::<OPERAND0_TYPE>(operand0));
            scale = cmp::max(scale, Self::scale_for_operand::<OPERAND1_TYPE>(operand1));
            scale = cmp::max(scale, Self::scale_for_operand::<OPERAND2_TYPE>(operand2));
            scale = cmp::max(scale, Self::scale_for_operand::<OPERAND3_TYPE>(operand3));

            BytecodeNode {
                bytecode_: BYTECODE,
                operand_count_: 4,
                operand_scale_: scale,
                source_info_: source_info,
                operands_: [operand0, operand1, operand2, operand3, 0],
            }
        }

       fn create5<const BYTECODE: Bytecode, const ACCUM_USE: ImplicitRegisterUse, const OPERAND0_TYPE: OperandType, const OPERAND1_TYPE: OperandType, const OPERAND2_TYPE: OperandType, const OPERAND3_TYPE: OperandType, const OPERAND4_TYPE: OperandType>(source_info: BytecodeSourceInfo, operand0: u32, operand1: u32, operand2: u32, operand3: u32, operand4: u32) -> BytecodeNode {
            debug_assert_eq!(Bytecodes::get_operand_type(BYTECODE, 0), OPERAND0_TYPE);
            debug_assert_eq!(Bytecodes::get_operand_type(BYTECODE, 1), OPERAND1_TYPE);
            debug_assert_eq!(Bytecodes::get_operand_type(BYTECODE, 2), OPERAND2_TYPE);
            debug_assert_eq!(Bytecodes::get_operand_type(BYTECODE, 3), OPERAND3_TYPE);
            debug_assert_eq!(Bytecodes::get_operand_type(BYTECODE, 4), OPERAND4_TYPE);
            let mut scale = OperandScale::kSingle;
            scale = cmp::max(scale, Self::scale_for_operand::<OPERAND0_TYPE>(operand0));
            scale = cmp::max(scale, Self::scale_for_operand::<OPERAND1_TYPE>(operand1));
            scale = cmp::max(scale, Self::scale_for_operand::<OPERAND2_TYPE>(operand2));
            scale = cmp::max(scale, Self::scale_for_operand::<OPERAND3_TYPE>(operand3));
            scale = cmp::max(scale, Self::scale_for_operand::<OPERAND4_TYPE>(operand4));

            BytecodeNode {
                bytecode_: BYTECODE,
                operand_count_: 5,
                operand_scale_: scale,
                source_info_: source_info,
                operands_: [operand0, operand1, operand2, operand3, operand4],
            }
        }


        #[inline]
        fn scale_for_operand<const OPERAND_TYPE: OperandType>(operand: u32) -> OperandScale {
            if BytecodeOperands::is_scalable_unsigned_byte(OPERAND_TYPE) {
                Bytecodes::scale_for_unsigned_operand(operand)
            } else if BytecodeOperands::is_scalable_signed_byte(OPERAND_TYPE) {
                Bytecodes::scale_for_signed_operand(operand)
            } else {
                OperandScale::kSingle
            }
        }

        #[inline]
        fn update_scale_for_operand(&mut self, operand_index: usize, operand: u32) {
            if Bytecodes::operand_is_scalable_signed_byte(self.bytecode(), operand_index) {
                self.operand_scale_ = cmp::max(self.operand_scale_, Bytecodes::scale_for_signed_operand(operand));
            } else if Bytecodes::operand_is_scalable_unsigned_byte(self.bytecode(), operand_index) {
                self.operand_scale_ = cmp::max(self.operand_scale_, Bytecodes::scale_for_unsigned_operand(operand));
            }
        }

        #[inline]
        fn set_operand(&mut self, operand_index: usize, operand: u32) {
            self.operands_[operand_index] = operand;
            self.update_scale_for_operand(operand_index, operand);
        }
    }

    impl PartialEq for BytecodeNode {
        fn eq(&self, other: &Self) -> bool {
            self.bytecode_ == other.bytecode_ &&
            self.operands_ == other.operands_ &&
            self.operand_count_ == other.operand_count_ &&
            self.operand_scale_ == other.operand_scale_ &&
            self.source_info_ == other.source_info_
        }
    }

    impl fmt::Display for BytecodeNode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BytecodeNode {{ bytecode_: {:?}, operands_: {:?}, operand_count_: {}, operand_scale_: {:?}, source_info_: {:?} }}",
                   self.bytecode_, self.operands_, self.operand_count_, self.operand_scale_, self.source_info_)
        }
    }

    pub fn operator_lshift(os: &mut dyn std::io::Write, node: &BytecodeNode) -> std::io::Result<()> {
        write!(os, "{}", node)
    }
}