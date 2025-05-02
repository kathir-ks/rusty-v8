// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Defines SSE instructions for the x64 architecture.
///
/// This module provides constants and macros for defining SSE instructions
/// used in the V8 JavaScript engine.

/// Macro for defining SSE unary instructions (AVX version has two operands).
macro_rules! define_sse_unop_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8) = ($op1, $op2);
            )*
        }
    };
}

/// Macro for defining SSE binary instructions (AVX version has three operands).
macro_rules! define_sse_binop_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8) = ($op1, $op2);
            )*
        }
    };
}

/// Macro for defining SSE single-precision scalar instructions.
macro_rules! define_sse_ss_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal, $op3:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8, u8) = ($op1, $op2, $op3);
            )*
        }
    };
}

/// Macro for defining SSE2 packed double-precision instructions.
macro_rules! define_sse2_pd_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal, $op3:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8, u8) = ($op1, $op2, $op3);
            )*
        }
    };
}

/// Macro for defining SSE2 packed integer instructions.
macro_rules! define_sse2_pi_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal, $op3:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8, u8) = ($op1, $op2, $op3);
            )*
        }
    };
}

/// Macro for defining SSE2 shift instructions.
macro_rules! define_sse2_shift_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal, $op3:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8, u8) = ($op1, $op2, $op3);
            )*
        }
    };
}

/// Macro for defining combined SSE2 instructions.
macro_rules! define_sse2_instructions {
    ($name:ident, $pd_name:ident, $pi_name:ident, $shift_name:ident) => {
        pub mod $name {
            pub use super::$pd_name::*;
            pub use super::$pi_name::*;
            pub use super::$shift_name::*;
        }
    };
}

/// Macro for defining SSE2 unary instructions (AVX version has two operands).
macro_rules! define_sse2_unop_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal, $op3:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8, u8) = ($op1, $op2, $op3);
            )*
        }
    };
}

/// Macro for defining SSE2 shift instructions with immediate operand.
macro_rules! define_sse2_shift_imm_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal, $op3:literal, $op4:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8, u8, u8) = ($op1, $op2, $op3, $op4);
            )*
        }
    };
}

/// Macro for defining SSE2 scalar double-precision instructions.
macro_rules! define_sse2_sd_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal, $op3:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8, u8) = ($op1, $op2, $op3);
            )*
        }
    };
}

/// Macro for defining SSSE3 instructions.
macro_rules! define_ssse3_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal, $op3:literal, $op4:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8, u8, u8) = ($op1, $op2, $op3, $op4);
            )*
        }
    };
}

/// Macro for defining SSSE3 unary instructions (AVX version has two operands).
macro_rules! define_ssse3_unop_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal, $op3:literal, $op4:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8, u8, u8) = ($op1, $op2, $op3, $op4);
            )*
        }
    };
}

/// Macro for defining SSE4 instructions.
macro_rules! define_sse4_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal, $op3:literal, $op4:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8, u8, u8) = ($op1, $op2, $op3, $op4);
            )*
        }
    };
}

/// Macro for defining SSE4 unary instructions (AVX version has two operands).
macro_rules! define_sse4_unop_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal, $op3:literal, $op4:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8, u8, u8) = ($op1, $op2, $op3, $op4);
            )*
        }
    };
}

/// Macro for defining SSE4 unary instructions (AVX version has two operands, PMOV variant).
macro_rules! define_sse4_unop_instructions_pmov {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal, $op3:literal, $op4:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8, u8, u8) = ($op1, $op2, $op3, $op4);
            )*
        }
    };
}

/// Macro for defining SSE4 extract instructions.
macro_rules! define_sse4_extract_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal, $op3:literal, $op4:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8, u8, u8) = ($op1, $op2, $op3, $op4);
            )*
        }
    };
}

/// Macro for defining SSE4.2 instructions.
macro_rules! define_sse4_2_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal, $op3:literal, $op4:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8, u8, u8) = ($op1, $op2, $op3, $op4);
            )*
        }
    };
}

/// Macro for defining AVX2 broadcast instructions.
macro_rules! define_avx2_broadcast_instructions {
    ($name:ident, $($instr:ident, $op1:literal, $op2:literal, $op3:literal, $op4:literal),*) => {
        pub mod $name {
            $(
                pub const $instr: (u8, u8, u8, u8) = ($op1, $op2, $op3, $op4);
            )*
        }
    };
}

define_sse_unop_instructions!(
    sse_unop,
    sqrtps, 0x0F, 0x51,
    rsqrtps, 0x0F, 0x52,
    rcpps, 0x0F, 0x53,
    cvtps2pd, 0x0F, 0x5A,
    cvtdq2ps, 0x0F, 0x5B
);

define_sse_binop_instructions!(
    sse_binop,
    unpcklps, 0x0F, 0x14,
    andps, 0x0F, 0x54,
    andnps, 0x0F, 0x55,
    orps, 0x0F, 0x56,
    xorps, 0x0F, 0x57,
    addps, 0x0F, 0x58,
    mulps, 0x0F, 0x59,
    subps, 0x0F, 0x5C,
    minps, 0x0F, 0x5D,
    divps, 0x0F, 0x5E,
    maxps, 0x0F, 0x5F
);

