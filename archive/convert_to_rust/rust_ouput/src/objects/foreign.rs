// Converted from V8 C++ source files:
// Header: foreign.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod foreign {
use crate::V8;
use crate::code;
use crate::Foreign;
use crate::JavaScript;
use crate::HashFieldType;
use crate::UseScratchRegisterScope;
use crate::Address;
use crate::isolate;
use crate::Load;
use crate::ExternalPointerTag;
use crate::x64;
use crate::arm64;
use crate::BodyDescriptor;
use crate::TrustedForeign;
use crate::InstructionOperand;
use std::io::Error;
use crate::Isolate;
use crate::Tagged;
use crate::String;
use crate::JSPluralRules;
use crate::Register;
use crate::Operand;
use crate::Condition;
use crate::PtrComprCageBase;
use crate::MaybeObject;
use crate::FeedbackSlot;
use crate::CodeEntrypointTag;
use crate::ValueType;
use v8_flags::flag_allow_unsafe_weakref;
use std::ptr::null_mut;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};

const kTaggedSize: usize = 8;
const kExternalPointerSlotSize: usize = 8;
const kForeignAddressOffset: usize = 8;
const kAnyForeignExternalPointerTagRange: usize = 0;

pub trait TorqueGeneratedForeign<T, U> {}
pub trait TorqueGeneratedTrustedForeign<T, U> {}
pub trait HeapObject {}
pub trait TrustedObject {}
pub struct FixedBodyDescriptorFor<T> {}
pub struct WithExternalPointer<const OFFSET: usize, const TAG_RANGE: usize> {}
pub struct StackedBodyDescriptor<T1, T2> {}

macro_rules! DECL_PRINTER {
    ($name:ident) => {
        #[allow(unused_variables)]
        fn print(&self) {
            println!("Printing {}", stringify!($name));
        }
    };
}

macro_rules! TQ_OBJECT_CONSTRUCTORS {
    ($name:ident) => {
        impl $name {
            #[allow(dead_code)]
            pub fn new() -> Self {
                Self {}
            }
        }
    };
}

#[derive(Debug)]
pub struct ForeignFields {
    pub heap_object: HeapObjectImpl,
    pub foreign_address: Address,
}

#[derive(Debug, Copy, Clone)]
pub struct Foreign {
    pub ptr: *mut ForeignFields,
}

impl Foreign {
    pub fn new(heap_object: HeapObjectImpl, foreign_address: Address) -> Self {
        let foreign_fields = ForeignFields {
            heap_object,
            foreign_address,
        };
        let ptr = Box::into_raw(Box::new(foreign_fields));
        Foreign { ptr }
    }

    pub fn foreign_address<const TAG: usize>(&self, _isolate: &isolate::Isolate) -> Address {
        unsafe { (*self.ptr).foreign_address }
    }

    pub fn foreign_address_unchecked(&self) -> Address {
        unsafe { (*self.ptr).foreign_address }
    }

    pub fn set_foreign_address<const TAG: usize>(&mut self, _isolate: &mut isolate::Isolate, value: Address) {
        unsafe {
            (*self.ptr).foreign_address = value;
        }
    }

    pub fn init_foreign_address<const TAG: usize>(&mut self, _isolate: &mut isolate::Isolate, initial_value: Address) {
        unsafe {
            (*self.ptr).foreign_address = initial_value;
        }
    }

     pub fn GetTag(&self) -> ExternalPointerTag {
         ExternalPointerTag {}
     }
}

impl Drop for Foreign {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.ptr));
        }
    }
}

impl TorqueGeneratedForeign<Foreign, HeapObjectImpl> for Foreign {}
impl HeapObject for Foreign {}

impl Foreign {
    DECL_PRINTER!(Foreign)
}

TQ_OBJECT_CONSTRUCTORS!(Foreign);

#[derive(Debug)]
pub struct TrustedForeignFields {
    pub heap_object: HeapObjectImpl,
}

#[derive(Debug, Copy, Clone)]
pub struct TrustedForeign {
    pub ptr: *mut TrustedForeignFields,
}

impl TrustedForeign {
    pub fn new(heap_object: HeapObjectImpl) -> Self {
        let trusted_foreign_fields = TrustedForeignFields {
            heap_object,
        };
        let ptr = Box::into_raw(Box::new(trusted_foreign_fields));
        TrustedForeign { ptr }
    }
}

impl Drop for TrustedForeign {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.ptr));
        }
    }
}

impl TorqueGeneratedTrustedForeign<TrustedForeign, TrustedObjectImpl> for TrustedForeign {}
impl TrustedObject for TrustedForeign {}

impl TrustedForeign {
    DECL_PRINTER!(TrustedForeign)
}

TQ_OBJECT_CONSTRUCTORS!(TrustedForeign);

#[derive(Debug, Copy, Clone)]
pub struct HeapObjectImpl {}

impl HeapObjectImpl {
    pub fn new() -> Self {
        HeapObjectImpl {}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TrustedObjectImpl {}

impl TrustedObjectImpl {
    pub fn new() -> Self {
        TrustedObjectImpl {}
    }
}
}
