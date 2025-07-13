// Converted from V8 C++ source files:
// Header: tnode.h
// Implementation: tnode.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use std::marker::PhantomData;

use crate::v8::internal::Object;
use crate::v8::internal::Smi;

pub struct UntaggedT {}

pub struct IntegralT : UntaggedT {}

pub struct WordT : IntegralT {
    pub const kMachineRepresentation: MachineRepresentation =
        MachineType::PointerRepresentation();
}

pub struct RawPtrT : WordT {
    pub const kMachineType: MachineType = MachineType::Pointer();
}

// A RawPtrT that is guaranteed to point into the sandbox.
pub struct SandboxedPtrT : WordT {
    pub const kMachineType: MachineType = MachineType::SandboxedPointer();
}

#[derive(Clone, Copy)]
pub struct RawPtr<To> {
    _phantom: PhantomData<To>,
}

impl<To> RawPtr<To> {
  pub fn new() -> Self {
    RawPtr{_phantom: PhantomData}
  }
}

pub struct Word32T : IntegralT {
    pub const kMachineRepresentation: MachineRepresentation =
        MachineRepresentation::kWord32;
}

pub struct Int32T : Word32T {
    pub const kMachineType: MachineType = MachineType::Int32();
}

pub struct Uint32T : Word32T {
    pub const kMachineType: MachineType = MachineType::Uint32();
}

pub struct Int16T : Int32T {
    pub const kMachineType: MachineType = MachineType::Int16();
}

pub struct Uint16T : Uint32T {
  
}
impl Uint16T {
    pub const kMachineType: MachineType = MachineType::Uint16();
}
pub struct Int8T : Int16T {
    pub const kMachineType: MachineType = MachineType::Int8();
}

pub struct Uint8T : Uint16T {
  
}

impl Uint8T {
    pub const kMachineType: MachineType = MachineType::Uint8();
}

pub struct Word64T : IntegralT {
    pub const kMachineRepresentation: MachineRepresentation =
        MachineRepresentation::kWord64;
}

pub struct AdditiveSafeIntegerT : Word64T {
    pub const kMachineType: MachineType = MachineType::Int64();
}

pub struct Int64T : Word64T {
    pub const kMachineType: MachineType = MachineType::Int64();
}

pub struct Uint64T : Word64T {
    pub const kMachineType: MachineType = MachineType::Uint64();
}

pub struct IntPtrT : WordT {
    pub const kMachineType: MachineType = MachineType::IntPtr();
}

pub struct UintPtrT : WordT {
    pub const kMachineType: MachineType = MachineType::UintPtr();
}

pub struct ExternalPointerHandleT : Uint32T {
    pub const kMachineType: MachineType = MachineType::Uint32();
}

pub struct CppHeapPointerHandleT : Uint32T {
    pub const kMachineType: MachineType = MachineType::Uint32();
}

pub struct IndirectPointerHandleT : Uint32T {
    pub const kMachineType: MachineType = MachineType::Uint32();
}

pub struct JSDispatchHandleT : Uint32T {
    pub const kMachineType: MachineType = MachineType::Uint32();
}

#[cfg(feature = "V8_ENABLE_SANDBOX")]
pub struct ExternalPointerT : Uint32T {
    pub const kMachineType: MachineType = MachineType::Uint32();
}

#[cfg(not(feature = "V8_ENABLE_SANDBOX"))]
pub struct ExternalPointerT : UntaggedT {
    pub const kMachineType: MachineType = MachineType::Pointer();
}

#[cfg(feature = "V8_COMPRESS_POINTERS")]
pub struct CppHeapPointerT : Uint32T {
    pub const kMachineType: MachineType = MachineType::Uint32();
}

#[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
pub struct CppHeapPointerT : UntaggedT {
    pub const kMachineType: MachineType = MachineType::Pointer();
}

pub struct Float16RawBitsT : Word32T {
    pub const kMachineType: MachineType = MachineType::Uint16();
}

pub struct Float32T : UntaggedT {
    pub const kMachineRepresentation: MachineRepresentation =
        MachineRepresentation::kFloat32;
    pub const kMachineType: MachineType = MachineType::Float32();
}

pub struct Float64T : UntaggedT {
    pub const kMachineRepresentation: MachineRepresentation =
        MachineRepresentation::kFloat64;
    pub const kMachineType: MachineType = MachineType::Float64();
}

#[cfg(feature = "V8_COMPRESS_POINTERS")]
pub type TaggedT = Int32T;

#[cfg(not(feature = "V8_COMPRESS_POINTERS"))]
pub type TaggedT = IntPtrT;

