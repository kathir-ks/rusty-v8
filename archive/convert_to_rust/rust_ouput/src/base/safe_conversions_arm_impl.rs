// Converted from V8 C++ source files:
// Header: safe_conversions_arm_impl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod safe_conversions_arm_impl {
    use std::marker::PhantomData;
    use std::mem;
    use std::convert::TryFrom;
    use std::convert::TryInto;
    use std::fmt;
    use std::i32;
    use std::u32;

    use crate::base::safe_conversions_impl::*;
    use crate::base::macros::*;

    pub mod v8 {
        pub mod base {
            pub mod internal {
                // Fast saturation to a destination type.
                pub struct SaturateFastAsmOp<Dst, Src> {
                    _dst: PhantomData<Dst>,
                    _src: PhantomData<Src>,
                }

                impl<Dst, Src> SaturateFastAsmOp<Dst, Src> {
                    pub fn do_op(value: Src) -> Dst
                    where
                        Dst: Sized + Copy + TryFrom<i32> + std::fmt::Debug,
                        Src: Sized + Copy + Into<i32> + std::fmt::Debug,
                        i32: TryInto<Dst>,
                        <Dst as TryFrom<i32>>::Error: std::fmt::Debug
                    {
                       
                        let src: i32 = value.into();
                        let result: i32;
                        
                        if std::any::TypeId::of::<Dst>() == std::any::TypeId::of::<i32>() {
                            result = src;
                        }
                        else if std::any::TypeId::of::<Dst>() == std::any::TypeId::of::<u32>() {
                            result = src;
                        }
                        else{
                            result = src;
                        }

                        let converted_result = result.try_into().unwrap_or_else(|_|{
                                if src > 0 {
                                    i32::MAX.try_into().unwrap()
                                } else {
                                    i32::MIN.try_into().unwrap()
                                }

                        });

                        converted_result
                    }
                }
            }
        }
    }
}
