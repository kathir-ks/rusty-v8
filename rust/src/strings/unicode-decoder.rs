// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod unicode_decoder {
    use crate::strings::unicode;
    use std::{mem, slice};

    /// The return value may point to the first aligned word containing the first
    /// non-one-byte character, rather than directly to the non-one-byte character.
    /// If the return value is >= the passed length, the entire string was
    /// one-byte.
    pub fn non_ascii_start(chars: &[u8]) -> u32 {
        let start = chars.as_ptr();
        let length = chars.len();
        let mut current = chars.as_ptr();
        let limit = unsafe { chars.as_ptr().add(length) };
        let k_intptr_size = mem::size_of::<usize>();

        if length >= k_intptr_size {
            // Check unaligned bytes.
            while (current as usize) % k_intptr_size != 0 {
                if unsafe { *current } > unicode::Utf8::K_MAX_ONE_BYTE_CHAR {
                    return unsafe { current.offset_from(start) } as u32;
                }
                unsafe { current = current.add(1) };
            }

            // Check aligned words.
            debug_assert_eq!(unicode::Utf8::K_MAX_ONE_BYTE_CHAR, 0x7F);
            let non_one_byte_mask = usize::MAX / 0xFF * 0x80;

            while unsafe { current.add(k_intptr_size) <= limit } {
                let word = unsafe { *(current as *const usize) };
                if word & non_one_byte_mask != 0 {
                    return unsafe { current.offset_from(start) } as u32;
                }
                unsafe { current = current.add(k_intptr_size) };
            }
        }

        // Check remaining unaligned bytes.
        while current < limit {
            if unsafe { *current } > unicode::Utf8::K_MAX_ONE_BYTE_CHAR {
                return unsafe { current.offset_from(start) } as u32;
            }
            unsafe { current = current.add(1) };
        }

        unsafe { current.offset_from(start) } as u32
    }

    /// Base struct for UTF-8 decoders.
    pub struct Utf8DecoderBase<D> {
        encoding: Encoding,
        non_ascii_start: i32,
        utf16_length: i32,
        _phantom: std::marker::PhantomData<D>,
    }

    impl<D> Utf8DecoderBase<D> {
        /// Represents the encoding type.
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum Encoding {
            /// ASCII encoding.
            Ascii,
            /// Latin-1 encoding.
            Latin1,
            /// UTF-16 encoding.
            Utf16,
            /// Invalid encoding.
            Invalid,
        }

        /// Checks if the decoder is in an invalid state.
        pub fn is_invalid(&self) -> bool
        where
            Self: AsRef<Utf8DecoderBase<D>>,
        {
            if let Some(decoder) = self.as_ref() as &dyn IsInvalid {
                decoder.is_invalid_impl()
            } else {
                panic!("Type D does not implement the IsInvalid trait");
            }
        }

        /// Checks if the encoding is ASCII.
        pub fn is_ascii(&self) -> bool {
            self.encoding == Encoding::Ascii
        }

        /// Checks if the encoding is one-byte (ASCII or Latin1).
        pub fn is_one_byte(&self) -> bool {
            self.encoding <= Encoding::Latin1
        }

        /// Returns the UTF-16 length. Panics if the decoder is invalid.
        pub fn utf16_length(&self) -> i32 {
            assert_ne!(self.encoding, Encoding::Invalid);
            self.utf16_length
        }

        /// Returns the index of the first non-ASCII character. Panics if the decoder is invalid.
        pub fn non_ascii_start(&self) -> i32 {
            assert_ne!(self.encoding, Encoding::Invalid);
            self.non_ascii_start
        }

        /// Decodes UTF-8 data into the provided buffer.
        pub fn decode<Char>(&mut self, out: &mut [Char], data: &[u8])
        where
            Char: From<u32> + Copy,
        {
            let mut utf16_length = 0;
            let mut non_ascii_start = -1;
            let mut encoding = Encoding::Ascii;

            if data.is_empty() {
                self.encoding = encoding;
                self.non_ascii_start = 0;
                self.utf16_length = 0;
                return;
            }

            let non_ascii = non_ascii_start(data);
            if non_ascii < data.len() as u32 {
                non_ascii_start = non_ascii as i32;
                encoding = Encoding::Utf16; // Optimistically assume UTF-16
            } else {
                encoding = Encoding::Ascii;
                self.encoding = encoding;
                self.non_ascii_start = non_ascii_start;
                self.utf16_length = data.len() as i32;
                return;
            }

            let mut i = 0;
            let mut j = 0;
            while i < data.len() {
                if data[i] <= unicode::Utf8::K_MAX_ONE_BYTE_CHAR {
                    out[j] = data[i].into();
                    i += 1;
                    j += 1;
                } else {
                    let mut u = unicode::DecodeUtf8::safe_get(data, &mut i).unwrap_or(0xFFFD);

                    if (0xD800..=0xDFFF).contains(&u) {
                        encoding = Encoding::Utf16;
                    }
                    out[j] = u.into();
                    j += 1;
                }
                utf16_length += 1;
            }
            self.encoding = encoding;
            self.non_ascii_start = non_ascii_start;
            self.utf16_length = utf16_length;
        }

        /// Creates a new `Utf8DecoderBase`.
        pub fn new(data: &[u8]) -> Self {
            Utf8DecoderBase {
                encoding: Encoding::Ascii,
                non_ascii_start: 0,
                utf16_length: 0,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    /// Trait for determining if a decoder is invalid.
    trait IsInvalid {
        fn is_invalid_impl(&self) -> bool;
    }

    /// A UTF-8 decoder. This decoder never fails; an invalid byte sequence
    /// decodes to U+FFFD and then the decode continues.
    pub struct Utf8Decoder {
        base: Utf8DecoderBase<Utf8Decoder>,
    }

    impl Utf8Decoder {
        /// Creates a new `Utf8Decoder`.
        pub fn new(data: &[u8]) -> Self {
            Utf8Decoder {
                base: Utf8DecoderBase::new(data),
            }
        }

        /// Exposes the underlying `Utf8DecoderBase`.
        pub fn base(&self) -> &Utf8DecoderBase<Utf8Decoder> {
            &self.base
        }

        /// Exposes the mutable underlying `Utf8DecoderBase`.
        pub fn base_mut(&mut self) -> &mut Utf8DecoderBase<Utf8Decoder> {
            &mut self.base
        }
    }

    impl AsRef<Utf8DecoderBase<Utf8Decoder>> for Utf8Decoder {
        fn as_ref(&self) -> &Utf8DecoderBase<Utf8Decoder> {
            &self.base
        }
    }

    impl AsMut<Utf8DecoderBase<Utf8Decoder>> for Utf8Decoder {
        fn as_mut(&mut self) -> &mut Utf8DecoderBase<Utf8Decoder> {
            &mut self.base
        }
    }

    impl IsInvalid for Utf8Decoder {
        fn is_invalid_impl(&self) -> bool {
            Utf8DecoderBase::Encoding::Invalid == self.base.encoding
        }
    }

    // TODO: webassembly feature gate for Wtf8Decoder and StrictUtf8Decoder
    // #[cfg(feature = "webassembly")]
    /// A UTF-8 decoder that supports WebAssembly. Like Utf8Decoder above, except
    /// that instead of replacing invalid sequences with U+FFFD, we have a separate
    /// Encoding::kInvalid state, and we also accept isolated surrogates.
    pub struct Wtf8Decoder {
        base: Utf8DecoderBase<Wtf8Decoder>,
    }

    // TODO: webassembly feature gate for Wtf8Decoder and StrictUtf8Decoder
    // #[cfg(feature = "webassembly")]
    impl Wtf8Decoder {
        /// Creates a new `Wtf8Decoder`.
        pub fn new(data: &[u8]) -> Self {
            Wtf8Decoder {
                base: Utf8DecoderBase::new(data),
            }
        }

        /// Exposes the underlying `Utf8DecoderBase`.
        pub fn base(&self) -> &Utf8DecoderBase<Wtf8Decoder> {
            &self.base
        }
        /// Exposes the mutable underlying `Utf8DecoderBase`.
        pub fn base_mut(&mut self) -> &mut Utf8DecoderBase<Wtf8Decoder> {
            &mut self.base
        }
    }
    // TODO: webassembly feature gate for Wtf8Decoder and StrictUtf8Decoder
    // #[cfg(feature = "webassembly")]
    impl AsRef<Utf8DecoderBase<Wtf8Decoder>> for Wtf8Decoder {
        fn as_ref(&self) -> &Utf8DecoderBase<Wtf8Decoder> {
            &self.base
        }
    }
    // TODO: webassembly feature gate for Wtf8Decoder and StrictUtf8Decoder
    // #[cfg(feature = "webassembly")]
    impl AsMut<Utf8DecoderBase<Wtf8Decoder>> for Wtf8Decoder {
        fn as_mut(&mut self) -> &mut Utf8DecoderBase<Wtf8Decoder> {
            &mut self.base
        }
    }

    // TODO: webassembly feature gate for Wtf8Decoder and StrictUtf8Decoder
    // #[cfg(feature = "webassembly")]
    impl IsInvalid for Wtf8Decoder {
        fn is_invalid_impl(&self) -> bool {
            Utf8DecoderBase::Encoding::Invalid == self.base.encoding
        }
    }

    // TODO: webassembly feature gate for Wtf8Decoder and StrictUtf8Decoder
    // #[cfg(feature = "webassembly")]
    /// A UTF-8 decoder. Like Utf8Decoder above, except that instead of replacing
    /// invalid sequences with U+FFFD, we have a separate Encoding::kInvalid state.
    pub struct StrictUtf8Decoder {
        base: Utf8DecoderBase<StrictUtf8Decoder>,
    }

    // TODO: webassembly feature gate for Wtf8Decoder and StrictUtf8Decoder
    // #[cfg(feature = "webassembly")]
    impl StrictUtf8Decoder {
        /// Creates a new `StrictUtf8Decoder`.
        pub fn new(data: &[u8]) -> Self {
            StrictUtf8Decoder {
                base: Utf8DecoderBase::new(data),
            }
        }

        /// Exposes the underlying `Utf8DecoderBase`.
        pub fn base(&self) -> &Utf8DecoderBase<StrictUtf8Decoder> {
            &self.base
        }
        /// Exposes the mutable underlying `Utf8DecoderBase`.
        pub fn base_mut(&mut self) -> &mut Utf8DecoderBase<StrictUtf8Decoder> {
            &mut self.base
        }
    }
    // TODO: webassembly feature gate for Wtf8Decoder and StrictUtf8Decoder
    // #[cfg(feature = "webassembly")]
    impl AsRef<Utf8DecoderBase<StrictUtf8Decoder>> for StrictUtf8Decoder {
        fn as_ref(&self) -> &Utf8DecoderBase<StrictUtf8Decoder> {
            &self.base
        }
    }
    // TODO: webassembly feature gate for Wtf8Decoder and StrictUtf8Decoder
    // #[cfg(feature = "webassembly")]
    impl AsMut<Utf8DecoderBase<StrictUtf8Decoder>> for StrictUtf8Decoder {
        fn as_mut(&mut self) -> &mut Utf8DecoderBase<StrictUtf8Decoder> {
            &mut self.base
        }
    }

    // TODO: webassembly feature gate for Wtf8Decoder and StrictUtf8Decoder
    // #[cfg(feature = "webassembly")]
    impl IsInvalid for StrictUtf8Decoder {
        fn is_invalid_impl(&self) -> bool {
            Utf8DecoderBase::Encoding::Invalid == self.base.encoding
        }
    }
}

pub mod strings {
    pub mod unicode {
        pub struct Utf8 {}

        impl Utf8 {
            pub const K_MAX_ONE_BYTE_CHAR: u8 = 0x7F;
        }

        pub trait DecodeUtf8 {
            fn safe_get(data: &[u8], i: &mut usize) -> Option<u32>;
        }

        impl DecodeUtf8 for Utf8 {
            fn safe_get(data: &[u8], i: &mut usize) -> Option<u32> {
                if *i >= data.len() {
                    return None;
                }
                let b0 = data[*i];
                if b0 < 0x80 {
                    *i += 1;
                    return Some(b0 as u32);
                }

                if *i + 1 > data.len() {
                    return None;
                }
                let b1 = data[*i + 1];
                if b0 < 0xE0 {
                    *i += 2;
                    return Some(((b0 as u32 & 0x1F) << 6) | (b1 as u32 & 0x3F));
                }

                if *i + 2 > data.len() {
                    return None;
                }
                let b2 = data[*i + 2];
                if b0 < 0xF0 {
                    *i += 3;
                    return Some(
                        ((b0 as u32 & 0x0F) << 12)
                            | ((b1 as u32 & 0x3F) << 6)
                            | (b2 as u32 & 0x3F),
                    );
                }
                if *i + 3 > data.len() {
                    return None;
                }
                let b3 = data[*i + 3];
                *i += 4;
                return Some(
                    ((b0 as u32 & 0x07) << 18)
                        | ((b1 as u32 & 0x3F) << 12)
                        | ((b2 as u32 & 0x3F) << 6)
                        | (b3 as u32 & 0x3F),
                );
            }
        }
    }
}