#[cfg(feature = "V8_ENABLE_SANDBOX")]
pub type TrustedPointerT = IndirectPointerHandleT;

#[cfg(not(feature = "V8_ENABLE_SANDBOX"))]
pub type TrustedPointerT = TaggedT;

// Result of a comparison operation.
pub struct BoolT : Word32T {
    pub const kMachineType: MachineType = MachineType::Int32();
}

// Value type of a Turbofan node with two results.
#[derive(Clone, Copy)]
pub struct PairT<T1, T2> {
    _phantom: PhantomData<(T1, T2)>,
}

impl<T1, T2> PairT<T1, T2> {
  pub fn new() -> Self {
    PairT{_phantom: PhantomData}
  }
}

pub struct Simd128T : UntaggedT {
    pub const kMachineRepresentation: MachineRepresentation =
        MachineRepresentation::kSimd128;
    pub const kMachineType: MachineType = MachineType::Simd128();
}

pub struct I8x16T : Simd128T {}
pub struct I16x8T : Simd128T {}
pub struct I32x2T : Simd128T {}

pub fn CommonMachineType(type1: MachineType, type2: MachineType) -> MachineType {
    if type1 == type2 {
        return type1;
    } else if type1.IsTagged() && type2.IsTagged() {
        return MachineType::AnyTagged();
    } else {
        return MachineType::None();
    }
}

pub struct MachineTypeOf<Type, Enable = ()> {
    _phantom: PhantomData<(Type, Enable)>,
}

impl<Type> MachineTypeOf<Type, ()> {
  pub const value: MachineType = Type::kMachineType;
}

pub struct MachineTypeOf_Object {}

impl MachineTypeOf_Object {
    pub const value: MachineType = MachineType::AnyTagged();
}

impl MachineTypeOf<Object> {
  pub const value: MachineType = MachineTypeOf_Object::value;
}

pub struct MachineTypeOf_MaybeObject {}

impl MachineTypeOf_MaybeObject {
  pub const value: MachineType = MachineType::AnyTagged();
}

impl MachineTypeOf<MaybeObject> {
  pub const value: MachineType = MachineTypeOf_MaybeObject::value;
}

pub struct MachineTypeOf_MaybeWeak_HeapObject {}

impl MachineTypeOf_MaybeWeak_HeapObject {
  pub const value: MachineType = MachineType::AnyTagged();
}
impl MachineTypeOf<MaybeWeak<HeapObject>> {
  pub const value: MachineType = MachineTypeOf_MaybeWeak_HeapObject::value;
}

pub struct MachineTypeOf_HeapObject {}

impl MachineTypeOf_HeapObject {
  pub const value: MachineType = MachineType::TaggedPointer();
}

impl MachineTypeOf<HeapObject> {
  pub const value: MachineType = MachineTypeOf_HeapObject::value;
}

pub struct MachineTypeOf_Smi {}
impl MachineTypeOf_Smi {
  pub const value: MachineType = MachineType::TaggedSigned();
}

impl MachineTypeOf<Smi> {
  pub const value: MachineTypeOf_Smi::value;
}

pub struct MachineTypeOf_TaggedIndex {}

impl MachineTypeOf_TaggedIndex {
  pub const value: MachineType = MachineType::Pointer();
}

impl MachineTypeOf<TaggedIndex> {
  pub const value: MachineTypeOf_TaggedIndex::value;
}

pub struct MachineTypeOf_HeapObjectSubtype<HeapObjectSubtype> {
    _phantom: PhantomData<HeapObjectSubtype>,
}

impl<HeapObjectSubtype> MachineTypeOf_HeapObjectSubtype<HeapObjectSubtype> {
    pub const value: MachineType = MachineType::TaggedPointer();
}

pub struct MachineTypeOf_ExternalReference {}

impl MachineTypeOf_ExternalReference {
  pub const value: MachineType = MachineType::Pointer();
}

impl MachineTypeOf<ExternalReference> {
  pub const value: MachineTypeOf_ExternalReference::value;
}

pub struct MachineTypeOf_Union<T> {
    _phantom: PhantomData<T>,
}

impl<T> MachineTypeOf_Union<T> {
  pub const value: MachineType = MachineTypeOf::<T>::value;
}

impl<T> MachineTypeOf<Union<T>> {
  pub const value: MachineTypeOf_Union::<T>::value;
}

pub struct MachineTypeOf_Union_Multiple<T, Ts> {
    _phantom: PhantomData<(T, Ts)>,
}

impl<T, Ts> MachineTypeOf_Union_Multiple<T, Ts> {
    pub const value: MachineType = CommonMachineType(
        MachineTypeOf::<T>::value,
        MachineTypeOf::<Union<Ts>>::value,
    );
}

