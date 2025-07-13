// Converted from V8 C++ source files:
// Header: property-descriptor-object.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod property_descriptor_object {
  pub use crate::objects::structs::StructBodyDescriptor;
  use crate::objects::heap_object::PropertyDescriptorObject as BasePropertyDescriptorObject;
  use crate::objects::object_macros::DEFINE_TORQUE_GENERATED_PROPERTY_DESCRIPTOR_OBJECT_FLAGS;
  use crate::objects::structs::Struct;
  use crate::torque_generated::bit_fields::*;
  use std::marker::PhantomData;

  pub struct PropertyDescriptorObject {
    pub base: BasePropertyDescriptorObject,
  }

  impl PropertyDescriptorObject {
    pub fn new() -> Self {
      Self {
        base: BasePropertyDescriptorObject {},
      }
    }

    pub fn define_torque_generated_property_descriptor_object_flags(&self) {
      DEFINE_TORQUE_GENERATED_PROPERTY_DESCRIPTOR_OBJECT_FLAGS();
    }

    pub const K_REGULAR_ACCESSOR_PROPERTY_BITS: i32 =
      HasEnumerableBit::K_MASK | HasConfigurableBit::K_MASK | HasGetBit::K_MASK | HasSetBit::K_MASK;

    pub const K_REGULAR_DATA_PROPERTY_BITS: i32 =
      HasEnumerableBit::K_MASK | HasConfigurableBit::K_MASK | HasWritableBit::K_MASK | HasValueBit::K_MASK;

    pub const K_HAS_MASK: i32 = HasEnumerableBit::K_MASK
      | HasConfigurableBit::K_MASK
      | HasWritableBit::K_MASK
      | HasValueBit::K_MASK
      | HasGetBit::K_MASK
      | HasSetBit::K_MASK;

    pub type BodyDescriptor = StructBodyDescriptor;

    pub fn tq_object_constructors() {}
  }

  pub mod structs {
    pub struct StructBodyDescriptor {}
  }

  pub mod object_macros {
    #[macro_export]
    macro_rules! DEFINE_TORQUE_GENERATED_PROPERTY_DESCRIPTOR_OBJECT_FLAGS {
      () => {};
    }
  }

  pub mod torque_generated {
    pub mod bit_fields {
      pub struct HasEnumerableBit {}
      impl HasEnumerableBit {
        pub const K_MASK: i32 = 1;
      }
      pub struct HasConfigurableBit {}
      impl HasConfigurableBit {
        pub const K_MASK: i32 = 2;
      }
      pub struct HasWritableBit {}
      impl HasWritableBit {
        pub const K_MASK: i32 = 4;
      }
      pub struct HasValueBit {}
      impl HasValueBit {
        pub const K_MASK: i32 = 8;
      }
      pub struct HasGetBit {}
      impl HasGetBit {
        pub const K_MASK: i32 = 16;
      }
      pub struct HasSetBit {}
      impl HasSetBit {
        pub const K_MASK: i32 = 32;
      }
    }
  }
}
