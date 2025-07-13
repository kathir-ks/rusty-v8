// Converted from V8 C++ source files:
// Header: cell-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_imports)]

use std::marker::PhantomData;

use crate::objects::js_objects::v8;

pub struct Cell {
    cage_base: *mut u8,
    value_offset: usize,
}

impl Cell {
    pub fn new(cage_base: *mut u8, value_offset: usize) -> Self {
        Cell {
            cage_base,
            value_offset,
        }
    }

    pub fn value(&self) -> TaggedObject {
        unsafe {
            let value_ptr = self.cage_base.add(self.value_offset) as *mut TaggedObject;
            *value_ptr
        }
    }

    pub fn set_value(&mut self, new_value: TaggedObject) {
        unsafe {
            let value_ptr = self.cage_base.add(self.value_offset) as *mut TaggedObject;
            *value_ptr = new_value;
        }
    }
}

#[derive(Clone, Copy)]
pub struct TaggedObject {
    ptr: *mut u8,
}

impl TaggedObject {
    pub fn new(ptr: *mut u8) -> Self {
        TaggedObject { ptr }
    }
}

trait RelaxedLoadable<T> {
    unsafe fn relaxed_load(cage_base: *mut u8, cell: &Cell) -> T;
}

impl RelaxedLoadable<TaggedObject> for TaggedObject {
    unsafe fn relaxed_load(cage_base: *mut u8, cell: &Cell) -> TaggedObject {
        let value_ptr = cage_base.add(cell.value_offset) as *mut TaggedObject;
        *value_ptr
    }
}

struct TaggedField<T, const OFFSET: usize>;

impl<T, const OFFSET: usize> TaggedField<T, OFFSET> {
    fn relaxed_load(cage_base: *mut u8, cell: &Cell) -> T
    where
        T: RelaxedLoadable<T>,
    {
        unsafe { T::relaxed_load(cage_base, cell) }
    }
}

macro_rules! DEF_RELAXED_GETTER {
    ($struct_name:ident, $field_name:ident, $field_type:ty) => {
        impl $struct_name {
            pub fn $field_name(&self) -> $field_type {
                 unsafe {
                    let value_ptr = self.cage_base.add(self.value_offset) as *mut $field_type;
                    *value_ptr
                }
            }
        }
    };
}

const kValueOffset: usize = 0;

impl Cell {
    pub fn value(&self) -> TaggedObject {
        TaggedField::<TaggedObject, kValueOffset>::relaxed_load(self.cage_base, self)
    }
}
