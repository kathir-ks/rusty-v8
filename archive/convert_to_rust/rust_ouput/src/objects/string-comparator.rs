// Converted from V8 C++ source files:
// Header: string-comparator.h
// Implementation: string-comparator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod logging {
        macro_rules! DCHECK {
            ($cond:expr) => {
                if !$cond {
                    panic!("DCHECK failed: {}", stringify!($cond));
                }
            };
        }
        pub(crate) use DCHECK;
    }
}
pub mod common {
pub mod globals {}
}
pub mod objects {
pub mod string {
        use crate::objects::string_comparator::ConsStringIterator;

        pub struct String {
            length: i32,
        }

        impl String {
            pub fn length(&self) -> i32 {
                self.length
            }

            pub fn VisitFlat<T>(
                visitor: &mut T,
                string: Tagged<String>,
                offset: i32,
                access_guard: SharedStringAccessGuardIfNeeded,
            ) -> Tagged<ConsString>
            where
                T: StringVisitor,
            {
                visitor.visit_string(string, offset, access_guard);
                Tagged::<ConsString>::null()
            }
        }

        pub trait StringVisitor {
            fn visit_string(
                &mut self,
                string: Tagged<String>,
                offset: i32,
                access_guard: SharedStringAccessGuardIfNeeded,
            );
        }

        #[derive(Clone, Copy)]
        pub struct Tagged<T> {
            _ptr: *mut T,
        }

        impl<T> Tagged<T> {
            pub fn is_null(&self) -> bool {
                self._ptr.is_null()
            }

            pub fn null() -> Self {
                Tagged {
                    _ptr: std::ptr::null_mut(),
                }
            }
        }

        pub struct ConsString {}
    }
pub mod string_comparator {
        use crate::base::logging::DCHECK;
        use crate::objects::string::{String, StringVisitor, Tagged};

        #[derive(Clone, Copy)]
        pub struct SharedStringAccessGuardIfNeeded {}

        impl SharedStringAccessGuardIfNeeded {
            pub fn new() -> Self {
                SharedStringAccessGuardIfNeeded {}
            }
        }

        pub struct StringComparator {
            state_1_: State,
            state_2_: State,
        }

        impl StringComparator {
            pub fn new() -> Self {
                StringComparator {
                    state_1_: State::new(),
                    state_2_: State::new(),
                }
            }

            pub fn equals(
                &mut self,
                string_1: Tagged<String>,
                string_2: Tagged<String>,
                access_guard: SharedStringAccessGuardIfNeeded,
            ) -> bool {
                let length = string_1.length();
                self.state_1_.init(string_1, access_guard);
                self.state_2_.init(string_2, access_guard);
                while true {
                    let to_check = std::cmp::min(self.state_1_.length_, self.state_2_.length_);
                    DCHECK!(to_check > 0 && to_check <= length);
                    let is_equal;
                    if self.state_1_.is_one_byte_ {
                        if self.state_2_.is_one_byte_ {
                            is_equal = StringComparator::equals_chars::<u8, u8>(
                                &mut self.state_1_,
                                &mut self.state_2_,
                                to_check,
                            );
                        } else {
                            is_equal = StringComparator::equals_chars::<u8, u16>(
                                &mut self.state_1_,
                                &mut self.state_2_,
                                to_check,
                            );
                        }
                    } else {
                        if self.state_2_.is_one_byte_ {
                            is_equal = StringComparator::equals_chars::<u16, u8>(
                                &mut self.state_1_,
                                &mut self.state_2_,
                                to_check,
                            );
                        } else {
                            is_equal = StringComparator::equals_chars::<u16, u16>(
                                &mut self.state_1_,
                                &mut self.state_2_,
                                to_check,
                            );
                        }
                    }
                    if !is_equal {
                        return false;
                    }
                    let length_u = length as usize;
                    let to_check_u = to_check as usize;
                    if length_u < to_check_u {
                        return false;
                    }
                    let length = length - to_check;
                    if length == 0 {
                        return true;
                    }
                    self.state_1_.advance(to_check, access_guard);
                    self.state_2_.advance(to_check, access_guard);
                }
            }

