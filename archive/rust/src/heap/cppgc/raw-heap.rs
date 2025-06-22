// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod raw_heap {
    use std::vec::Vec;
    use std::boxed::Box;
    use std::iter::Iterator;
    use std::ops::Add;

    /// Opaque type representing a custom space index.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CustomSpaceIndex {
        pub value: usize,
    }

    impl CustomSpaceIndex {
        pub fn new(value: usize) -> Self {
            CustomSpaceIndex { value }
        }
    }

    pub trait CustomSpaceBase {}

    pub trait Heap {
        // Define the methods that a Heap must implement.
    }

    pub trait Space {}

    /// RawHeap is responsible for space management.
    pub struct RawHeap<'a, H: Heap, S: Space, CS: CustomSpaceBase> {
        main_heap_: &'a H,
        spaces_: Vec<Box<dyn Space>>,
        _phantom: std::marker::PhantomData<CS>,
    }

    /// Normal spaces are used to store objects of different size classes:
    /// - kNormal1:  < 32 bytes
    /// - kNormal2:  < 64 bytes
    /// - kNormal3:  < 128 bytes
    /// - kNormal4: >= 128 bytes
    ///
    /// Objects of size greater than 2^16 get stored in the large space.
    ///
    /// Users can override where objects are allocated via `cppgc::CustomSpace` to
    /// force allocation in a custom space.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum RegularSpaceType {
        kNormal1,
        kNormal2,
        kNormal3,
        kNormal4,
        kLarge,
    }

    impl RegularSpaceType {
        pub fn as_usize(self) -> usize {
            match self {
                RegularSpaceType::kNormal1 => 0,
                RegularSpaceType::kNormal2 => 1,
                RegularSpaceType::kNormal3 => 2,
                RegularSpaceType::kNormal4 => 3,
                RegularSpaceType::kLarge => 4,
            }
        }
    }

    pub const NUMBER_OF_REGULAR_SPACES: usize = RegularSpaceType::kLarge.as_usize() + 1;

    impl<'a, H: Heap, S: Space, CS: CustomSpaceBase> RawHeap<'a, H, S, CS> {
        pub fn new(heap: &'a H, custom_spaces: &Vec<Box<dyn CustomSpaceBase>>) -> Self {
            let mut spaces: Vec<Box<dyn Space>> = Vec::new();
            // Initialize regular spaces
            for _ in 0..NUMBER_OF_REGULAR_SPACES {
                // Placeholder for BaseSpace construction
                // Need BaseSpace struct to implement Space trait
                // spaces.push(Box::new(BaseSpace::new()));
                todo!("Implement BaseSpace and its construction");
            }

            // Initialize custom spaces
            for _ in custom_spaces {
                // Placeholder for BaseSpace construction
                // Need BaseSpace struct to implement Space trait
                // spaces.push(Box::new(BaseSpace::new()));
                todo!("Implement BaseSpace and its construction");
            }

            RawHeap {
                main_heap_: heap,
                spaces_: spaces,
                _phantom: std::marker::PhantomData,
            }
        }

        /// Space iteration support.
        pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn Space>> {
            self.spaces_.iter()
        }

        pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Box<dyn Space>> {
            self.spaces_.iter_mut()
        }

        pub fn custom_iter(&self) -> std::slice::Iter<'_, Box<dyn Space>> {
            self.spaces_[NUMBER_OF_REGULAR_SPACES..].iter()
        }

        pub fn custom_iter_mut(&mut self) -> std::slice::IterMut<'_, Box<dyn Space>> {
            self.spaces_[NUMBER_OF_REGULAR_SPACES..].iter_mut()
        }

        pub fn size(&self) -> usize {
            self.spaces_.len()
        }

        pub fn space(&self, space_type: RegularSpaceType) -> Option<&dyn Space> {
             if space_type.as_usize() < NUMBER_OF_REGULAR_SPACES {
                self.spaces_.get(space_type.as_usize()).map(|s| s.as_ref())
            } else {
                None
            }
        }

        pub fn space_mut(&mut self, space_type: RegularSpaceType) -> Option<&mut dyn Space> {
            if space_type.as_usize() < NUMBER_OF_REGULAR_SPACES {
                self.spaces_.get_mut(space_type.as_usize()).map(|s| s.as_mut())
            } else {
                None
            }
        }

        pub fn custom_space(&self, space_index: CustomSpaceIndex) -> Option<&dyn Space> {
             let index = self.space_index_for_custom_space(space_index);
            self.spaces_.get(index).map(|s| s.as_ref())
        }

        pub fn custom_space_mut(&mut self, space_index: CustomSpaceIndex) -> Option<&mut dyn Space> {
            let index = self.space_index_for_custom_space(space_index);
            self.spaces_.get_mut(index).map(|s| s.as_mut())
        }

        pub fn heap(&self) -> &H {
            self.main_heap_
        }

        fn space_index_for_custom_space(&self, space_index: CustomSpaceIndex) -> usize {
            assert!(space_index.value < self.spaces_.len() - NUMBER_OF_REGULAR_SPACES);
            NUMBER_OF_REGULAR_SPACES + space_index.value
        }

        fn get_space(&self, space_index: usize) -> Option<&dyn Space> {
            assert!(space_index < self.spaces_.len());
            self.spaces_.get(space_index).map(|s| s.as_ref())
        }

        fn get_space_mut(&mut self, space_index: usize) -> Option<&mut dyn Space> {
             assert!(space_index < self.spaces_.len());
            self.spaces_.get_mut(space_index).map(|s| s.as_mut())
        }
    }
}