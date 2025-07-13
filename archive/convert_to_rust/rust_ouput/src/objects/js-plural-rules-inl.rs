// Converted from V8 C++ source files:
// Header: js-plural-rules-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]

use std::marker::PhantomData;

//use crate::objects::js_plural_rules::Type;
//use crate::objects::managed::Managed;
//use crate::objects::objects::Object;
//use crate::objects::tagged_impl::Tagged;

pub mod icu {
    pub struct PluralRules {}
    pub mod number {
        pub struct LocalizedNumberFormatter {}
    }
}

pub mod v8 {
    pub mod internal {
        pub struct Managed<T> {
            phantom: PhantomData<T>,
        }

        impl<T> Managed<T> {
            pub fn new() -> Self {
                Managed { phantom: PhantomData }
            }
        }
        pub struct TaggedObject {}
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidType,
    ICUError,
}

pub struct JSPluralRules {
    icu_plural_rules: Tagged<Managed<icu::PluralRules>>,
    icu_number_formatter: Tagged<Managed<icu::number::LocalizedNumberFormatter>>,
    flags: i32,
}

impl JSPluralRules {
    pub fn icu_plural_rules(&self) -> &Tagged<Managed<icu::PluralRules>> {
        &self.icu_plural_rules
    }
    pub fn set_icu_plural_rules(&mut self, value: Tagged<Managed<icu::PluralRules>>) {
        self.icu_plural_rules = value;
    }
    pub fn icu_number_formatter(&self) -> &Tagged<Managed<icu::number::LocalizedNumberFormatter>> {
        &self.icu_number_formatter
    }
    pub fn set_icu_number_formatter(&mut self, value: Tagged<Managed<icu::number::LocalizedNumberFormatter>>) {
        self.icu_number_formatter = value;
    }
    fn flags(&self) -> i32 {
        self.flags
    }

    fn set_flags(&mut self, flags: i32) {
        self.flags = flags;
    }

    pub fn set_type(&mut self, type_: Type) -> Result<(), Error> {
        if !TypeBit::is_valid(type_ as i32) {
            return Err(Error::InvalidType);
        }
        let mut hints = self.flags();
        hints = TypeBit::update(hints, type_ as i32);
        self.set_flags(hints);
        Ok(())
    }

    pub fn type_(&self) -> Type {
        TypeBit::decode(self.flags())
    }
}

const kIcuPluralRulesOffset: usize = 0;
const kIcuNumberFormatterOffset: usize = 8;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Cardinal,
    Ordinal,
}

mod TypeBit {
    const TYPE_BIT: i32 = 1;
    const TYPE_SHIFT: i32 = 0;

    pub fn update(hints: i32, value: i32) -> i32 {
        (hints & !(TYPE_BIT << TYPE_SHIFT)) | (value << TYPE_SHIFT)
    }

    pub fn decode(hints: i32) -> Type {
        match (hints >> TYPE_SHIFT) & TYPE_BIT {
            0 => Type::Cardinal,
            _ => Type::Ordinal,
        }
    }

    pub fn is_valid(type_: i32) -> bool {
        type_ == 0 || type_ == 1
    }
}

pub struct Tagged<T> {
    phantom: PhantomData<T>,
}

impl<T> Tagged<T> {
    pub fn new() -> Self {
        Tagged { phantom: PhantomData }
    }
}
