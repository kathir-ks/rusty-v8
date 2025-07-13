// Converted from V8 C++ source files:
// Header: js-array-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
use std::marker::PhantomData;

use crate::v8::internal::{
    Address, Isolate, IsolateForSandbox, Managed, Map, Number, Object, OpIndex, RegisterT, Smi,
    Tagged, TaggedField, WriteBarrierMode, InstructionOperand,
};

struct JSArray {
    dummy: i32,
    phantom: PhantomData<()>,
}

struct JSArrayIterator {
    dummy: i32,
    phantom: PhantomData<()>,
}

struct TemplateLiteralObject {
    dummy: i32,
    phantom: PhantomData<()>,
}

impl JSArray {
    fn new() -> Self {
        Self {
            dummy: 0,
            phantom: PhantomData,
        }
    }
}

impl JSArrayIterator {
    fn new() -> Self {
        Self {
            dummy: 0,
            phantom: PhantomData,
        }
    }
}

impl TemplateLiteralObject {
    fn new() -> Self {
        Self {
            dummy: 0,
            phantom: PhantomData,
        }
    }
}

const kLengthOffset: usize = 0;
const kKindOffset: usize = 0;
const kMaxFastArrayLength: u32 = 4294967295;
const SKIP_WRITE_BARRIER: WriteBarrierMode = WriteBarrierMode {};
const ALLOW_COPIED_DOUBLE_ELEMENTS: bool = true;

impl JSArray {
    fn length(&self, cage_base: PtrComprCageBase) -> Tagged<Number> {
        TaggedField::<Number, kLengthOffset>::load(cage_base, *self)
    }

    fn set_length(&self, value: Tagged<Number>, mode: WriteBarrierMode) {
        TaggedField::<Number, kLengthOffset>::Relaxed_Store(*self, value);
        conditional_write_barrier(*self, kLengthOffset, value, mode);
    }

    fn length_relaxed(&self, cage_base: PtrComprCageBase, tag: RelaxedLoadTag) -> Tagged<Number> {
        TaggedField::<Number, kLengthOffset>::Relaxed_Load(cage_base, *self)
    }

    fn set_length_smi(&self, length: Tagged<Smi>) {
        self.set_length(Tagged::<Number>(length), SKIP_WRITE_BARRIER);
    }

    fn set_elements(&self, _elements: FixedArrayBase) {}
    fn GetElementsKind(&self) -> ElementsKind {
        ElementsKind::kSmiElements
    }
    fn map(&self) -> Tagged<Map> {
        todo!()
    }
}

impl JSArray {
    fn SetLengthWouldNormalize(_heap: *mut Heap, new_length: u32) -> bool {
        new_length > kMaxFastArrayLength
    }

    fn SetContent(array: DirectHandle<JSArray>, storage: DirectHandle<FixedArrayBase>) {
        Self::EnsureCanContainElements(
            array,
            storage,
            storage.get().length(),
            ALLOW_COPIED_DOUBLE_ELEMENTS,
        );
        array.get().set_elements(*storage.get());
        array.get().set_length_smi(Smi::FromInt(storage.get().length() as i32));
    }

    fn HasArrayPrototype(_isolate: *mut Isolate) -> bool {
        true
    }

    fn EnsureCanContainElements(
        _array: DirectHandle<JSArray>,
        _storage: DirectHandle<FixedArrayBase>,
        _length: i32,
        _allow_copied_double_elements: bool,
    ) {
    }
}

impl JSArrayIterator {
    fn raw_kind(&self) -> i32 {
        0
    }

    fn set_raw_kind(&self, kind: i32) {}

    fn kind(&self) -> IterationKind {
        IterationKind::KEY
    }

    fn set_kind(&self, kind: IterationKind) {
        self.set_raw_kind(kind as i32);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum ElementsKind {
    kSmiElements,
    kObjectElements,
    kDoubleElements,
}

struct Heap {}
struct FixedArrayBase {
    length_: i32,
    map_: Tagged<Map>,
}
impl FixedArrayBase {
    fn length(&self) -> i32 {
        self.length_
    }

    fn map(&self) -> Tagged<Map> {
        self.map_
    }
}
struct FixedArray {}
impl FixedArray {
    fn length(&self) -> i32 {
        0
    }
    fn get(&self, _i: i32) -> Tagged<Object> {
        todo!()
    }
}
struct DirectHandle<T> {
    obj: T,
}

impl<T> DirectHandle<T> {
    fn get(&self) -> &T {
        &self.obj
    }
}

struct PtrComprCageBase {}

#[derive(Clone, Copy)]
struct RelaxedLoadTag {}

fn conditional_write_barrier(
    _object: JSArray,
    _offset: usize,
    _value: Tagged<Number>,
    _mode: WriteBarrierMode,
) {
}

impl Smi {
    fn FromInt(value: i32) -> Self {
        Smi {
            value_: value,
            phantom: PhantomData,
        }
    }
}
#[derive(Clone, Copy)]
struct Smi {
    value_: i32,
    phantom: PhantomData<()>,
}

impl Tagged<Smi> {
    fn value(&self) -> i32 {
        self.ptr as i32
    }
}

impl Tagged<Number> {
    fn value(&self) -> f64 {
        self.ptr as f64
    }
}

impl Heap {
    fn read_only_roots(&mut self) -> ReadOnlyRoots {
        ReadOnlyRoots {}
    }
}