impl<T, Ts> MachineTypeOf<Union<T, Ts>> {
  pub const value: MachineTypeOf_Union_Multiple::<T, Ts>::value;
}

pub struct MachineTypeOf_Union_HeapObject_TaggedIndex {}
impl MachineTypeOf_Union_HeapObject_TaggedIndex {
  pub const value: MachineType = MachineType::AnyTagged();
}
impl MachineTypeOf<Union<HeapObject, TaggedIndex>> {
  pub const value: MachineTypeOf_Union_HeapObject_TaggedIndex::value;
}

pub struct MachineRepresentationOf<Type, Enable = ()> {
    _phantom: PhantomData<(Type, Enable)>,
}

impl<Type> MachineRepresentationOf<Type, ()> {
  pub const value: MachineRepresentation = Type::kMachineRepresentation;
}

pub struct MachineRepresentationOf_kMachineType<T> {
  _phantom: PhantomData<T>,
}

impl<T> MachineRepresentationOf_kMachineType<T> {
  pub const value: MachineRepresentation =
    T::kMachineType.representation();
}

impl<T> MachineRepresentationOf<T, std::option::Option<()>> {
  pub const value: MachineRepresentation =
      MachineRepresentationOf_kMachineType::<T>::value;
}

pub struct MachineRepresentationOf_ExternalReference {}

impl MachineRepresentationOf_ExternalReference {
  pub const value: MachineRepresentation =
    RawPtrT::kMachineRepresentation;
}

impl MachineRepresentationOf<ExternalReference> {
  pub const value: MachineRepresentationOf_ExternalReference::value;
}

pub const fn IsMachineRepresentationOf<T>(r: MachineRepresentation) -> bool {
    MachineRepresentationOf::<T>::value == r
}

pub type PhiMachineRepresentationOf<T> = MachineRepresentation;

pub struct is_valid_type_tag<T> {
    _phantom: PhantomData<T>,
}

impl<T> is_valid_type_tag<T> {
    pub const value: bool = false;
    pub const is_tagged: bool = false;
}

pub struct is_valid_type_tag_PairT<T1, T2> {
    _phantom: PhantomData<(T1, T2)>,
}

impl<T1, T2> is_valid_type_tag_PairT<T1, T2> {
    pub const value: bool = false;
    pub const is_tagged: bool = false;
}

pub struct is_valid_type_tag_Union<T> {
    _phantom: PhantomData<T>,
}

impl<T> is_valid_type_tag_Union<T> {
  pub const value: bool = false;
  pub const is_tagged: bool = false;
}

pub type AnyTaggedT = Union<Object, MaybeObject>;
pub type ContextOrEmptyContext = Union<Context, Smi>;

// A pointer to a builtin function, used by Torque's function pointers.
pub type BuiltinPtr = Smi;

pub struct is_subtype<ExternalReference, RawPtrT> {}

impl is_subtype<ExternalReference, RawPtrT> {
  pub const value: bool = true;
}

pub struct is_subtype<IntPtrT, RawPtrT> {}

impl is_subtype<IntPtrT, RawPtrT> {
  pub const value: bool = true;
}

pub struct types_have_common_values<T, U> {
    _phantom: PhantomData<(T, U)>,
}

impl<T, U> types_have_common_values<T, U> {
  pub const value: bool = false;
}

pub struct TNode<T> {
  node_: *mut compiler::Node,
  _phantom: PhantomData<T>,
}

impl<T> TNode<T> {
  pub fn new(node_: *mut compiler::Node) -> Self {
    TNode {
      node_,
      _phantom: PhantomData,
    }
  }
}

impl<T> Clone for TNode<T> {
  fn clone(&self) -> Self {
    TNode {
      node_: self.node_,
      _phantom: PhantomData,
    }
  }
}

impl<T> Copy for TNode<T> {}

impl<T> TNode<T> {
  pub fn unchecked_cast(node: *mut compiler::Node) -> Self {
    TNode {
      node_: node,
      _phantom: PhantomData,
    }
  }
}

impl<T> TNode<T> {
  pub fn empty() -> Self {
      TNode{
          node_: std::ptr::null_mut(),
          _phantom: PhantomData,
      }
  }
}

impl<T> From<TNode<T>> for *mut compiler::Node {
    fn from(tnode: TNode<T>) -> Self {
        tnode.node_
    }
}

impl<T> From<&TNode<T>> for *mut compiler::Node {
    fn from(tnode: &TNode<T>) -> Self {
        tnode.node_
    }
}

impl<T> TNode<T> {
  fn LazyTemplateChecks(&self) {
      
  }
}

