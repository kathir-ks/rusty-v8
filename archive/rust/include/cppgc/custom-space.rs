// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Index identifying a custom space.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomSpaceIndex {
    pub value: usize,
}

impl CustomSpaceIndex {
    pub const fn new(value: usize) -> Self {
        CustomSpaceIndex { value }
    }
}

/// Top-level base trait for custom spaces. Users must implement CustomSpaceTrait.
pub trait CustomSpaceBase {
    fn get_custom_space_index(&self) -> CustomSpaceIndex;
    fn is_compactable(&self) -> bool;
}

/// Base trait custom spaces should directly implement. The type implementing
/// `CustomSpaceTrait` must define `SPACE_INDEX` as unique space index. These
/// indices need to form a sequence starting at 0.
///
/// Example:
/// ```
/// struct CustomSpace1;
/// impl CustomSpaceTrait for CustomSpace1 {
///     const SPACE_INDEX: CustomSpaceIndex = CustomSpaceIndex::new(0);
/// }
/// struct CustomSpace2;
/// impl CustomSpaceTrait for CustomSpace2 {
///     const SPACE_INDEX: CustomSpaceIndex = CustomSpaceIndex::new(1);
/// }
/// ```
pub trait CustomSpaceTrait: CustomSpaceBase {
    /// Unique index for the custom space.  Must be a const.
    const SPACE_INDEX: CustomSpaceIndex;

    /// Compaction is only supported on spaces that manually manage slots recording.
    const SUPPORTS_COMPACTION: bool = false;
}

impl<T: CustomSpaceTrait> CustomSpaceBase for T {
    fn get_custom_space_index(&self) -> CustomSpaceIndex {
        T::SPACE_INDEX
    }
    fn is_compactable(&self) -> bool {
        T::SUPPORTS_COMPACTION
    }
}

/// User-overridable trait that allows pinning types to custom spaces.
pub trait SpaceTrait<T> {
    type Space;
}

/// Default implementation of `SpaceTrait` where no specific space is defined.
pub struct DefaultSpaceTrait<T>(std::marker::PhantomData<T>);

impl<T> SpaceTrait<T> for DefaultSpaceTrait<T> {
    type Space = Void;
}

/// Represents the absence of a custom space.
pub enum Void {}

pub mod internal {
    use super::*;

    pub struct IsAllocatedOnCompactableSpaceImpl<CustomSpace> {
        _phantom: std::marker::PhantomData<CustomSpace>,
    }

    impl<CustomSpace> IsAllocatedOnCompactableSpaceImpl<CustomSpace> {
        pub const VALUE: bool = <CustomSpace as CustomSpaceTrait>::SUPPORTS_COMPACTION;
    }

    impl IsAllocatedOnCompactableSpaceImpl<Void> {
        pub const VALUE: bool = false;
    }

    pub struct IsAllocatedOnCompactableSpace<T>(std::marker::PhantomData<T>);

    impl<T> IsAllocatedOnCompactableSpace<T> {
        pub const VALUE: bool =
            IsAllocatedOnCompactableSpaceImpl::<<DefaultSpaceTrait<T> as SpaceTrait<T>>::Space>::VALUE;
    }
}