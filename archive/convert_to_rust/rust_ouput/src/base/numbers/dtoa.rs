// Converted from V8 C++ source files:
// Header: dtoa.h
// Implementation: dtoa.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod numbers {
        pub mod dtoa {
            use crate::base::numbers::bignum_dtoa::{BignumDtoa, BignumDtoaMode};
            use crate::base::numbers::double::Double;
            use crate::base::numbers::fast_dtoa::FastDtoa;
            use crate::base::numbers::fast_dtoa::FastDtoaMode;
            use crate::base::numbers::fixed_dtoa::FastFixedDtoa;
            use crate::base::vector::Vector;

            #[derive(PartialEq, Eq, Debug, Copy, Clone)]
            pub enum DtoaMode {
                DTOA_SHORTEST,
                DTOA_FIXED,
                DTOA_PRECISION,
            }

            pub const K_BASE10_MAXIMAL_LENGTH: i32 = 17;

            fn dtoa_to_bignum_dtoa_mode(dtoa_mode: DtoaMode) -> BignumDtoaMode {
                match dtoa_mode {
                    DtoaMode::DTOA_SHORTEST => BignumDtoaMode::BIGNUM_DTOA_SHORTEST,
                    DtoaMode::DTOA_FIXED => BignumDtoaMode::BIGNUM_DTOA_FIXED,
                    DtoaMode::DTOA_PRECISION => BignumDtoaMode::BIGNUM_DTOA_PRECISION,
                }
            }

            pub fn double_to_ascii(
                v: f64,
                mode: DtoaMode,
                requested_digits: i32,
                mut buffer: Vector<char>,
                sign: &mut i32,
                length: &mut i32,
                point: &mut i32,
            ) {
                assert!(!Double::new(v).is_special());
                assert!(mode == DtoaMode::DTOA_SHORTEST || requested_digits >= 0);

                if Double::new(v).sign() < 0 {
                    *sign = 1;
                    let mut v = v;
                    v = -v;
                } else {
                    *sign = 0;
                }

                if v == 0.0 {
                    buffer[0] = '0';
                    buffer[1] = '\0';
                    *length = 1;
                    *point = 1;
                    return;
                }

                if mode == DtoaMode::DTOA_PRECISION && requested_digits == 0 {
                    buffer[0] = '\0';
                    *length = 0;
                    return;
                }

                let fast_worked: bool;
                match mode {
                    DtoaMode::DTOA_SHORTEST => {
                        fast_worked = FastDtoa(
                            v,
                            FastDtoaMode::FAST_DTOA_SHORTEST,
                            0,
                            &mut buffer,
                            length,
                            point,
                        );
                    }
                    DtoaMode::DTOA_FIXED => {
                        fast_worked = FastFixedDtoa(v, requested_digits, &mut buffer, length, point);
                    }
                    DtoaMode::DTOA_PRECISION => {
                        fast_worked = FastDtoa(
                            v,
                            FastDtoaMode::FAST_DTOA_PRECISION,
                            requested_digits,
                            &mut buffer,
                            length,
                            point,
                        );
                    }
                }
                if fast_worked {
                    return;
                }
                let bignum_mode = dtoa_to_bignum_dtoa_mode(mode);
                BignumDtoa(v, bignum_mode, requested_digits, &mut buffer, length, point);
                buffer[*length as usize] = '\0';
            }
        }
    }
}