impl<T> From<*mut compiler::Node> for TNode<T> {
    fn from(node_: *mut compiler::Node) -> Self {
        TNode {
          node_: node_,
          _phantom: PhantomData,
        }
    }
}

impl<T> TNode<T> {
    fn operator_equals(&mut self, other: TNode<T>) -> Self {
      self.node_ = other.node_;
      return *self;
    }
}

impl<T> From<TNode<T>> for bool {
  fn from(val: TNode<T>) -> Self {
    return val.node_ != std::ptr::null_mut()
  }
}

pub struct TNode_Tagged<T> {
    _phantom: PhantomData<T>,
}

pub struct SloppyTNode<T> {

}

impl<T> SloppyTNode<T> {
    pub fn new(node: *mut compiler::Node) -> Self {
        SloppyTNode{}
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MachineRepresentation {
    kNone,
    kBit,
    kWord8,
    kWord16,
    kWord32,
    kWord64,
    kFloat32,
    kFloat64,
    kSimd128,
    kTaggedSigned,
    kTaggedPointer,
    kTagged,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MachineType {
    representation: MachineRepresentation,
}

impl MachineType {
    pub const fn Int32() -> Self {
        MachineType {
            representation: MachineRepresentation::kWord32,
        }
    }
    pub const fn Uint32() -> Self {
        MachineType {
            representation: MachineRepresentation::kWord32,
        }
    }
    pub const fn Int16() -> Self {
        MachineType {
            representation: MachineRepresentation::kWord16,
        }
    }
    pub const fn Uint16() -> Self {
        MachineType {
            representation: MachineRepresentation::kWord16,
        }
    }
    pub const fn Int8() -> Self {
        MachineType {
            representation: MachineRepresentation::kWord8,
        }
    }
    pub const fn Uint8() -> Self {
        MachineType {
            representation: MachineRepresentation::kWord8,
        }
    }
    pub const fn Int64() -> Self {
        MachineType {
            representation: MachineRepresentation::kWord64,
        }
    }
    pub const fn Uint64() -> Self {
        MachineType {
            representation: MachineRepresentation::kWord64,
        }
    }
    pub const fn IntPtr() -> Self {
        MachineType {
            representation: MachineRepresentation::kWord64,
        }
    }
    pub const fn UintPtr() -> Self {
        MachineType {
            representation: MachineRepresentation::kWord64,
        }
    }
    pub const fn Float32() -> Self {
        MachineType {
            representation: MachineRepresentation::kFloat32,
        }
    }
    pub const fn Float64() -> Self {
        MachineType {
            representation: MachineRepresentation::kFloat64,
        }
    }
    pub const fn Simd128() -> Self {
        MachineType {
            representation: MachineRepresentation::kSimd128,
        }
    }
    pub const fn AnyTagged() -> Self {
        MachineType {
            representation: MachineRepresentation::kTagged,
        }
    }
    pub const fn TaggedSigned() -> Self {
        MachineType {
            representation: MachineRepresentation::kTaggedSigned,
        }
    }
    pub const fn TaggedPointer() -> Self {
        MachineType {
            representation: MachineRepresentation::kTaggedPointer,
        }
    }
    pub const fn Pointer() -> Self {
        MachineType {
            representation: MachineRepresentation::kWord64,
        }
    }
    pub const fn SandboxedPointer() -> Self {
      MachineType {
        representation: MachineRepresentation::kWord64,
      }
    }
    pub const fn None() -> Self {
        MachineType {
            representation: MachineRepresentation::kNone,
        }
    }
    pub const fn Bit() -> Self {
      MachineType {
        representation: MachineRepresentation::kBit,
      }
    }

    pub const fn PointerRepresentation() -> MachineRepresentation {
        MachineRepresentation::kWord64
    }

    pub const fn representation(&self) -> MachineRepresentation {
        self.representation
    }

    pub const fn IsTagged(&self) -> bool {
        self.representation == MachineRepresentation::kTagged
            || self.representation == MachineRepresentation::kTaggedPointer
            || self.representation == MachineRepresentation::kTaggedSigned
    }
}

pub struct ExternalReference {}

pub struct Union<T, Ts = ()> {
    _phantom: PhantomData<(T, Ts)>,
}

pub struct MaybeObject {}
pub struct MaybeWeak<T> {
  _phantom: PhantomData<T>,
}
pub struct HeapObject {}
pub struct HeapObjectLayout {}

pub struct TaggedIndex {}

pub mod compiler {
  pub struct Node {}
}

pub mod std {
  pub mod option {
    pub struct Option<T> {
        _phantom: std::marker::PhantomData<T>,
    }
  }
}
