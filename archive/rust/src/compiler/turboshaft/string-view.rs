// Copyright 2024 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is a translation of the C++ header file:
// /home/kathirks_gc/v8_go/codebase/src/compiler/turboshaft/string-view.h

pub mod string_view {
    use crate::compiler::turboshaft::operations::*;

    // TODO: Define missing constants and types from the original C++ code.
    // const OFFSET_OF_DATA_START_SEQ_ONE_BYTE_STRING: usize = ...;
    // const OFFSET_OF_DATA_START_SEQ_TWO_BYTE_STRING: usize = ...;
    // const K_HEAP_OBJECT_TAG: usize = ...;

    /// `StringView` implements the `ForeachIterable` concept for iterating the
    /// characters of a string.
    pub struct StringView<'a> {
        string_: V<String>,
        encoding_: StringEncoding,
        start_index_: ConstOrV<WordPtr>,
        character_count_: ConstOrV<WordPtr>,
        end_offset_: Option<V<WordPtr>>,
        can_rely_on_no_gc_: Option<&'a DisallowGarbageCollection>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum StringEncoding {
        ONE_BYTE_ENCODING,
        TWO_BYTE_ENCODING,
    }

    impl<'a> StringView<'a> {
        pub type value_type = V<Word32>;
        pub type iterator_type = V<WordPtr>;

        pub fn new(
            can_rely_on_no_gc: &'a DisallowGarbageCollection,
            string: V<String>,
            encoding: StringEncoding,
            start_index: ConstOrV<WordPtr>,
            character_count: ConstOrV<WordPtr>,
        ) -> Self {
            StringView {
                string_: string,
                encoding_: encoding,
                start_index_: start_index,
                character_count_: character_count,
                end_offset_: None,
                can_rely_on_no_gc_: Some(can_rely_on_no_gc),
            }
        }

        pub fn new_without_gc(
            string: V<String>,
            encoding: StringEncoding,
            start_index: ConstOrV<WordPtr>,
            character_count: ConstOrV<WordPtr>,
        ) -> Self {
            StringView {
                string_: string,
                encoding_: encoding,
                start_index_: start_index,
                character_count_: character_count,
                end_offset_: None,
                can_rely_on_no_gc_: None,
            }
        }

        pub fn begin<A: AssemblerLike>(&mut self, assembler: &mut A) -> iterator_type {
            // static_assert!(OFFSET_OF_DATA_START_SEQ_ONE_BYTE_STRING == OFFSET_OF_DATA_START_SEQ_TWO_BYTE_STRING);

            // TODO: Retrieve actual values from defined constants once available.
            let data_offset: usize = 0; //OFFSET_OF_DATA_START_SEQ_ONE_BYTE_STRING;
            let k_heap_object_tag: usize = 0; //K_HEAP_OBJECT_TAG;

            let stride = match self.encoding_ {
                StringEncoding::ONE_BYTE_ENCODING => 1,
                StringEncoding::TWO_BYTE_ENCODING => 2,
            };

            if self.can_rely_on_no_gc_.is_none() {
                // TODO(nicohartmann): If we cannot rely on no GC happening during
                // iteration, we cannot operate on raw inner pointers but have to
                // recompute the character address from the base on each dereferencing.
                // unimplemented!();
                println!("UNIMPLEMENTED: GC might happen during iteration");
            }

            let begin_offset = assembler.word_ptr_add(
                assembler.bitcast_tagged_to_word_ptr(self.string_.clone()),
                assembler.word_ptr_add(
                    data_offset as i64 - k_heap_object_tag as i64,
                    assembler.word_ptr_mul(assembler.resolve(&self.start_index_), stride as i64),
                ),
            );

            let count: V<WordPtr>;
            if self.character_count_.is_constant() {
                count = assembler.resolve(&self.character_count_);
            } else if self.character_count_.value().is_valid() {
                count = self.character_count_.value();
            } else {
                // TODO(nicohartmann): Load from string.
                // unimplemented!();
                println!("UNIMPLEMENTED: Load from string");
                count = V::invalid(); // Placeholder
            }

            let end_offset =
                assembler.word_ptr_add(begin_offset.clone(), assembler.word_ptr_mul(count, stride as i64));
            self.end_offset_ = Some(end_offset);
            begin_offset
        }

        pub fn is_end<A: AssemblerLike>(
            &self,
            assembler: &mut A,
            current_iterator: iterator_type,
        ) -> OptionalV<Word32> {
            if let Some(end_offset) = &self.end_offset_ {
                assembler.uint_ptr_less_than_or_equal(end_offset.clone(), current_iterator)
            } else {
                OptionalV::invalid() // Should not happen if begin was called first
            }
        }

        pub fn advance<A: AssemblerLike>(
            &self,
            assembler: &mut A,
            current_iterator: iterator_type,
        ) -> iterator_type {
            let stride = match self.encoding_ {
                StringEncoding::ONE_BYTE_ENCODING => 1,
                StringEncoding::TWO_BYTE_ENCODING => 2,
            };
            assembler.word_ptr_add(current_iterator, stride as i64)
        }

        pub fn dereference<A: AssemblerLike>(
            &self,
            assembler: &mut A,
            current_iterator: iterator_type,
        ) -> value_type {
            let loaded_rep = match self.encoding_ {
                StringEncoding::ONE_BYTE_ENCODING => MemoryRepresentation::Uint8,
                StringEncoding::TWO_BYTE_ENCODING => MemoryRepresentation::Uint16,
            };
            assembler.load(
                current_iterator,
                LoadOpKind::RawAligned,
                loaded_rep,
            )
        }
    }

    // Mock implementations of required structs and enums, if not already available.
    pub trait AssemblerLike {
        fn bitcast_tagged_to_word_ptr(&mut self, value: V<String>) -> V<WordPtr>;
        fn word_ptr_add(&mut self, a: i64, b: i64) -> V<WordPtr>;
        fn word_ptr_add(&mut self, a: V<WordPtr>, b: i64) -> V<WordPtr>;
        fn word_ptr_add(&mut self, a: V<WordPtr>, b: V<WordPtr>) -> V<WordPtr>;
        fn word_ptr_mul(&mut self, a: V<WordPtr>, b: i64) -> i64;
        fn resolve(&mut self, const_or_v: &ConstOrV<WordPtr>) -> V<WordPtr>;
        fn uint_ptr_less_than_or_equal(&mut self, a: V<WordPtr>, b: V<WordPtr>) -> OptionalV<Word32>;
        fn load(&mut self, ptr: V<WordPtr>, kind: LoadOpKind, rep: MemoryRepresentation) -> V<Word32>;
    }

    pub struct DisallowGarbageCollection {}

    #[derive(Debug, Clone)]
    pub struct V<T> {
       _phantom: std::marker::PhantomData<T>,
       valid: bool,
    }

    impl<T> V<T> {
        pub fn valid(&self) -> bool {
            self.valid
        }
        pub fn invalid() -> Self {
            V{_phantom: std::marker::PhantomData, valid: false}
        }
    }

    pub struct String {}

    #[derive(Clone, Debug)]
    pub struct WordPtr {}

    #[derive(Clone, Debug)]
    pub struct Word32 {}

    #[derive(Clone, Debug)]
    pub enum MemoryRepresentation {
        Uint8,
        Uint16,
    }

    #[derive(Clone, Debug)]
    pub enum LoadOpKind {
        RawAligned,
    }

    #[derive(Clone, Debug)]
    pub enum ConstOrV<T> {
        Const(T),
        V(V<T>),
    }

    impl<T> ConstOrV<T> {
        pub fn is_constant(&self) -> bool {
            match self {
                ConstOrV::Const(_) => true,
                _ => false,
            }
        }

        pub fn value(&self) -> V<T> where T: Clone {
          match self {
            ConstOrV::V(v) => v.clone(),
            _ => V::invalid(),
          }
        }
    }

    impl ConstOrV<WordPtr> {
        pub fn new(value: WordPtr) -> Self {
            ConstOrV::Const(value)
        }
    }

    impl ConstOrV<WordPtr> {
      pub fn invalid() -> Self {
          ConstOrV::V(V::invalid())
      }
    }

    #[derive(Clone, Debug)]
    pub struct OptionalV<T> {
      _phantom: std::marker::PhantomData<T>
    }

    impl<T> OptionalV<T> {
      pub fn invalid() -> Self {
        OptionalV{_phantom: std::marker::PhantomData}
      }
    }
}

pub mod compiler {
  pub mod turboshaft {
    pub mod operations {
      pub use crate::string_view::*;
    }
  }
}