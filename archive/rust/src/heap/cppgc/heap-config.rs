// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//use crate::base::platform::time::TimeTicks; // Assuming a suitable Rust equivalent exists

/// Internal namespace for cppgc.
pub mod internal {
    use cppgc::Heap;

    pub type StackState = Heap::StackState;

    /// Represents the type of garbage collection.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum CollectionType {
        /// Minor garbage collection.
        kMinor,
        /// Major garbage collection.
        kMajor,
    }

    /// Specifies how free memory should be handled.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum FreeMemoryHandling {
        /// Do not discard free memory.
        kDoNotDiscard,
        /// Discard free memory where possible.
        kDiscardWherePossible,
    }

    /// Configuration for the marking phase of garbage collection.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct MarkingConfig {
        pub collection_type: CollectionType,
        pub stack_state: StackState,
        pub marking_type: Heap::MarkingType,
        pub is_forced_gc: MarkingConfig::IsForcedGC,
    }

    impl MarkingConfig {
        pub type MarkingType = Heap::MarkingType;

        /// Specifies whether this GC is forced.
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(u8)]
        pub enum IsForcedGC {
            /// GC is not forced.
            kNotForced,
            /// GC is forced.
            kForced,
        }

        /// Returns the default marking configuration.
        pub const fn default() -> Self {
            MarkingConfig {
                collection_type: CollectionType::kMajor,
                stack_state: StackState::kMayContainHeapPointers,
                marking_type: Heap::MarkingType::kIncremental,
                is_forced_gc: IsForcedGC::kNotForced,
            }
        }
    }

    /// Configuration for the sweeping phase of garbage collection.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct SweepingConfig {
        pub sweeping_type: Heap::SweepingType,
        pub compactable_space_handling: SweepingConfig::CompactableSpaceHandling,
        pub free_memory_handling: FreeMemoryHandling,
    }

    impl SweepingConfig {
        pub type SweepingType = Heap::SweepingType;

        /// Specifies how compactable spaces are handled during sweeping.
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum CompactableSpaceHandling {
            /// Sweep compactable spaces.
            kSweep,
            /// Ignore compactable spaces.
            kIgnore,
        }
    }

    /// Configuration for a garbage collection cycle.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct GCConfig {
        pub collection_type: CollectionType,
        pub stack_state: StackState,
        pub marking_type: MarkingConfig::MarkingType,
        pub sweeping_type: SweepingConfig::SweepingType,
        pub free_memory_handling: FreeMemoryHandling,
        pub is_forced_gc: MarkingConfig::IsForcedGC,
    }

    impl GCConfig {
        pub type MarkingType = MarkingConfig::MarkingType;
        pub type SweepingType = SweepingConfig::SweepingType;
        pub type FreeMemoryHandling = FreeMemoryHandling;
        pub type IsForcedGC = MarkingConfig::IsForcedGC;

        /// Returns a conservative atomic garbage collection configuration.
        pub const fn conservative_atomic_config() -> Self {
            GCConfig {
                collection_type: CollectionType::kMajor,
                stack_state: StackState::kMayContainHeapPointers,
                marking_type: Heap::MarkingType::kAtomic,
                sweeping_type: Heap::SweepingType::kAtomic,
                free_memory_handling: FreeMemoryHandling::kDoNotDiscard,
                is_forced_gc: IsForcedGC::kNotForced,
            }
        }

        /// Returns a precise atomic garbage collection configuration.
        pub const fn precise_atomic_config() -> Self {
            GCConfig {
                collection_type: CollectionType::kMajor,
                stack_state: StackState::kNoHeapPointers,
                marking_type: Heap::MarkingType::kAtomic,
                sweeping_type: Heap::SweepingType::kAtomic,
                free_memory_handling: FreeMemoryHandling::kDoNotDiscard,
                is_forced_gc: IsForcedGC::kNotForced,
            }
        }

        /// Returns a conservative incremental garbage collection configuration.
        pub const fn conservative_incremental_config() -> Self {
            GCConfig {
                collection_type: CollectionType::kMajor,
                stack_state: StackState::kMayContainHeapPointers,
                marking_type: Heap::MarkingType::kIncremental,
                sweeping_type: Heap::SweepingType::kAtomic,
                free_memory_handling: FreeMemoryHandling::kDoNotDiscard,
                is_forced_gc: IsForcedGC::kNotForced,
            }
        }

        /// Returns a precise incremental garbage collection configuration.
        pub const fn precise_incremental_config() -> Self {
            GCConfig {
                collection_type: CollectionType::kMajor,
                stack_state: StackState::kNoHeapPointers,
                marking_type: Heap::MarkingType::kIncremental,
                sweeping_type: Heap::SweepingType::kAtomic,
                free_memory_handling: FreeMemoryHandling::kDoNotDiscard,
                is_forced_gc: IsForcedGC::kNotForced,
            }
        }

        /// Returns a precise incremental marking concurrent sweeping garbage collection configuration.
        pub const fn precise_incremental_marking_concurrent_sweeping_config() -> Self {
            GCConfig {
                collection_type: CollectionType::kMajor,
                stack_state: StackState::kNoHeapPointers,
                marking_type: Heap::MarkingType::kIncremental,
                sweeping_type: Heap::SweepingType::kIncrementalAndConcurrent,
                free_memory_handling: FreeMemoryHandling::kDoNotDiscard,
                is_forced_gc: IsForcedGC::kNotForced,
            }
        }

        /// Returns a precise concurrent garbage collection configuration.
        pub const fn precise_concurrent_config() -> Self {
            GCConfig {
                collection_type: CollectionType::kMajor,
                stack_state: StackState::kNoHeapPointers,
                marking_type: Heap::MarkingType::kIncrementalAndConcurrent,
                sweeping_type: Heap::SweepingType::kIncrementalAndConcurrent,
                free_memory_handling: FreeMemoryHandling::kDoNotDiscard,
                is_forced_gc: IsForcedGC::kNotForced,
            }
        }

        /// Returns a minor precise atomic garbage collection configuration.
        pub const fn minor_precise_atomic_config() -> Self {
            GCConfig {
                collection_type: CollectionType::kMinor,
                stack_state: StackState::kNoHeapPointers,
                marking_type: Heap::MarkingType::kAtomic,
                sweeping_type: Heap::SweepingType::kAtomic,
                free_memory_handling: FreeMemoryHandling::kDoNotDiscard,
                is_forced_gc: IsForcedGC::kNotForced,
            }
        }

        /// Returns a minor conservative atomic garbage collection configuration.
        pub const fn minor_conservative_atomic_config() -> Self {
            GCConfig {
                collection_type: CollectionType::kMinor,
                stack_state: StackState::kMayContainHeapPointers,
                marking_type: Heap::MarkingType::kAtomic,
                sweeping_type: Heap::SweepingType::kAtomic,
                free_memory_handling: FreeMemoryHandling::kDoNotDiscard,
                is_forced_gc: IsForcedGC::kNotForced,
            }
        }
    }
}