            fn equals_chars<Chars1, Chars2>(
                state_1: &mut State,
                state_2: &mut State,
                to_check: i32,
            ) -> bool {
                let a: &[Chars1] = unsafe {
                    std::slice::from_raw_parts(
                        state_1.buffer8_ as *const Chars1,
                        state_1.length_ as usize,
                    )
                };
                let b: &[Chars2] = unsafe {
                    std::slice::from_raw_parts(
                        state_2.buffer8_ as *const Chars2,
                        state_2.length_ as usize,
                    )
                };
                StringComparator::compare_chars_equal(a, b, to_check)
            }

            fn compare_chars_equal<Chars1: PartialEq, Chars2: PartialEq<Chars1>>(
                a: &[Chars1],
                b: &[Chars2],
                to_check: i32,
            ) -> bool {
                if to_check as usize > a.len() || to_check as usize > b.len() {
                    return false;
                }
                for i in 0..to_check as usize {
                    if a[i] != b[i] {
                        return false;
                    }
                }
                true
            }
        }

        pub struct State {
            iter_: ConsStringIterator,
            is_one_byte_: bool,
            length_: i32,
            buffer8_: *const u8,
            buffer16_: *const u16,
        }

        impl State {
            pub fn new() -> Self {
                State {
                    is_one_byte_: true,
                    length_: 0,
                    buffer8_: std::ptr::null(),
                    buffer16_: std::ptr::null(),
                    iter_: ConsStringIterator::new(),
                }
            }

            pub fn init(
                &mut self,
                string: Tagged<String>,
                access_guard: SharedStringAccessGuardIfNeeded,
            ) {
                let cons_string = String::VisitFlat(self, string, 0, access_guard);
                self.iter_.reset(cons_string);
                if !cons_string.is_null() {
                    let mut offset: i32 = 0;
                    let next_string = self.iter_.next(&mut offset);
                    DCHECK!(offset == 0);
                    String::VisitFlat(self, next_string, 0, access_guard);
                }
            }

            pub fn visit_one_byte_string(&mut self, chars: *const u8, length: i32) {
                self.is_one_byte_ = true;
                self.buffer8_ = chars;
                self.length_ = length;
            }

            pub fn visit_two_byte_string(&mut self, chars: *const u16, length: i32) {
                self.is_one_byte_ = false;
                self.buffer16_ = chars;
                self.length_ = length;
            }

            pub fn advance(
                &mut self,
                consumed: i32,
                access_guard: SharedStringAccessGuardIfNeeded,
            ) {
                DCHECK!(consumed <= self.length_);
                if self.length_ != consumed {
                    if self.is_one_byte_ {
                        self.buffer8_ = unsafe { self.buffer8_.add(consumed as usize) };
                    } else {
                        self.buffer16_ = unsafe { self.buffer16_.add(consumed as usize) };
                    }
                    self.length_ -= consumed;
                    return;
                }
                let mut offset: i32 = 0;
                let next_string = self.iter_.next(&mut offset);
                DCHECK!(offset == 0);
                String::VisitFlat(self, next_string, 0, access_guard);
            }
        }

        impl StringVisitor for State {
            fn visit_string(
                &mut self,
                string: Tagged<String>,
                offset: i32,
                _access_guard: SharedStringAccessGuardIfNeeded,
            ) {
                let length = string.length();
                if length > 0 {
                    let mut vec: Vec<u8> = Vec::with_capacity(length as usize);
                    unsafe {
                        vec.set_len(length as usize);
                    }
                    self.visit_one_byte_string(vec.as_ptr(), length);
                } else {
                    self.visit_one_byte_string(std::ptr::null(), 0);
                }
            }
        }

        pub struct ConsStringIterator {
            current: Tagged<String>,
        }

        impl ConsStringIterator {
            pub fn new() -> Self {
                ConsStringIterator {
                    current: Tagged::<String>::null(),
                }
            }

            pub fn reset(&mut self, cons_string: Tagged<String>) {
                self.current = cons_string;
            }

            pub fn next(&mut self, offset: &mut i32) -> Tagged<String> {
                *offset = 0;
                self.current
            }
        }
    }
}
pub mod utils {
pub mod utils {}
}
