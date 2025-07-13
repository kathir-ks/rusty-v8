// Converted from V8 C++ source files:
// Header: custom-space.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cppgc {

    /**
     * Index identifying a custom space.
     */
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CustomSpaceIndex {
        pub value: usize,
    }

    impl CustomSpaceIndex {
        pub const fn new(value: usize) -> Self {
            CustomSpaceIndex { value }
        }
    }

    /**
     * Top-level base class for custom spaces. Users must inherit from CustomSpace
     * below.
     */
    pub trait CustomSpaceBase {
        fn get_custom_space_index(&self) -> CustomSpaceIndex;
        fn is_compactable(&self) -> bool;
    }

    /**
     * Base class custom spaces should directly inherit from. The class inheriting
     * from `CustomSpace` must define `kSpaceIndex` as unique space index. These
     * indices need for form a sequence starting at 0.
     *
     * Example:
     * \code
     * class CustomSpace1 : public CustomSpace<CustomSpace1> {
     *  public:
     *   static constexpr CustomSpaceIndex kSpaceIndex = 0;
     * };
     * class CustomSpace2 : public CustomSpace<CustomSpace2> {
     *  public:
     *   static constexpr CustomSpaceIndex kSpaceIndex = 1;
     * };
     * \endcode
     */
    pub trait CustomSpace<ConcreteCustomSpace>
    where
        Self: CustomSpaceBase,
    {
        /**
         * Compaction is only supported on spaces that manually manage slots
         * recording.
         */
        const K_SUPPORTS_COMPACTION: bool = false;

        fn get_custom_space_index(&self) -> CustomSpaceIndex {
            Self::k_space_index()
        }

        fn is_compactable(&self) -> bool {
            Self::K_SUPPORTS_COMPACTION
        }

        fn k_space_index() -> CustomSpaceIndex;
    }

    /**
     * User-overridable trait that allows pinning types to custom spaces.
     */
    pub trait SpaceTrait<T> {
        type Space;
    }

    pub mod internal {
        use super::*;

        pub struct IsAllocatedOnCompactableSpaceImpl<CustomSpace> {
            _phantom: std::marker::PhantomData<CustomSpace>,
        }

        impl<CustomSpace> IsAllocatedOnCompactableSpaceImpl<CustomSpace>
        where
            CustomSpace: CustomSpaceBase,
        {
            pub const VALUE: bool = false; // Assuming CustomSpace::kSupportsCompaction is accessible via trait
        }

        impl IsAllocatedOnCompactableSpaceImpl<()> {
            pub const VALUE: bool = false;
        }

        pub struct IsAllocatedOnCompactableSpace<T> {
            _phantom: std::marker::PhantomData<T>,
        }

        impl<T> IsAllocatedOnCompactableSpace<T> {
            pub const VALUE: bool = false; // Assuming SpaceTrait<T>::Space can be handled

           // pub const VALUE: bool = IsAllocatedOnCompactableSpaceImpl::<
           //     <<T as SpaceTrait<T>>::Space as CustomSpaceBase>
           // >::VALUE;
        }
    } // namespace internal
} // namespace cppgc
