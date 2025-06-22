// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! `HeapStatistics` contains memory consumption and utilization statistics for a
//! cppgc heap.

/// `HeapStatistics` contains memory consumption and utilization statistics for a
/// cppgc heap.
#[derive(Default, Debug)]
pub struct HeapStatistics {
    /// Overall committed amount of memory for the heap.
    pub committed_size_bytes: usize,
    /// Resident amount of memory held by the heap.
    pub resident_size_bytes: usize,
    /// Amount of memory actually used on the heap.
    pub used_size_bytes: usize,
    /// Memory retained in the page pool, not used directly by the heap.
    pub pooled_memory_size_bytes: usize,
    /// Detail level of this HeapStatistics.
    pub detail_level: DetailLevel,
    /// Statistics for each of the spaces in the heap. Filled only when
    /// `detail_level` is `DetailLevel::kDetailed`.
    pub space_stats: Vec<SpaceStatistics>,
    /// Vector of `cppgc::GarbageCollected` type names.
    pub type_names: Vec<String>,
}

/// Specifies the detail level of the heap statistics. Brief statistics contain
/// only the top-level allocated and used memory statistics for the entire
/// heap. Detailed statistics also contain a break down per space and page, as
/// well as freelist statistics and object type histograms. Note that used
/// memory reported by brief statistics and detailed statistics might differ
/// slightly.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum DetailLevel {
    #[default]
    kBrief,
    kDetailed,
}

/// Object statistics for a single type.
#[derive(Default, Debug)]
pub struct ObjectStatsEntry {
    /// Number of allocated bytes.
    pub allocated_bytes: usize,
    /// Number of allocated objects.
    pub object_count: usize,
}

/// Page granularity statistics. For each page the statistics record the
/// allocated memory size and overall used memory size for the page.
#[derive(Default, Debug)]
pub struct PageStatistics {
    /// Overall committed amount of memory for the page.
    pub committed_size_bytes: usize,
    /// Resident amount of memory held by the page.
    pub resident_size_bytes: usize,
    /// Amount of memory actually used on the page.
    pub used_size_bytes: usize,
    /// Statistics for object allocated on the page. Filled only when
    /// NameProvider::SupportsCppClassNamesAsObjectNames() is true.
    pub object_statistics: Vec<ObjectStatsEntry>,
}

/// Statistics of the freelist (used only in non-large object spaces). For
/// each bucket in the freelist the statistics record the bucket size, the
/// number of freelist entries in the bucket, and the overall allocated memory
/// consumed by these freelist entries.
#[derive(Default, Debug)]
pub struct FreeListStatistics {
    /// bucket sizes in the freelist.
    pub bucket_size: Vec<usize>,
    /// number of freelist entries per bucket.
    pub free_count: Vec<usize>,
    /// memory size consumed by freelist entries per size.
    pub free_size: Vec<usize>,
}

/// Space granularity statistics. For each space the statistics record the
/// space name, the amount of allocated memory and overall used memory for the
/// space. The statistics also contain statistics for each of the space's
/// pages, its freelist and the objects allocated on the space.
#[derive(Default, Debug)]
pub struct SpaceStatistics {
    /// The space name
    pub name: String,
    /// Overall committed amount of memory for the heap.
    pub committed_size_bytes: usize,
    /// Resident amount of memory held by the heap.
    pub resident_size_bytes: usize,
    /// Amount of memory actually used on the space.
    pub used_size_bytes: usize,
    /// Statistics for each of the pages in the space.
    pub page_stats: Vec<PageStatistics>,
    /// Statistics for the freelist of the space.
    pub free_list_stats: FreeListStatistics,
}