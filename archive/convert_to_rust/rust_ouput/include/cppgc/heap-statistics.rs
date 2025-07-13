// Converted from V8 C++ source files:
// Header: heap-statistics.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub struct HeapStatistics {
    pub committed_size_bytes: usize,
    pub resident_size_bytes: usize,
    pub used_size_bytes: usize,
    pub pooled_memory_size_bytes: usize,
    pub detail_level: DetailLevel,
    pub space_stats: Vec<SpaceStatistics>,
    pub type_names: Vec<String>,
}

pub enum DetailLevel {
    kBrief,
    kDetailed,
}

impl HeapStatistics {
    pub fn new() -> Self {
        HeapStatistics {
            committed_size_bytes: 0,
            resident_size_bytes: 0,
            used_size_bytes: 0,
            pooled_memory_size_bytes: 0,
            detail_level: DetailLevel::kBrief,
            space_stats: Vec::new(),
            type_names: Vec::new(),
        }
    }
}

pub struct ObjectStatsEntry {
    pub allocated_bytes: usize,
    pub object_count: usize,
}

pub struct PageStatistics {
    pub committed_size_bytes: usize,
    pub resident_size_bytes: usize,
    pub used_size_bytes: usize,
    pub object_statistics: Vec<ObjectStatsEntry>,
}

impl PageStatistics {
    pub fn new() -> Self {
        PageStatistics {
            committed_size_bytes: 0,
            resident_size_bytes: 0,
            used_size_bytes: 0,
            object_statistics: Vec::new(),
        }
    }
}

pub struct FreeListStatistics {
    pub bucket_size: Vec<usize>,
    pub free_count: Vec<usize>,
    pub free_size: Vec<usize>,
}

impl FreeListStatistics {
    pub fn new() -> Self {
        FreeListStatistics {
            bucket_size: Vec::new(),
            free_count: Vec::new(),
            free_size: Vec::new(),
        }
    }
}

pub struct SpaceStatistics {
    pub name: String,
    pub committed_size_bytes: usize,
    pub resident_size_bytes: usize,
    pub used_size_bytes: usize,
    pub page_stats: Vec<PageStatistics>,
    pub free_list_stats: FreeListStatistics,
}

impl SpaceStatistics {
    pub fn new() -> Self {
        SpaceStatistics {
            name: String::new(),
            committed_size_bytes: 0,
            resident_size_bytes: 0,
            used_size_bytes: 0,
            page_stats: Vec::new(),
            free_list_stats: FreeListStatistics::new(),
        }
    }
}
