// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    fmt::{self, Write},
    mem,
    os::raw::c_char,
    ptr,
};

use bitflags::bitflags;

//use crate::base::platform::{Malloc, Free}; // Assuming these are implemented similarly
//use crate::base::strings; // Assuming these are implemented similarly
//use crate::base::vector::Vector; // Assuming these are implemented similarly
//use crate::codegen::arm64::decoder_arm64_inl::*; // Assuming this is implemented in a separate module
//use crate::codegen::arm64::utils_arm64::*; // Assuming this is implemented in a separate module
//use crate::diagnostics::disasm::*; // Assuming this is implemented in a separate module

const kXRegSizeInBits: usize = 64;
const kWRegSizeInBits: usize = 32;
const kLinkRegCode: u32 = 30;
const NZCV: u32 = 28;
const FPCR: u32 = 2;

macro_rules! UNREACHABLE {
    () => {
        panic!("Unreachable code reached");
    };
}

macro_rules! DCHECK {
    ($x:expr) => {
        if !$x {
            panic!("DCheck failed: {}", stringify!($x));
        }
    };
}

macro_rules! arraysize {
    ($arr:expr) => {
        mem::size_of_val(&$arr) / mem::size_of_val(&$arr[0])
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Extend {
    UXTX,
    SXTX,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    eq,
    ne,
    cs,
    cc,
    mi,
    pl,
    vs,
    vc,
    hi,
    ls,
    ge,
    lt,
    gt,
    le,
    al,
    nv,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LSL {
    LSL,
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct AddSubImmediateMask: u32 {
        const ADD_w_imm = 0b00000000000000000000000000000001; // Example value
        const ADD_x_imm = 0b00000000000000000000000000000010;
        const ADDS_w_imm = 0b00000000000000000000000000000100;
        const ADDS_x_imm = 0b00000000000000000000000000001000;
        const SUB_w_imm = 0b00000000000000000000000000010000;
        const SUB_x_imm = 0b00000000000000000000000000100000;
        const SUBS_w_imm = 0b00000000000000000000000001000000;
        const SUBS_x_imm = 0b00000000000000000000000010000000;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct AddSubShiftedMask: u32 {
        const ADD_w_shift = 0b00000000000000000000000000000001;
        const ADD_x_shift = 0b00000000000000000000000000000010;
        const ADDS_w_shift = 0b00000000000000000000000000000100;
        const ADDS_x_shift = 0b00000000000000000000000000001000;
        const SUB_w_shift = 0b00000000000000000000000000010000;
        const SUB_x_shift = 0b00000000000000000000000000100000;
        const SUBS_w_shift = 0b00000000000000000000000001000000;
        const SUBS_x_shift = 0b00000000000000000000000010000000;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct AddSubExtendedMask: u32 {
        const ADD_w_ext = 0b00000000000000000000000000000001;
        const ADD_x_ext = 0b00000000000000000000000000000010;
        const ADDS_w_ext = 0b00000000000000000000000000000100;
        const ADDS_x_ext = 0b00000000000000000000000000001000;
        const SUB_w_ext = 0b00000000000000000000000000010000;
        const SUB_x_ext = 0b00000000000000000000000000100000;
        const SUBS_w_ext = 0b00000000000000000000000001000000;
        const SUBS_x_ext = 0b00000000000000000000000010000000;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct AddSubWithCarryMask: u32 {
        const ADC_w = 0b00000000000000000000000000000001;
        const ADC_x = 0b00000000000000000000000000000010;
        const ADCS_w = 0b00000000000000000000000000000100;
        const ADCS_x = 0b00000000000000000000000000001000;
        const SBC_w = 0b00000000000000000000000000010000;
        const SBC_x = 0b00000000000000000000000000100000;
        const SBCS_w = 0b00000000000000000000000001000000;
        const SBCS_x = 0b00000000000000000000000010000000;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct LogicalImmediateMask: u32 {
        const AND_w_imm = 0b00000000000000000000000000000001;
        const AND_x_imm = 0b00000000000000000000000000000010;
        const ORR_w_imm = 0b00000000000000000000000000000100;
        const ORR_x_imm = 0b00000000000000000000000000001000;
        const EOR_w_imm = 0b00000000000000000000000000010000;
        const EOR_x_imm = 0b00000000000000000000000000100000;
        const ANDS_w_imm = 0b00000000000000000000000001000000;
        const ANDS_x_imm = 0b00000000000000000000000010000000;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct LogicalShiftedMask: u32 {
        const AND_w = 0b00000000000000000000000000000001;
        const AND_x = 0b00000000000000000000000000000010;
        const BIC_w = 0b00000000000000000000000000000100;
        const BIC_x = 0b00000000000000000000000000001000;
        const EOR_w = 0b00000000000000000000000000010000;
        const EOR_x = 0b00000000000000000000000000100000;
        const EON_w = 0b00000000000000000000000001000000;
        const EON_x = 0b00000000000000000000000010000000;
        const BICS_w = 0b000000000000000000000000010000000;
        const BICS_x = 0b000000000000000000000000100000000;
        const ANDS_w = 0b000000000000000000000000000000001;
        const ANDS_x = 0b000000000000000000000000000000010;
        const ORR_w = 0b000000000000000000000000000000100;
        const ORR_x = 0b000000000000000000000000000001000;
        const ORN_w = 0b000000000000000000000000000100000;
        const ORN_x = 0b000000000000000000000000001000000;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct ConditionalCompareRegisterMask: u32 {
        const CCMN_w = 0b00000000000000000000000000000001;
        const CCMN_x = 0b00000000000000000000000000000010;
        const CCMP_w = 0b00000000000000000000000000000100;
        const CCMP_x = 0b00000000000000000000000000001000;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct ConditionalCompareImmediateMask: u32 {
        const CCMN_w_imm = 0b00000000000000000000000000000001;
        const CCMN_x_imm = 0b00000000000000000000000000000010;
        const CCMP_w_imm = 0b00000000000000000000000000000100;
        const CCMP_x_imm = 0b00000000000000000000000000001000;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct ConditionalSelectMask: u32 {
        const CSEL_w = 0b00000000000000000000000000000001;
        const CSEL_x = 0b00000000000000000000000000000010;
        const CSINC_w = 0b00000000000000000000000000000100;
        const CSINC_x = 0b00000000000000000000000000001000;
        const CSINV_w = 0b00000000000000000000000000010000;
        const CSINV_x = 0b00000000000000000000000000100000;
        const CSNEG_w = 0b00000000000000000000000001000000;
        const CSNEG_x = 0b00000000000000000000000010000000;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct BitfieldMask: u32 {
        const SBFM_w = 0b00000000000000000000000000000001;
        const SBFM_x = 0b00000000000000000000000000000010;
        const UBFM_w = 0b00000000000000000000000000000100;
        const UBFM_x = 0b00000000000000000000000000001000;
        const BFM_w = 0b00000000000000000000000000010000;
        const BFM_x = 0b00000000000000000000000000100000;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct ExtractMask: u32 {
        const EXTR_w = 0b00000000000000000000000000000001;
        const EXTR_x = 0b00000000000000000000000000000010;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct PCRelAddressingMask: u32 {
        const ADR = 0b00000000000000000000000000000001;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct ConditionalBranchMask: u32 {
        const B_cond = 0b00000000000000000000000000000001;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct UnconditionalBranchToRegisterMask: u32 {
        const BR = 0b00000000000000000000000000000001;
        const BLR = 0b00000000000000000000000000000010;
        const RET = 0b00000000000000000000000000000100;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct UnconditionalBranchMask: u32 {
        const B = 0b00000000000000000000000000000001;
        const BL = 0b00000000000000000000000000000010;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct DataProcessing1SourceMask: u32 {
        const RBIT_w = 0b00000000000000000000000000000001;
        const RBIT_x = 0b00000000000000000000000000000010;
        const REV16_w = 0b00000000000000000000000000000100;
        const REV16_x = 0b00000000000000000000000000001000;
        const REV_w = 0b00000000000000000000000000010000;
        const REV_x = 0b00000000000000000000000000100000;
        const CLZ_w = 0b00000000000000000000000001000000;
        const CLZ_x = 0b00000000000000000000000010000000;
        const CLS_w = 0b000000000000000000000000000000001;
        const CLS_x = 0b000000000000000000000000000000010;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct DataProcessing2SourceMask: u32 {
        const UDIV_w = 0b00000000000000000000000000000001;
        const UDIV_x = 0b00000000000000000000000000000010;
        const SDIV_w = 0b00000000000000000000000000000100;
        const SDIV_x = 0b00000000000000000000000000001000;
        const LSLV_w = 0b00000000000000000000000000010000;
        const LSLV_x = 0b00000000000000000000000000100000;
        const LSRV_w = 0b00000000000000000000000001000000;
        const LSRV_x = 0b00000000000000000000000010000000;
        const ASRV_w = 0b000000000000000000000000000000001;
        const ASRV_x = 0b000000000000000000000000000000010;
        const RORV_w = 0b000000000000000000000000000000100;
        const RORV_x = 0b000000000000000000000000000001000;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct DataProcessing3SourceMask: u32 {
        const MADD_w = 0b00000000000000000000000000000001;
        const MADD_x = 0b00000000000000000000000000000010;
        const MSUB_w = 0b00000000000000000000000000000100;
        const MSUB_x = 0b00000000000000000000000000001000;
        const SMADDL_x = 0b00000000000000000000000000010000;
        const SMSUBL_x = 0b00000000000000000000000000100000;
        const UMADDL_x = 0b00000000000000000000000001000000;
        const UMSUBL_x = 0b00000000000000000000000010000000;
        const SMULH_x = 0b000000000000000000000000000000001;
        const UMULH_x = 0b000000000000000000000000000000010;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct CompareBranchMask: u32 {
        const CBZ_w = 0b00000000000000000000000000000001;
        const CBZ_x = 0b00000000000000000000000000000010;
        const CBNZ_w = 0b00000000000000000000000000000100;
        const CBNZ_x = 0b00000000000000000000000000001000;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct TestBranchMask: u32 {
        const TBZ = 0b00000000000000000000000000000001;
        const TBNZ = 0b00000000000000000000000000000010;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct MoveWideImmediateMask: u32 {
        const MOVN_w = 0b00000000000000000000000000000001;
        const MOVN_x = 0b00000000000000000000000000000010;
        const MOVZ_w = 0b00000000000000000000000000000100;
        const MOVZ_x = 0b00000000000000000000000000001000;
        const MOVK_w = 0b00000000000000000000000000010000;
        const MOVK_x = 0b00000000000000000000000000100000;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct LoadStorePreIndexMask: u32 {
        const STRB_w_pre = 0b00000000000000000000000000000001;
        const STRH_w_pre = 0b00000000000000000000000000000010;
        const STR_w_pre = 0b00000000000000000000000000000100;
        const STR_x_pre = 0b00000000000000000000000000001000;
        const LDRB_w_pre = 0b00000000000000000000000000010000;
        const LDRH_w_pre = 0b00000000000000000000000000100000;
        const LDR_w_pre = 0b00000000000000000000000001000000;
        const LDR_x_pre = 0b00000000000000000000000010000000;
        const LDRSB_x_pre = 0b00000000000000000000000100000000;
        const LDRSH_x_pre = 0b00000000000000000000001000000000;
        const LDRSW_x_pre = 0b00000000000000000000010000000000;
        const LDRSB_w_pre = 0b00000000000000000000100000000000;
        const LDRSH_w_pre = 0b00000000000000000001000000000000;
        const STR_b_pre = 0b00000000000000000010000000000000;
        const STR_h_pre = 0b00000000000000000100000000000000;
        const STR_s_pre = 0b00000000000000001000000000000000;
        const STR_d_pre = 0b00000000000000010000000000000000;
        const LDR_b_pre = 0b00000000000000100000000000000000;
        const LDR_h_pre = 0b00000000000001000000