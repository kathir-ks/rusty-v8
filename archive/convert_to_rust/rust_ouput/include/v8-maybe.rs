// Converted from V8 C++ source files:
// Header: v8-maybe.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {
pub mod internal {
pub struct ConditionalStackAllocatedBase<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ConditionalStackAllocatedBase<T> {
    pub fn new() -> Self {
        ConditionalStackAllocatedBase {
            _phantom: std::marker::PhantomData,
        }
    }
}
}
}
pub mod v8_internal {
    pub unsafe fn V8_UNLIKELY(condition: bool) -> bool {
        condition
    }
    pub unsafe fn V8_LIKELY(condition: bool) -> bool {
        condition
    }
}

pub mod v8config {}

pub mod api_internal {
    use crate::V8_EXPORT;

    #[no_mangle]
    pub extern "C" fn FromJustIsNothing() {
        panic!("FromJust called on a Nothing value");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaybeError {
    Nothing,
    InvalidOperation,
}

pub struct Maybe<T> {
    has_value: bool,
    value: Option<T>,
}

impl<T> Maybe<T> {
    #[inline]
    pub fn is_nothing(&self) -> bool {
        !self.has_value
    }

    #[inline]
    pub fn is_just(&self) -> bool {
        self.has_value
    }

    #[inline]
    pub fn to_checked(&self) -> T
    where
        T: Copy,
    {
        self.from_just()
    }

    #[inline]
    pub fn check(&self) {
        if unsafe { v8_internal::V8_UNLIKELY(!self.is_just()) } {
            api_internal::FromJustIsNothing();
        }
    }

    #[inline]
    pub fn to(&self, out: &mut T) -> bool
    where
        T: Copy,
    {
        if unsafe { v8_internal::V8_LIKELY(self.is_just()) } {
            if let Some(val) = self.value {
                *out = val;
            }
        }
        self.is_just()
    }

    #[inline]
    pub fn from_just(&self) -> T
    where
        T: Copy,
    {
        if unsafe { v8_internal::V8_UNLIKELY(!self.is_just()) } {
            api_internal::FromJustIsNothing();
        }
        self.value.unwrap()
    }

    #[inline]
    pub fn from_just_owned(mut self) -> T {
        if unsafe { v8_internal::V8_UNLIKELY(!self.is_just()) } {
            api_internal::FromJustIsNothing();
        }
        self.value.take().unwrap()
    }

    #[inline]
    pub fn from_maybe(&self, default_value: &T) -> T
    where
        T: Copy,
    {
        if self.has_value {
            self.value.unwrap()
        } else {
            *default_value
        }
    }

    #[inline]
    pub fn equals(&self, other: &Maybe<T>) -> bool
    where
        T: PartialEq + Copy,
    {
        (self.is_just() == other.is_just())
            && (!self.is_just() || self.from_just() == other.from_just())
    }

    #[inline]
    pub fn not_equals(&self, other: &Maybe<T>) -> bool
    where
        T: PartialEq + Copy,
    {
        !self.equals(other)
    }
}

impl<T> PartialEq for Maybe<T>
where
    T: PartialEq + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl<T> Maybe<T> {
    fn new_nothing() -> Self {
        Maybe {
            has_value: false,
            value: None,
        }
    }

    fn new_just(t: T) -> Self {
        Maybe {
            has_value: true,
            value: Some(t),
        }
    }
}

pub fn Nothing<T>() -> Maybe<T> {
    Maybe::<T>::new_nothing()
}

pub fn Just<T>(t: T) -> Maybe<T> {
    Maybe::<T>::new_just(t)
}

pub struct MaybeVoid {
    is_valid: bool,
}

impl MaybeVoid {
    #[inline]
    pub fn is_nothing(&self) -> bool {
        !self.is_valid
    }

    #[inline]
    pub fn is_just(&self) -> bool {
        self.is_valid
    }

    #[inline]
    pub fn equals(&self, other: &MaybeVoid) -> bool {
        self.is_just() == other.is_just()
    }

    #[inline]
    pub fn not_equals(&self, other: &MaybeVoid) -> bool {
        !self.equals(other)
    }

    fn new_nothing() -> Self {
        MaybeVoid { is_valid: false }
    }

    fn new_just() -> Self {
        MaybeVoid { is_valid: true }
    }
}

impl PartialEq for MaybeVoid {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

pub fn NothingVoid() -> MaybeVoid {
    MaybeVoid::new_nothing()
}

pub fn JustVoid() -> MaybeVoid {
    MaybeVoid::new_just()
}
