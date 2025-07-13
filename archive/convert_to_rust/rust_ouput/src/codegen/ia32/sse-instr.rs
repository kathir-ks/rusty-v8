// Converted from V8 C++ source files:
// Header: sse-instr.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod sse_instr {

    macro_rules! define_sse_unop_instruction {
        ($name:ident, $byte1:literal, $byte2:literal) => {
            pub struct $name;
            impl $name {
                pub const BYTE_1: u8 = $byte1;
                pub const BYTE_2: u8 = $byte2;
            }
        };
    }

    macro_rules! define_sse2_instruction {
        ($name:ident, $byte0:literal, $byte1:literal, $byte2:literal) => {
            pub struct $name;
            impl $name {
                pub const BYTE_0: u8 = $byte0;
                pub const BYTE_1: u8 = $byte1;
                pub const BYTE_2: u8 = $byte2;
            }
        };
    }

    macro_rules! define_sse2_instruction_sd {
        ($name:ident, $byte0:literal, $byte1:literal, $byte2:literal) => {
            pub struct $name;
            impl $name {
                pub const BYTE_0: u8 = $byte0;
                pub const BYTE_1: u8 = $byte1;
                pub const BYTE_2: u8 = $byte2;
            }
        };
    }

    macro_rules! define_ssse3_instruction {
        ($name:ident, $byte0:literal, $byte1:literal, $byte2:literal, $byte3:literal) => {
            pub struct $name;
            impl $name {
                pub const BYTE_0: u8 = $byte0;
                pub const BYTE_1: u8 = $byte1;
                pub const BYTE_2: u8 = $byte2;
                pub const BYTE_3: u8 = $byte3;
            }
        };
    }

    macro_rules! define_ssse3_unop_instruction {
        ($name:ident, $byte0:literal, $byte1:literal, $byte2:literal, $byte3:literal) => {
            pub struct $name;
            impl $name {
                pub const BYTE_0: u8 = $byte0;
                pub const BYTE_1: u8 = $byte1;
                pub const BYTE_2: u8 = $byte2;
                pub const BYTE_3: u8 = $byte3;
            }
        };
    }

    macro_rules! define_sse4_instruction {
        ($name:ident, $byte0:literal, $byte1:literal, $byte2:literal, $byte3:literal) => {
            pub struct $name;
            impl $name {
                pub const BYTE_0: u8 = $byte0;
                pub const BYTE_1: u8 = $byte1;
                pub const BYTE_2: u8 = $byte2;
                pub const BYTE_3: u8 = $byte3;
            }
        };
    }

    macro_rules! define_sse4_rm_instruction {
        ($name:ident, $byte0:literal, $byte1:literal, $byte2:literal, $byte3:literal) => {
            pub struct $name;
            impl $name {
                pub const BYTE_0: u8 = $byte0;
                pub const BYTE_1: u8 = $byte1;
                pub const BYTE_2: u8 = $byte2;
                pub const BYTE_3: u8 = $byte3;
            }
        };
    }

    macro_rules! define_avx2_broadcast_instruction {
        ($name:ident, $byte0:literal, $byte1:literal, $byte2:literal, $byte3:literal) => {
            pub struct $name;
            impl $name {
                pub const BYTE_0: u8 = $byte0;
                pub const BYTE_1: u8 = $byte1;
                pub const BYTE_2: u8 = $byte2;
                pub const BYTE_3: u8 = $byte3;
            }
        };
    }


    define_sse_unop_instruction!(sqrtps, 0x0F, 0x51);
    define_sse_unop_instruction!(rsqrtps, 0x0F, 0x52);
    define_sse_unop_instruction!(rcpps, 0x0F, 0x53);
    define_sse_unop_instruction!(cvtps2pd, 0x0F, 0x5A);
    define_sse_unop_instruction!(cvtdq2ps, 0x0F, 0x5B);

    define_sse2_instruction!(packsswb, 0x66, 0x0F, 0x63);
    define_sse2_instruction!(packssdw, 0x66, 0x0F, 0x6B);
    define_sse2_instruction!(packuswb, 0x66, 0x0F, 0x67);
    define_sse2_instruction!(pmaddwd, 0x66, 0x0F, 0xF5);
    define_sse2_instruction!(paddb, 0x66, 0x0F, 0xFC);
    define_sse2_instruction!(paddw, 0x66, 0x0F, 0xFD);
    define_sse2_instruction!(paddd, 0x66, 0x0F, 0xFE);
    define_sse2_instruction!(paddq, 0x66, 0x0F, 0xD4);
    define_sse2_instruction!(paddsb, 0x66, 0x0F, 0xEC);
    define_sse2_instruction!(paddsw, 0x66, 0x0F, 0xED);
    define_sse2_instruction!(paddusb, 0x66, 0x0F, 0xDC);
    define_sse2_instruction!(paddusw, 0x66, 0x0F, 0xDD);
    define_sse2_instruction!(pand, 0x66, 0x0F, 0xDB);
    define_sse2_instruction!(pandn, 0x66, 0x0F, 0xDF);
    define_sse2_instruction!(pcmpeqb, 0x66, 0x0F, 0x74);
    define_sse2_instruction!(pcmpeqw, 0x66, 0x0F, 0x75);
    define_sse2_instruction!(pcmpeqd, 0x66, 0x0F, 0x76);
    define_sse2_instruction!(pcmpgtb, 0x66, 0x0F, 0x64);
    define_sse2_instruction!(pcmpgtw, 0x66, 0x0F, 0x65);
    define_sse2_instruction!(pcmpgtd, 0x66, 0x0F, 0x66);
    define_sse2_instruction!(pmaxsw, 0x66, 0x0F, 0xEE);
    define_sse2_instruction!(pmaxub, 0x66, 0x0F, 0xDE);
    define_sse2_instruction!(pminsw, 0x66, 0x0F, 0xEA);
    define_sse2_instruction!(pminub, 0x66, 0x0F, 0xDA);
    define_sse2_instruction!(pmullw, 0x66, 0x0F, 0xD5);
    define_sse2_instruction!(por, 0x66, 0x0F, 0xEB);
    define_sse2_instruction!(psllw, 0x66, 0x0F, 0xF1);
    define_sse2_instruction!(pslld, 0x66, 0x0F, 0xF2);
    define_sse2_instruction!(psllq, 0x66, 0x0F, 0xF3);
    define_sse2_instruction!(pmuludq, 0x66, 0x0F, 0xF4);
    define_sse2_instruction!(pavgb, 0x66, 0x0F, 0xE0);
    define_sse2_instruction!(psraw, 0x66, 0x0F, 0xE1);
    define_sse2_instruction!(psrad, 0x66, 0x0F, 0xE2);
    define_sse2_instruction!(pavgw, 0x66, 0x0F, 0xE3);
    define_sse2_instruction!(pmulhuw, 0x66, 0x0F, 0xE4);
    define_sse2_instruction!(pmulhw, 0x66, 0x0F, 0xE5);
    define_sse2_instruction!(psrlw, 0x66, 0x0F, 0xD1);
    define_sse2_instruction!(psrld, 0x66, 0x0F, 0xD2);
    define_sse2_instruction!(psrlq, 0x66, 0x0F, 0xD3);
    define_sse2_instruction!(psubb, 0x66, 0x0F, 0xF8);
    define_sse2_instruction!(psubw, 0x66, 0x0F, 0xF9);
    define_sse2_instruction!(psubd, 0x66, 0x0F, 0xFA);
    define_sse2_instruction!(psubq, 0x66, 0x0F, 0xFB);
    define_sse2_instruction!(psubsb, 0x66, 0x0F, 0xE8);
    define_sse2_instruction!(psubsw, 0x66, 0x0F, 0xE9);
    define_sse2_instruction!(psubusb, 0x66, 0x0F, 0xD8);
    define_sse2_instruction!(psubusw, 0x66, 0x0F, 0xD9);
    define_sse2_instruction!(punpcklbw, 0x66, 0x0F, 0x60);
    define_sse2_instruction!(punpcklwd, 0x66, 0x0F, 0x61);
    define_sse2_instruction!(punpckldq, 0x66, 0x0F, 0x62);
    define_sse2_instruction!(punpcklqdq, 0x66, 0x0F, 0x6C);
    define_sse2_instruction!(punpckhbw, 0x66, 0x0F, 0x68);
    define_sse2_instruction!(punpckhwd, 0x66, 0x0F, 0x69);
    define_sse2_instruction!(punpckhdq, 0x66, 0x0F, 0x6A);
    define_sse2_instruction!(punpckhqdq, 0x66, 0x0F, 0x6D);
    define_sse2_instruction!(pxor, 0x66, 0x0F, 0xEF);

    define_sse2_instruction_sd!(sqrtsd, 0xF2, 0x0F, 0x51);
    define_sse2_instruction_sd!(addsd, 0xF2, 0x0F, 0x58);
    define_sse2_instruction_sd!(mulsd, 0xF2, 0x0F, 0x59);
    define_sse2_instruction_sd!(cvtsd2ss, 0xF2, 0x0F, 0x5A);
    define_sse2_instruction_sd!(subsd, 0xF2, 0x0F, 0x5C);
    define_sse2_instruction_sd!(minsd, 0xF2, 0x0F, 0x5D);
    define_sse2_instruction_sd!(divsd, 0xF2, 0x0F, 0x5E);
    define_sse2_instruction_sd!(maxsd, 0xF2, 0x0F, 0x5F);

    define_ssse3_instruction!(pshufb, 0x66, 0x0F, 0x38, 0x00);
    define_ssse3_instruction!(phaddw, 0x66, 0x0F, 0x38, 0x01);
    define_ssse3_instruction!(phaddd, 0x66, 0x0F, 0x38, 0x02);
    define_ssse3_instruction!(pmaddubsw, 0x66, 0x0F, 0x38, 0x04);
    define_ssse3_instruction!(psignb, 0x66, 0x0F, 0x38, 0x08);
    define_ssse3_instruction!(psignw, 0x66, 0x0F, 0x38, 0x09);
    define_ssse3_instruction!(psignd, 0x66, 0x0F, 0x38, 0x0A);
    define_ssse3_instruction!(pmulhrsw, 0x66, 0x0F, 0x38, 0x0B);

    define_ssse3_unop_instruction!(pabsb, 0x66, 0x0F, 0x38, 0x1C);
    define_ssse3_unop_instruction!(pabsw, 0x66, 0x0F, 0x38, 0x1D);
    define_ssse3_unop_instruction!(pabsd, 0x66, 0x0F, 0x38, 0x1E);

    define_sse4_instruction!(pmuldq, 0x66, 0x0F, 0x38, 0x28);
    define_sse4_instruction!(pcmpeqq, 0x66, 0x0F, 0x38, 0x29);
    define_sse4_instruction!(packusdw, 0x66, 0x0F, 0x38, 0x2B);
    define_sse4_instruction!(pminsb, 0x66, 0x0F, 0x38, 0x38);
    define_sse4_instruction!(pminsd, 0x66, 0x0F, 0x38, 0x39);
    define_sse4_instruction!(pminuw, 0x66, 0x0F, 0x38, 0x3A);
    define_sse4_instruction!(pminud, 0x66, 0x0F, 0x38, 0x3B);
    define_sse4_instruction!(pmaxsb, 0x66, 0x0F, 0x38, 0x3C);
    define_sse4_instruction!(pmaxsd, 0x66, 0x0F, 0x38, 0x3D);
    define_sse4_instruction!(pmaxuw, 0x66, 0x0F, 0x38, 0x3E);
    define_sse4_instruction!(pmaxud, 0x66, 0x0F, 0x38, 0x3F);
    define_sse4_instruction!(pmulld, 0x66, 0x0F, 0x38, 0x40);

    define_sse4_rm_instruction!(pmovsxbw, 0x66, 0x0F, 0x38, 0x20);
    define_sse4_rm_instruction!(pmovsxwd, 0x66, 0x0F, 0x38, 0x23);
    define_sse4_rm_instruction!(pmovsxdq, 0x66, 0x0F, 0x38, 0x25);
    define_sse4_rm_instruction!(pmovzxbw, 0x66, 0x0F, 0x38, 0x30);
    define_sse4_rm_instruction!(pmovzxbd, 0x66, 0x0F, 0x38, 0x31);
    define_sse4_rm_instruction!(pmovzxwd, 0x66, 0x0F, 0x38, 0x33);
    define_sse4_rm_instruction!(pmovzxdq, 0x66, 0x0F, 0x38, 0x35);
    define_sse4_rm_instruction!(ptest, 0x66, 0x0F, 0x38, 0x17);

    define_avx2_broadcast_instruction!(vpbroadcastd, 0x66, 0x0F, 0x38, 0x58);
    define_avx2_broadcast_instruction!(vpbroadcastb, 0x66, 0x0F, 0x38, 0x78);
    define_avx2_broadcast_instruction!(vpbroadcastw, 0x66, 0x0F, 0x38, 0x79);

}
