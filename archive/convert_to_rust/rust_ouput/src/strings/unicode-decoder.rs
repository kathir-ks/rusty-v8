// Converted from V8 C++ source files:
// Header: unicode-decoder.h
// Implementation: unicode-decoder.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod unicode_decoder {
    use crate::base::vector::Vector;
    use crate::strings::unicode;
    use std::mem;

    const kIntptrSize: usize = mem::size_of::<usize>();
    const kUintptrAllBitsSet: usize = usize::MAX;

    #[inline]
    fn is_aligned(ptr: usize, alignment: usize) -> bool {
        ptr % alignment == 0
    }

    pub fn non_ascii_start(chars: &[u8], length: u32) -> u32 {
        let start = chars.as_ptr();
        let mut chars_ptr = chars.as_ptr();
        let limit = chars.as_ptr() as usize + length as usize;

        if length as usize >= kIntptrSize {
            // Check unaligned bytes.
            while !is_aligned(chars_ptr as usize, kIntptrSize) {
                if unsafe { *chars_ptr } > unicode::Utf8::kMaxOneByteChar {
                    return (chars_ptr as usize - start as usize) as u32;
                }
                chars_ptr = unsafe { chars_ptr.add(1) };
            }

            // Check aligned words.
            assert_eq!(unicode::Utf8::kMaxOneByteChar, 0x7F);
            let non_one_byte_mask = kUintptrAllBitsSet / 0xFF * 0x80;
            while (chars_ptr as usize + mem::size_of::<usize>() <= limit) {
                let word = unsafe { *(chars_ptr as *const usize) };
                if word & non_one_byte_mask != 0 {
                    return (chars_ptr as usize - start as usize) as u32;
                }
                chars_ptr = unsafe { chars_ptr.add(mem::size_of::<usize>()) };
            }
        }

        // Check remaining unaligned bytes.
        while chars_ptr as usize < limit {
            if unsafe { *chars_ptr } > unicode::Utf8::kMaxOneByteChar {
                return (chars_ptr as usize - start as usize) as u32;
            }
            chars_ptr = unsafe { chars_ptr.add(1) };
        }

        (chars_ptr as usize - start as usize) as u32
    }

    #[allow(dead_code)]
    pub struct Utf8DecoderBase<D: DecoderTraits> {
        encoding_: Encoding,
        non_ascii_start_: i32,
        utf16_length_: i32,
        _phantom: std::marker::PhantomData<D>,
    }

    impl<D: DecoderTraits> Utf8DecoderBase<D> {
        pub fn is_invalid(&self) -> bool {
            D::is_invalid(self)
        }
        pub fn is_ascii(&self) -> bool {
            self.encoding_ == Encoding::kAscii
        }
        pub fn is_one_byte(&self) -> bool {
            self.encoding_ <= Encoding::kLatin1
        }
        pub fn utf16_length(&self) -> i32 {
            assert!(self.is_invalid() == false);
            self.utf16_length_
        }
        pub fn non_ascii_start(&self) -> i32 {
            assert!(self.is_invalid() == false);
            self.non_ascii_start_
        }

        pub fn decode<Char>(&self, out: &mut [Char], data: Vector<u8>) {
            D::decode(self, out, data);
        }
    }

    impl<D: DecoderTraits> Utf8DecoderBase<D> {
        pub fn new(data: Vector<u8>) -> Self {
            let non_ascii_start_ = non_ascii_start(data.begin(), data.length()) as i32;
            let mut utf16_length_ = non_ascii_start_ as i32;
            let mut encoding_ = Encoding::kAscii;

            if non_ascii_start_ == data.length() as i32 {
                return Utf8DecoderBase {
                    encoding_: Encoding::kAscii,
                    non_ascii_start_: non_ascii_start_ as i32,
                    utf16_length_: utf16_length_ as i32,
                    _phantom: std::marker::PhantomData,
                };
            }

            let mut is_one_byte = true;
            let mut state = D::DfaDecoder::kAccept;
            let mut current: u32 = 0;
            let mut previous: u32 = 0;
            let mut cursor = data.begin().add(non_ascii_start_ as usize);
            let end = data.begin().add(data.length() as usize);

            while cursor < end {
                if unsafe { *cursor } <= unicode::Utf8::kMaxOneByteChar && state == D::DfaDecoder::kAccept {
                    assert_eq!(0, current);
                    assert!(!D::is_invalid_surrogate_pair(previous, unsafe { *cursor } as u32));
                    previous = unsafe { *cursor } as u32;
                    utf16_length_ += 1;
                    cursor = unsafe { cursor.add(1) };
                    continue;
                }

                let previous_state = state;
                D::DfaDecoder::decode(unsafe { *cursor }, &mut state, &mut current);
                if state < D::DfaDecoder::kAccept {
                    assert_eq!(state, D::DfaDecoder::kReject);
                    if D::kAllowIncompleteSequences {
                        state = D::DfaDecoder::kAccept;
                        assert!(unicode::Utf8::kBadChar > unicode::Latin1::kMaxChar);
                        is_one_byte = false;
                        utf16_length_ += 1;
                        previous = unicode::Utf8::kBadChar as u32;
                        current = 0;
                        if previous_state != D::DfaDecoder::kAccept {
                            continue;
                        }
                    } else {
                        return Utf8DecoderBase {
                            encoding_: Encoding::kInvalid,
                            non_ascii_start_: non_ascii_start_,
                            utf16_length_: utf16_length_,
                            _phantom: std::marker::PhantomData,
                        };
                    }
                } else if state == D::DfaDecoder::kAccept {
                    if D::is_invalid_surrogate_pair(previous, current) {
                        return Utf8DecoderBase {
                            encoding_: Encoding::kInvalid,
                            non_ascii_start_: non_ascii_start_,
                            utf16_length_: utf16_length_,
                            _phantom: std::marker::PhantomData,
                        };
                    }
                    is_one_byte = is_one_byte && current <= unicode::Latin1::kMaxChar as u32;
                    utf16_length_ += 1;
                    if current > unicode::Utf16::kMaxNonSurrogateCharCode as u32 {
                        utf16_length_ += 1;
                    }
                    previous = current;
                    current = 0;
                }
                cursor = unsafe { cursor.add(1) };
            }

            if state == D::DfaDecoder::kAccept {
                encoding_ = if is_one_byte { Encoding::kLatin1 } else { Encoding::kUtf16 };
            } else if D::kAllowIncompleteSequences {
                assert!(unicode::Utf8::kBadChar > unicode::Latin1::kMaxChar);
                encoding_ = Encoding::kUtf16;
                utf16_length_ += 1;
            } else {
                encoding_ = Encoding::kInvalid;
            }

            Utf8DecoderBase {
                encoding_: encoding_,
                non_ascii_start_: non_ascii_start_ as i32,
                utf16_length_: utf16_length_ as i32,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
    #[repr(u8)]
    pub enum Encoding {
        kAscii,
        kLatin1,
        kUtf16,
        kInvalid,
    }

    pub trait DecoderTraits {
        type DfaDecoder: DfaDecoderTrait;
        const kAllowIncompleteSequences: bool;
        fn is_invalid_surrogate_pair(lead: u32, trail: u32) -> bool;
        fn is_invalid<D: DecoderTraits>(slf: &Utf8DecoderBase<D>) -> bool;
        fn decode<D: DecoderTraits, Char>(slf: &Utf8DecoderBase<D>, out: &mut [Char], data: Vector<u8>);
    }

    pub struct Utf8Decoder {}

    impl DecoderTraits for Utf8Decoder {
        type DfaDecoder = Utf8DfaDecoder;
        const kAllowIncompleteSequences: bool = true;
        fn is_invalid_surrogate_pair(lead: u32, trail: u32) -> bool {
            assert!(!unicode::Utf16::is_lead_surrogate(trail));
            assert!(!unicode::Utf16::is_trail_surrogate(trail));
            false
        }
        fn is_invalid<D: DecoderTraits>(slf: &Utf8DecoderBase<D>) -> bool {
            assert_ne!(slf.encoding_, Encoding::kInvalid);
            false
        }
        fn decode<D: DecoderTraits, Char>(slf: &Utf8DecoderBase<D>, out: &mut [Char], data: Vector<u8>) {
            assert!(slf.is_invalid() == false);
            copy_chars(out, data.begin(), slf.non_ascii_start_ as usize);

            let mut out_ptr = out.as_mut_ptr().add(slf.non_ascii_start_ as usize);

            let mut state = Self::DfaDecoder::kAccept;
            let mut current: u32 = 0;
            let mut cursor = data.begin().add(slf.non_ascii_start_ as usize);
            let end = data.begin().add(data.length() as usize);

            while cursor < end {
                if unsafe { *cursor } <= unicode::Utf8::kMaxOneByteChar && state == Self::DfaDecoder::kAccept {
                    assert_eq!(0, current);
                    unsafe { *out_ptr = *cursor as Char };
                    out_ptr = unsafe { out_ptr.add(1) };
                    cursor = unsafe { cursor.add(1) };
                    continue;
                }

                let previous_state = state;
                Self::DfaDecoder::decode(unsafe { *cursor }, &mut state, &mut current);

                if Self::kAllowIncompleteSequences && state < Self::DfaDecoder::kAccept {
                    state = Self::DfaDecoder::kAccept;
                    unsafe { *out_ptr = unicode::Utf8::kBadChar as Char };
                    out_ptr = unsafe { out_ptr.add(1) };
                    current = 0;
                    if previous_state != Self::DfaDecoder::kAccept {
                        continue;
                    }
                } else if state == Self::DfaDecoder::kAccept {
                    if mem::size_of::<Char>() == 1 || current <= unicode::Utf16::kMaxNonSurrogateCharCode as u32 {
                        unsafe { *out_ptr = current as Char };
                        out_ptr = unsafe { out_ptr.add(1) };
                    } else {
                        unsafe { *out_ptr = unicode::Utf16::lead_surrogate(current) as Char };
                        out_ptr = unsafe { out_ptr.add(1) };
                        unsafe { *out_ptr = unicode::Utf16::trail_surrogate(current) as Char };
                        out_ptr = unsafe { out_ptr.add(1) };
                    }
                    current = 0;
                }
                cursor = unsafe { cursor.add(1) };
            }

            if Self::kAllowIncompleteSequences && state != Self::DfaDecoder::kAccept {
                unsafe { *out_ptr = unicode::Utf8::kBadChar as Char };
            } else {
                assert_eq!(state, Self::DfaDecoder::kAccept);
            }
        }
    }

    pub struct Utf8DfaDecoder {}

    impl Utf8DfaDecoder {
        const kAccept: i8 = 0;
        const kReject: i8 = -1;

        fn decode(byte: u8, state: &mut i8, current: &mut u32) {
            if *state == Self::kAccept {
                if byte < 0x80 {
                    *current = byte as u32;
                } else if byte < 0xE0 {
                    *state = 1;
                    *current = (byte as u32 & 0x1F) << 6;
                } else if byte < 0xF0 {
                    *state = 2;
                    *current = (byte as u32 & 0x0F) << 12;
                } else if byte < 0xF8 {
                    *state = 3;
                    *current = (byte as u32 & 0x07) << 18;
                } else {
                    *state = Self::kReject;
                }
            } else if *state == 1 {
                if byte >= 0x80 && byte < 0xC0 {
                    *current |= (byte as u32 & 0x3F);
                    *state = Self::kAccept;
                } else {
                    *state = Self::kReject;
                }
            } else if *state == 2 {
                if byte >= 0x80 && byte < 0xC0 {
                    *current |= (byte as u32 & 0x3F) << 6;
                    *state = 11;
                } else {
                    *state = Self::kReject;
                }
            } else if *state == 3 {
                if byte >= 0x80 && byte < 0xC0 {
                    *current |= (byte as u32 & 0x3F) << 12;
                    *state = 21;
                } else {
                    *state = Self::kReject;
                }
            } else if *state == 11 {
                if byte >= 0x80 && byte < 0xC0 {
                    *current |= (byte as u32 & 0x3F);
                    *state = Self::kAccept;
                } else {
                    *state = Self::kReject;
                }
            } else if *state == 21 {
                if byte >= 0x80 && byte < 0xC0 {
                    *current |= (byte as u32 & 0x3F) << 6;
                    *state = 31;
                } else {
                    *state = Self::kReject;
                }
            } else if *state == 31 {
                if byte >= 0x80 && byte < 0xC0 {
                    *current |= (byte as u32 & 0x3F);
                    *state = Self::kAccept;
                } else {
                    *state = Self::kReject;
                }
            }
        }
    }

    pub trait DfaDecoderTrait {
        const kAccept: i8;
        const kReject: i8;
        fn decode(byte: u8, state: &mut i8, current: &mut u32);
    }

    impl DfaDecoderTrait for Utf8DfaDecoder {
        const kAccept: i8 = Utf8DfaDecoder::kAccept;
        const kReject: i8 = Utf8DfaDecoder::kReject;
        fn decode(byte: u8, state: &mut i8, current: &mut u32) {
            Utf8DfaDecoder::decode(byte, state, current);
        }
    }

    fn copy_chars<Char>(out: &mut [Char], begin: *const u8, non_ascii_start: usize) {
        let mut out_ptr = out.as_mut_ptr();
        let mut begin_ptr = begin;

        for _ in 0..non_ascii_start {
            unsafe {
                *out_ptr = *begin_ptr as Char;
                out_ptr = out_ptr.add(1);
                begin_ptr = begin_ptr.add(1);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::base::vector::Vector;

        #[test]
        fn test_non_ascii_start() {
            let ascii_string = "hello".as_bytes();
            let non_ascii_string = "hello你好".as_bytes();

            assert_eq!(non_ascii_start(ascii_string, ascii_string.len() as u32), ascii_string.len() as u32);
            assert_eq!(non_ascii_start(non_ascii_string, non_ascii_string.len() as u32), 5);
        }

        #[test]
        fn test_utf8_decoder() {
            let data = "hello你好".as_bytes();
            let vector = Vector::from_slice(data);
            let decoder = Utf8DecoderBase::<Utf8Decoder>::new(vector);
            assert_eq!(decoder.encoding_, Encoding::kUtf16);
            assert_eq!(decoder.non_ascii_start_, 5);
            assert_eq!(decoder.utf16_length_, 7);

            let mut output = vec!['\0'; decoder.utf16_length_ as usize];
            decoder.decode(&mut output, Vector::from_slice(data));
            assert_eq!(output.len(), 7);
            assert_eq!(output[0], 'h');
            assert_eq!(output[1], 'e');
            assert_eq!(output[2], 'l');
            assert_eq!(output[3], 'l');
            assert_eq!(output[4], 'o');
            assert_eq!(output[5], '你');
            assert_eq!(output[6], '好');
        }

        #[test]
        fn test_ascii_decoder() {
            let data = "hello".as_bytes();
            let vector = Vector::from_slice(data);
            let decoder = Utf8DecoderBase::<Utf8Decoder>::new(vector);
            assert_eq!(decoder.encoding_, Encoding::kAscii);
            assert_eq!(decoder.non_ascii_start_, 5);
            assert_eq!(decoder.utf16_length_, 5);

            let mut output = vec!['\0'; decoder.utf16_length_ as usize];
            decoder.decode(&mut output, Vector::from_slice(data));
            assert_eq!(output.len(), 5);
            assert_eq!(output[0], 'h');
            assert_eq!(output[1], 'e');
            assert_eq!(output[2], 'l');
            assert_eq!(output[3], 'l');
            assert_eq!(output[4], 'o');
        }

        #[test]
        fn test_latin1_decoder() {
            let data = "hëllo".as_bytes();
            let vector = Vector::from_slice(data);
            let decoder = Utf8DecoderBase::<Utf8Decoder>::new(vector);
            assert_eq!(decoder.encoding_, Encoding::kLatin1);
            assert_eq!(decoder.non_ascii_start_, 1);
            assert_eq!(decoder.utf16_length_, 5);

            let mut output = vec!['\0'; decoder.utf16_length_ as usize];
            decoder.decode(&mut output, Vector::from_slice(data));
            assert_eq!(output.len(), 5);
            assert_eq!(output[0], 'h');
            assert_eq!(output[1], '');
            assert_eq!(output[2], 'l');
            assert_eq!(output[3], 'l');
            assert_eq!(output[4], 'o');
        }

        #[test]
        fn test_bad_utf8_decoder() {
            let data = b"h\xF0llo";
            let vector = Vector::from_slice(data);
            let decoder = Utf8DecoderBase::<Utf8Decoder>::new(vector);
            assert_eq!(decoder.encoding_, Encoding::kUtf16);
            assert_eq!(decoder.non_ascii_start_, 1);
            assert_eq!(decoder.utf16_length_, 6);

            let mut output = vec!['\0'; decoder.utf16_length_ as usize];
            decoder.decode(&mut output, Vector::from_slice(data));
             assert_eq!(output.len(), 6);
            assert_eq!(output[0], 'h');
            assert_eq!(output[1], '');
            assert_eq!(output[2], 'l');
            assert_eq!(output[3], 'l');
            assert_eq!(output[4], 'o');
            assert_eq!(output[5], '');
        }
    }

    // WebAssembly decoders and traits
    #[cfg(feature = "v8_enable_webassembly")]
    pub mod wasm {
        use super::*;
        use crate::base::vector::Vector;

        pub struct Wtf8Decoder {}

        impl DecoderTraits for Wtf8Decoder {
            type DfaDecoder = GeneralizedUtf8DfaDecoder;
            const kAllowIncompleteSequences: bool = false;
            fn is_invalid_surrogate_pair(lead: u32, trail: u32) -> bool {
                unicode::Utf16::is_surrogate_pair(lead, trail)
            }
            fn is_invalid<D: DecoderTraits>(slf: &Utf8DecoderBase<D>) -> bool {
                slf.encoding_ == Encoding::kInvalid
            }
            fn decode<D: DecoderTraits, Char>(slf: &Utf8DecoderBase<D>, out: &mut [Char], data: Vector<u8>) {
                assert!(slf.is_invalid() == false);
                copy_chars(out, data.begin(), slf.non_ascii_start_ as usize);

                let mut out_ptr = out.as_mut_ptr().add(slf.non_ascii_start_ as usize);

                let mut state = Self::DfaDecoder::kAccept;
                let mut current: u32 = 0;
                let mut cursor = data.begin().add(slf.non_ascii_start_ as usize);
                let end = data.begin().add(data.length() as usize);

                while cursor < end {
                    if unsafe { *cursor } <= unicode::Utf8::kMaxOneByteChar && state == Self::DfaDecoder::kAccept {
                        assert_eq!(0, current);
                        unsafe { *out_ptr = *cursor as Char };
                        out_ptr = unsafe { out_ptr.add(1) };
                        cursor = unsafe { cursor.add(1) };
                        continue;
                    }

                    let previous_state = state;
                    Self::DfaDecoder::decode(unsafe { *cursor }, &mut state, &mut current);

                    if Self::kAllowIncompleteSequences && state < Self::DfaDecoder::kAccept {
                        state = Self::DfaDecoder::kAccept;
                        unsafe { *out_ptr = unicode::Utf8::kBadChar as Char };
                        out_ptr = unsafe { out_ptr.add(1) };
                        current = 0;
                        if previous_state != Self::DfaDecoder::kAccept {
                            continue;
                        }
                    } else if state == Self::DfaDecoder::kAccept {
                        if mem::size_of::<Char>() == 1 || current <= unicode::Utf16::kMaxNonSurrogateCharCode as u32 {
                            unsafe { *out_ptr = current as Char };
                            out_ptr = unsafe { out_ptr.add(1) };
                        } else {
                            unsafe { *out_ptr = unicode::Utf16::lead_surrogate(current) as Char };
                            out_ptr = unsafe { out_ptr.add(1) };
                            unsafe { *out_ptr = unicode::Utf16::trail_surrogate(current) as Char };
                            out_ptr = unsafe { out_ptr.add(1) };
                        }
                        current = 0;
                    }
                    cursor = unsafe { cursor.add(1) };
                }

                if Self::kAllowIncompleteSequences && state != Self::DfaDecoder::kAccept {
                    unsafe { *out_ptr = unicode::Utf8::kBadChar as Char };
                } else {
                    assert_eq!(state, Self::DfaDecoder::kAccept);
                }
            }
        }

        pub struct StrictUtf8Decoder {}

        impl DecoderTraits for StrictUtf8Decoder {
            type DfaDecoder = Utf8DfaDecoder;
            const kAllowIncompleteSequences: bool = false;
            fn is_invalid_surrogate_pair(lead: u32, trail: u32) -> bool {
                assert!(!unicode::Utf16::is_lead_surrogate(trail));
                assert!(!unicode::Utf16::is_trail_surrogate(trail));
                false
            }
            fn is_invalid<D: DecoderTraits>(slf: &Utf8DecoderBase<D>) -> bool {
                slf.encoding_ == Encoding::kInvalid
            }
             fn decode<D: DecoderTraits, Char>(slf: &Utf8DecoderBase<D>, out: &mut [Char], data: Vector<u8>) {
                assert!(slf.is_invalid() == false);
                copy_chars(out, data.begin(), slf.non_ascii_start_ as usize);

                let mut out_ptr = out.as_mut_ptr().add(slf.non_ascii_start_ as usize);

                let mut state = Self::DfaDecoder::kAccept;
                let mut current: u32 = 0;
                let mut cursor = data.begin().add(slf.non_ascii_start_ as usize);
                let end = data.begin().add(data.length() as usize);

                while cursor < end {
                    if unsafe { *cursor } <= unicode::Utf8::kMaxOneByteChar && state == Self::DfaDecoder::kAccept {
                        assert_eq!(0, current);
                        unsafe { *out_ptr = *cursor as Char };
                        out_ptr = unsafe { out_ptr.add(1) };
                        cursor = unsafe { cursor.add(1) };
                        continue;
                    }

                    let previous_state = state;
                    Self::DfaDecoder::decode(unsafe { *cursor }, &mut state, &mut current);

                    if Self::kAllowIncompleteSequences && state < Self::DfaDecoder::kAccept {
                        state = Self::DfaDecoder::kAccept;
                        unsafe { *out_ptr = unicode::Utf8::kBadChar as Char };
                        out_ptr = unsafe { out_ptr.add(1) };
                        current = 0;
                        if previous_state != Self::DfaDecoder::kAccept {
                            continue;
                        }
                    } else if state == Self::DfaDecoder::kAccept {
                        if mem::size_of::<Char>() == 1 || current <= unicode::Utf16::kMaxNonSurrogateCharCode as u32 {
                            unsafe { *out_ptr = current as Char };
                            out_ptr = unsafe { out_ptr.add(1) };
                        } else {
                            unsafe { *out_ptr = unicode::Utf16::lead_surrogate(current) as Char };
                            out_ptr = unsafe { out_ptr.add(1) };
                            unsafe { *out_ptr = unicode::Utf16::trail_surrogate(current) as Char };
                            out_ptr = unsafe { out_ptr.add(1) };
                        }
                        current = 0;
                    }
                    cursor = unsafe { cursor.add(1) };
                }

                if Self::kAllowIncompleteSequences && state != Self::DfaDecoder::kAccept {
                    unsafe { *out_ptr = unicode::Utf8::kBadChar as Char };
                } else {
                    assert_eq!(state, Self::DfaDecoder::kAccept);
                }
            }
        }

        pub struct GeneralizedUtf8DfaDecoder {}

        impl GeneralizedUtf8DfaDecoder {
            const kAccept: i8 = 0;
            const kReject: i8 = -1;

            fn decode(byte: u8, state: &mut i8, current: &mut u32) {
                if *state == Self::kAccept {
                    if byte < 0x80 {
                        *current = byte as u32;
                    } else if byte < 0xE0 {
                        *state = 1;
                        *current = (byte as u32 & 0x1F) << 6;
                    } else if byte < 0xF0 {
                        *state = 2;
                        *current = (byte as u32 & 0x0F) << 12;
                    } else if byte < 0xF8 {
                        *state = 3;
                        *current = (byte as u32 & 0x07) << 18;
                    } else {
                        *state = Self::kReject;
                    }
                } else if *state == 1 {
                    if byte >= 0x80 && byte < 0xC0 {
                        *current |= (byte as u32 & 0x3F);
                        *state = Self::kAccept;
                    } else {
                        *state = Self::kReject;
                    }
                } else if *state == 2 {
                    if byte >= 0x80 && byte < 0xC0 {
                        *current |= (byte as u32 & 0x3F) << 6;
                        *state = 11;
                    } else {
                        *state = Self::kReject;
                    }
                } else if *state == 3 {
                    if byte >= 0x80 && byte < 0xC0 {
                        *current |= (byte as u32 & 0x3F) << 12;
                        *state = 21;
                    } else {
                        *state = Self::kReject;
                    }
                } else if *state == 11 {
                    if byte >= 0x80 && byte < 0xC0 {
                        *current |= (byte as u32 & 0x3F);
                        *state = Self::kAccept;
                    } else {
                        *state = Self::kReject;
                    }
                } else if *state == 21 {
                    if byte >= 0x80 && byte < 0xC0 {
                        *current |= (byte as u32 & 0x3F) << 6;
                        *state = 31;
                    } else {
                        *state = Self::kReject;
                    }
                } else if *state == 31 {
                    if byte >= 0x80 && byte < 0xC0 {
                        *current |= (byte as u32 & 0x3F);
                        *state = Self::kAccept;
                    } else {
                        *state = Self::kReject;
                    }
                }
            }
        }

        impl DfaDecoderTrait for GeneralizedUtf8DfaDecoder {
            const kAccept: i8 = GeneralizedUtf8DfaDecoder::kAccept;
            const kReject: i8 = GeneralizedUtf8DfaDecoder::kReject;
            fn decode(byte: u8, state: &mut i8, current: &mut u32) {
                GeneralizedUtf8DfaDecoder::decode(byte, state, current);
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_wtf8_decoder() {
                let data = "hello你好".as_bytes();
                let vector = Vector::from_slice(data);
                let decoder = Utf8DecoderBase::<Wtf8Decoder>::new(vector);
                assert_eq!(decoder.encoding_, Encoding::kUtf16);
                assert_eq!(decoder.non_ascii_start_, 5);
                assert_eq!(decoder.utf16_length_, 7);

                let mut output = vec!['\0'; decoder.utf16_length_ as usize];
                decoder.decode(&mut output, Vector::from_slice(data));
                assert_eq!(output.len(), 7);
                assert_eq!(output[0], 'h');
                assert_eq!(output[1], 'e');
                assert_eq!(output[2], 'l');
                assert_eq!(output[3], 'l');
                assert_eq!(output[4], 'o');
                assert_eq!(output[5], '你');
                assert_eq!(output[6], '好');
            }

            #[test]
            fn test_strict_utf8_decoder() {
                let data = "hello你好".as_bytes();
                let vector = Vector::from_slice(data);
                let decoder = Utf8DecoderBase::<StrictUtf8Decoder>::new(vector);
                assert_eq!(decoder.encoding_, Encoding::kUtf16);
                assert_eq!(decoder.non_ascii_start_, 5);
                assert_eq!(decoder.utf16_length_, 7);

                let mut output = vec!['\0'; decoder.utf16_length_ as usize];
                decoder.decode(&mut output, Vector::from_slice(data));
                assert_eq!(output.len(), 7);
                assert_eq!(output[0], 'h');
                assert_eq!(output[1], 'e');
                assert_eq!(output[2], 'l');
                assert_eq!(output[3], 'l');
                assert_eq!(output[4], 'o');
                assert_eq!(output[5], '你');
                assert_eq!(output[6], '好');
            }

            #[test]
            fn test_invalid_strict_utf8_decoder() {
                let data = b"h\xF0llo";
                let vector = Vector::from_slice(data);
                let decoder = Utf8DecoderBase::<StrictUtf8Decoder>::new(vector);
                assert_eq!(
