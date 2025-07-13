// Converted from V8 C++ source files:
// Header: arguments-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod arguments_inl {
    use crate::execution::isolate_inl::IsolateForSandbox;
    use crate::objects::arguments::*;
    use crate::objects::contexts_inl::*;
    use crate::objects::fixed_array_inl::*;
    use crate::objects::objects_inl::*;
    use crate::objects::name::WriteBarrierMode;
    use crate::objects::feedback_vector::RelaxedLoadTag;
    use crate::objects::free_space::RelaxedStoreTag;
    use crate::objects::string::v8;
    use std::marker::PhantomData;

    macro_rules! TQ_OBJECT_CONSTRUCTORS_IMPL {
        ($name:ident) => {
            impl $name {
                pub fn new() -> Self {
                    Self { dummy: 0, phantom: PhantomData }
                }
            }
        };
    }

    TQ_OBJECT_CONSTRUCTORS_IMPL!(JSArgumentsObject);
    TQ_OBJECT_CONSTRUCTORS_IMPL!(AliasedArgumentsEntry);

    impl SloppyArgumentsElements {
        pub fn context(&self) -> Tagged<Context> {
            self.context_
        }
        pub fn set_context(&mut self, value: Tagged<Context>, mode: WriteBarrierMode) {
            self.context_ = value;
        }
        pub fn arguments(&self) -> Tagged<UnionOf<FixedArray, NumberDictionary>> {
            self.arguments_
        }
        pub fn set_arguments(
            &mut self,
            value: Tagged<UnionOf<FixedArray, NumberDictionary>>,
            mode: WriteBarrierMode,
        ) {
            self.arguments_ = value;
        }

        pub fn mapped_entries(
            &self,
            index: i32,
            tag: RelaxedLoadTag,
        ) -> Tagged<UnionOf<Smi, Hole>> {
            assert!(index as u32  < self.length() as u32);
            self.objects_[index as usize]
        }

        pub fn set_mapped_entries(&mut self, index: i32, value: Tagged<UnionOf<Smi, Hole>>) {
             assert!(index as u32  < self.length() as u32);
            self.objects_[index as usize] = value;
        }

        pub fn set_mapped_entries_relaxed(
            &mut self,
            index: i32,
            value: Tagged<UnionOf<Smi, Hole>>,
            tag: RelaxedStoreTag,
        ) {
            assert!(index as u32  < self.length() as u32);
            self.objects_[index as usize] = value;
        }
    }
}
