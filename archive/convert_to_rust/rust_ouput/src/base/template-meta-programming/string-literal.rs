// Converted from V8 C++ source files:
// Header: string-literal.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod tmp {

    #[cfg(feature = "cpp_class_types_as_template_args")]
    pub mod string_literal {
        use std::array;
        use std::marker::PhantomData;

        pub struct StringLiteral<const N: usize> {
            data_: array::from_fn(|_| 0u8),
            _phantom: PhantomData<[(); N]>,
        }

        impl<const N: usize> StringLiteral<N> {
            pub fn new(s: &[u8; N]) -> Self {
                assert_eq!(s[N - 1], 0, "String must be null terminated");
                let mut data_: [u8; N] = array::from_fn(|_| 0u8);
                data_.copy_from_slice(s);
                StringLiteral {
                    data_: data_,
                    _phantom: PhantomData,
                }
            }

            pub fn size(&self) -> usize {
                assert_eq!(self.data_[N - 1], 0, "String must be null terminated");
                N - 1
            }

            pub fn c_str(&self) -> &[u8; N] {
                &self.data_
            }

            pub fn data(&self) -> &[u8; N] {
                &self.data_
            }
        }
    }

}