define_sse_ss_instructions!(
    sse_ss,
    sqrtss, 0xF3, 0x0F, 0x51,
    addss, 0xF3, 0x0F, 0x58,
    mulss, 0xF3, 0x0F, 0x59,
    cvtss2sd, 0xF3, 0x0F, 0x5A,
    subss, 0xF3, 0x0F, 0x5C,
    minss, 0xF3, 0x0F, 0x5D,
    divss, 0xF3, 0x0F, 0x5E,
    maxss, 0xF3, 0x0F, 0x5F
);

define_sse2_pd_instructions!(
    sse2_pd,
    andpd, 0x66, 0x0F, 0x54,
    andnpd, 0x66, 0x0F, 0x55,
    orpd, 0x66, 0x0F, 0x56,
    xorpd, 0x66, 0x0F, 0x57,
    addpd, 0x66, 0x0F, 0x58,
    mulpd, 0x66, 0x0F, 0x59,
    subpd, 0x66, 0x0F, 0x5C,
    minpd, 0x66, 0x0F, 0x5D,
    divpd, 0x66, 0x0F, 0x5E,
    maxpd, 0x66, 0x0F, 0x5F
);

define_sse2_pi_instructions!(
    sse2_pi,
    punpcklbw, 0x66, 0x0F, 0x60,
    punpcklwd, 0x66, 0x0F, 0x61,
    punpckldq, 0x66, 0x0F, 0x62,
    packsswb, 0x66, 0x0F, 0x63,
    pcmpgtb, 0x66, 0x0F, 0x64,
    pcmpgtw, 0x66, 0x0F, 0x65,
    pcmpgtd, 0x66, 0x0F, 0x66,
    packuswb, 0x66, 0x0F, 0x67,
    punpckhbw, 0x66, 0x0F, 0x68,
    punpckhwd, 0x66, 0x0F, 0x69,
    punpckhdq, 0x66, 0x0F, 0x6A,
    packssdw, 0x66, 0x0F, 0x6B,
    punpcklqdq, 0x66, 0x0F, 0x6C,
    punpckhqdq, 0x66, 0x0F, 0x6D,
    pcmpeqb, 0x66, 0x0F, 0x74,
    pcmpeqw, 0x66, 0x0F, 0x75,
    pcmpeqd, 0x66, 0x0F, 0x76,
    paddq, 0x66, 0x0F, 0xD4,
    pmullw, 0x66, 0x0F, 0xD5,
    psubusb, 0x66, 0x0F, 0xD8,
    psubusw, 0x66, 0x0F, 0xD9,
    pminub, 0x66, 0x0F, 0xDA,
    pand, 0x66, 0x0F, 0xDB,
    paddusb, 0x66, 0x0F, 0xDC,
    paddusw, 0x66, 0x0F, 0xDD,
    pmaxub, 0x66, 0x0F, 0xDE,
    pandn, 0x66, 0x0F, 0xDF,
    pavgb, 0x66, 0x0F, 0xE0,
    pavgw, 0x66, 0x0F, 0xE3,
    pmulhuw, 0x66, 0x0F, 0xE4,
    pmulhw, 0x66, 0x0F, 0xE5,
    psubsb, 0x66, 0x0F, 0xE8,
    psubsw, 0x66, 0x0F, 0xE9,
    pminsw, 0x66, 0x0F, 0xEA,
    por, 0x66, 0x0F, 0xEB,
    paddsb, 0x66, 0x0F, 0xEC,
    paddsw, 0x66, 0x0F, 0xED,
    pmaxsw, 0x66, 0x0F, 0xEE,
    pxor, 0x66, 0x0F, 0xEF,
    pmuludq, 0x66, 0x0F, 0xF4,
    pmaddwd, 0x66, 0x0F, 0xF5,
    psubb, 0x66, 0x0F, 0xF8,
    psubw, 0x66, 0x0F, 0xF9,
    psubd, 0x66, 0x0F, 0xFA,
    psubq, 0x66, 0x0F, 0xFB,
    paddb, 0x66, 0x0F, 0xFC,
    paddw, 0x66, 0x0F, 0xFD,
    paddd, 0x66, 0x0F, 0xFE
);

define_sse2_shift_instructions!(
    sse2_shift,
    psrlw, 0x66, 0x0F, 0xD1,
    psrld, 0x66, 0x0F, 0xD2,
    psrlq, 0x66, 0x0F, 0xD3,
    psraw, 0x66, 0x0F, 0xE1,
    psrad, 0x66, 0x0F, 0xE2,
    psllw, 0x66, 0x0F, 0xF1,
    pslld, 0x66, 0x0F, 0xF2,
    psllq, 0x66, 0x0F, 0xF3
);

define_sse2_instructions!(
    sse2,
    sse2_pd,
    sse2_pi,
    sse2_shift
);

define_sse2_unop_instructions!(
    sse2_unop,
    ucomisd, 0x66, 0x0F, 0x2E,
    sqrtpd, 0x66, 0x0F, 0x51,
    cvtpd2ps, 0x66, 0x0F, 0x5A,
    cvtps2dq, 0x66, 0x0F, 0x5B,
    cvttpd2dq, 0x66, 0x0F, 0xE6
);

define_sse2_shift_imm_instructions!(
    sse2_shift_imm,
    psrlw, 0x66, 0x0F, 0x71, 0x2,
    psrld, 0x66, 0x0F, 0x72, 0x2,
    psrlq, 0x66, 0x0F, 0x73, 0x2,
    psraw, 0x66, 0x0F, 0x71, 0x4,
    psrad, 0x66, 0x0F, 0x72, 0x4,
    psllw, 0x66, 0x0F, 0x71, 0x6,
    pslld, 0x66, 0x0F, 0x72, 0x6,
    psllq, 0x66, 0x0F, 0x73, 0x6
);

define_sse2_sd_instructions!(
    sse2_sd,
    sqrtsd, 0xF2, 0x0F, 0x51,
    addsd, 0xF2, 0x0F, 0x58,
    mulsd, 0xF2, 0x0F, 0x59,
    cvtsd2ss, 0xF2, 0x0F, 0x5A,
    subsd, 0xF2, 0x0F, 0x5C,
    minsd, 0xF2, 0x0F, 0x5D,
    divsd, 0xF2, 0x0F, 0x5E,
    maxsd, 0xF2, 0x0F, 0x5F
);

define_ssse3_instructions!(
    ssse3,
    pshufb, 0x66, 0x0F, 0x38, 0x00,
    phaddw, 0x66, 0x0F, 0x38, 0x01,
    phaddd, 0x66, 0x0F, 0x38, 0x02,
    pmaddubsw, 0x66, 0x0F, 0x38, 0x04,
    psignb, 0x66, 0x0F, 0x38, 0x08,
    psignw, 0x66, 0x0F, 0x38, 0x09,
    psignd, 0x66, 0x0F, 0x38, 0x0A,
    pmulhrsw, 0x66, 0x0F, 0x38, 0x0B
);

define_ssse3_unop_instructions!(
    ssse3_unop,
    pabsb, 0x66, 0x0F, 0x38, 0x1C,
    pabsw, 0x66, 0x0F, 0x38, 0x1D,
    pabsd, 0x66, 0x0F, 0x38, 0x1E
);

define_sse4_instructions!(
    sse4,
    pmuldq, 0x66, 0x0F, 0x38, 0x28,
    pcmpeqq, 0x66, 0x0F, 0x38, 0x29,
    packusdw, 0x66, 0x0F, 0x38, 0x2B,
    pminsb, 0x66, 0x0F, 0x38, 0x38,
    pminsd, 0x66, 0x0F, 0x38, 0x39,
    pminuw, 0x66, 0x0F, 0x38, 0x3A,
    pminud, 0x66, 0x0F, 0x38, 0x3B,
    pmaxsb, 0x66, 0x0F, 0x38, 0x3C,
    pmaxsd, 0x66, 0x0F, 0x38, 0x3D,
    pmaxuw, 0x66, 0x0F, 0x38, 0x3E,
    pmaxud, 0x66, 0x0F, 0x38, 0x3F,
    pmulld, 0x66, 0x0F, 0x38, 0x40
);

define_sse4_unop_instructions!(
    sse4_unop,
    ptest, 0x66, 0x0F, 0x38, 0x17
);

define_sse4_unop_instructions_pmov!(
    sse4_unop_pmov,
    pmovsxbw, 0x66, 0x0F, 0x38, 0x20,
    pmovsxwd, 0x66, 0x0F, 0x38, 0x23,
    pmovsxdq, 0x66, 0x0F, 0x38, 0x25,
    pmovzxbw, 0x66, 0x0F, 0x38, 0x30,
    pmovzxbd, 0x66, 0x0F, 0x38, 0x31,
    pmovzxwd, 0x66, 0x0F, 0x38, 0x33,
    pmovzxdq, 0x66, 0x0F, 0x38, 0x35
);

define_sse4_extract_instructions!(
    sse4_extract,
    extractps, 0x66, 0x0F, 0x3A, 0x17,
    pextrb, 0x66, 0x0F, 0x3A, 0x14,
    pextrw, 0x66, 0x0F, 0x3A, 0x15,
    pextrd, 0x66, 0x0F, 0x3A, 0x16
);

define_sse4_2_instructions!(
    sse4_2,
    pcmpgtq, 0x66, 0x0F, 0x38, 0x37
);

define_avx2_broadcast_instructions!(
    avx2_broadcast,
    vpbroadcastb, 0x66, 0x0F, 0x38, 0x78,
    vpbroadcastw, 0x66, 0x0F, 0x38, 0x79,
    vpbroadcastd, 0x66, 0x0F, 0x38, 0x58,
    vpbroadcastq, 0x66, 0x0F, 0x38, 0x59
);

pub mod sse4_unop_combined {
    pub use super::sse4_unop::*;
    pub use super::sse4_unop_pmov::*;
}