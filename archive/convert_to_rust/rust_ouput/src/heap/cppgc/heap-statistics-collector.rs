// Converted from V8 C++ source files:
// Header: heap-statistics-collector.h
// Implementation: heap-statistics-collector.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/heap/cppgc/heap-statistics-collector.h
use std::collections::HashMap;

use crate::HeapStatistics;

pub mod internal {
    use std::collections::HashMap;
    use crate::HeapStatistics;
    use crate::HeapBase;
    use crate::NormalPageSpace;
    use crate::LargePageSpace;
    use crate::NormalPage;
    use crate::LargePage;
    use crate::HeapObjectHeader;
    use crate::NameProvider;
    use crate::StatsCollector;
    use crate::ClassNameAsHeapObjectNameScope;
    use crate::RawHeap;
    use crate::PageBackend;
    use crate::BasePage;
    use crate::LargePage;
    use crate::FreeList;
    use crate::PagePool;
    use crate::DCHECK_NE;
    use crate::DCHECK_EQ;
    use crate::DCHECK_NOT_NULL;
    use crate::RawHeap_kNumberOfRegularSpaces;
    use crate::RawHeap_kNumberOfRegularSpaces_minus_1;
    use crate::kPageSize;
    use crate::DCHECK_GE;

    pub struct HeapStatisticsCollector {
        type_name_to_index_map_: HashMap<*const std::ffi::c_void, usize>,
        current_stats_: *mut HeapStatistics,
        current_space_stats_: *mut HeapStatistics_SpaceStatistics,
        current_page_stats_: *mut HeapStatistics_PageStatistics,
    }

    #[allow(non_snake_case)]
    pub struct HeapStatistics_SpaceStatistics{
        pub name: String,
        pub committed_size_bytes: usize,
        pub resident_size_bytes: usize,
        pub used_size_bytes: usize,
        pub page_stats: Vec<HeapStatistics_PageStatistics>,
        pub free_list_stats: FreeList::FreeListStats,
    }

    impl HeapStatistics_SpaceStatistics{
        pub fn new() -> Self {
            HeapStatistics_SpaceStatistics{
                name: String::new(),
                committed_size_bytes: 0,
                resident_size_bytes: 0,
                used_size_bytes: 0,
                page_stats: Vec::new(),
                free_list_stats: FreeList::FreeListStats::new(),
            }
        }
    }

    #[allow(non_snake_case)]
    pub struct HeapStatistics_PageStatistics{
        pub committed_size_bytes: usize,
        pub resident_size_bytes: usize,
        pub used_size_bytes: usize,
        pub object_statistics: Vec<HeapStatistics::ObjectStatsEntry>,
    }

    impl HeapStatistics_PageStatistics{
        pub fn new() -> Self {
            HeapStatistics_PageStatistics{
                committed_size_bytes: 0,
                resident_size_bytes: 0,
                used_size_bytes: 0,
                object_statistics: Vec::new(),
            }
        }
    }

    impl HeapStatisticsCollector {
        pub fn new() -> Self {
            HeapStatisticsCollector {
                type_name_to_index_map_: HashMap::new(),
                current_stats_: std::ptr::null_mut(),
                current_space_stats_: std::ptr::null_mut(),
                current_page_stats_: std::ptr::null_mut(),
            }
        }

        pub fn CollectDetailedStatistics(&mut self, heap: *mut HeapBase) -> HeapStatistics {
            let mut stats = HeapStatistics {
                committed_size_bytes: 0,
                resident_size_bytes: 0,
                used_size_bytes: 0,
                pooled_memory_size_bytes: 0,
                space_stats: Vec::new(),
                type_names: Vec::new(),
                object_stats: Vec::new(),
                detail_level: HeapStatistics::DetailLevel::kDetailed,
            };

            self.current_stats_ = &mut stats;

            let heap_base = unsafe{ &mut *heap };

            let mut class_names_scope = ClassNameAsHeapObjectNameScope::new(heap_base);

            self.Traverse(unsafe{&mut *heap_base.raw_heap()});
            Self::FinalizeSpace(self.current_stats_, &mut self.current_space_stats_, &mut self.current_page_stats_);

            if NameProvider::SupportsCppClassNamesAsObjectNames() {
                stats.type_names.resize(self.type_name_to_index_map_.len());
                for (key, val) in &self.type_name_to_index_map_ {
                    stats.type_names[*val] = unsafe { std::ffi::CStr::from_ptr(*key as *const i8).to_string_lossy().into_owned() };
                }
            }

            unsafe{
                DCHECK_GE(heap_base.stats_collector().allocated_memory_size(), stats.resident_size_bytes);
            }

            let pooled_memory = unsafe{heap_base.page_backend().page_pool().PooledMemory()};
            stats.committed_size_bytes += pooled_memory;
            stats.resident_size_bytes += pooled_memory;
            stats.pooled_memory_size_bytes = pooled_memory;

            stats
        }

        fn VisitNormalPageSpace(&mut self, space: &mut NormalPageSpace) -> bool {
            unsafe {
                DCHECK_EQ!(0u, space.linear_allocation_buffer().size());
            }

            Self::FinalizeSpace(self.current_stats_, &mut self.current_space_stats_, &mut self.current_page_stats_);

            self.current_space_stats_ = Self::InitializeSpace(self.current_stats_, Self::GetNormalPageSpaceName(space.index()));

            let current_space_stats = unsafe{ &mut *self.current_space_stats_ };
            space.free_list().CollectStatistics(&mut current_space_stats.free_list_stats);

            false
        }

        fn VisitLargePageSpace(&mut self, space: &mut LargePageSpace) -> bool {
            Self::FinalizeSpace(self.current_stats_, &mut self.current_space_stats_, &mut self.current_page_stats_);

            self.current_space_stats_ = Self::InitializeSpace(self.current_stats_, "LargePageSpace".to_string());

            false
        }

        fn VisitNormalPage(&mut self, page: &mut NormalPage) -> bool {
            unsafe {
                DCHECK_NOT_NULL(self.current_space_stats_);
            }
            Self::FinalizePage(unsafe{ &mut *self.current_space_stats_ }, &mut self.current_page_stats_);

            self.current_page_stats_ = Self::InitializePage(unsafe{ &mut *self.current_space_stats_ });

            let current_page_stats = unsafe{ &mut *self.current_page_stats_ };
            current_page_stats.committed_size_bytes = kPageSize;
            current_page_stats.resident_size_bytes = kPageSize - page.discarded_memory();
            false
        }

        fn VisitLargePage(&mut self, page: &mut LargePage) -> bool {
            unsafe {
                DCHECK_NOT_NULL(self.current_space_stats_);
            }
            Self::FinalizePage(unsafe{ &mut *self.current_space_stats_ }, &mut self.current_page_stats_);

            let object_size = page.PayloadSize();
            let allocated_size = LargePage::AllocationSize(object_size);

            self.current_page_stats_ = Self::InitializePage(unsafe{ &mut *self.current_space_stats_ });

            let current_page_stats = unsafe{ &mut *self.current_page_stats_ };
            current_page_stats.committed_size_bytes = allocated_size;
            current_page_stats.resident_size_bytes = allocated_size;
            false
        }

        fn VisitHeapObjectHeader(&mut self, header: &mut HeapObjectHeader) -> bool {
            if header.IsFree() {
                return true;
            }

            unsafe {
                DCHECK_NOT_NULL(self.current_space_stats_);
                DCHECK_NOT_NULL(self.current_page_stats_);
            }

            let allocated_object_size = if header.IsLargeObject() {
                let large_page = LargePage::From(BasePage::FromPayload(header));
                large_page.PayloadSize()
            } else {
                header.AllocatedSize()
            };

            let current_page_stats = unsafe{ &mut *self.current_page_stats_ };

            Self::RecordObjectType(&mut self.type_name_to_index_map_, &mut current_page_stats.object_statistics, header, allocated_object_size);
            current_page_stats.used_size_bytes += allocated_object_size;

            true
        }

        fn Traverse(&mut self, raw_heap: &mut RawHeap) {
            // Visit NormalPageSpaces
            for i in 0..RawHeap::kNumberOfRegularSpaces() {
                let mut space = raw_heap.normal_page_space(i);
                self.VisitNormalPageSpace(space);

                let normal_space_ref = unsafe{&mut *space};
                for page_idx in 0..normal_space_ref.number_of_pages(){
                   if let Some(page) = normal_space_ref.page_at(page_idx){
                        self.VisitNormalPage(unsafe { &mut *page });
                        let page_ref = unsafe{ &mut *page };
                        // Iterate through objects on the page
                         page_ref.VisitAllObjects(|object_header|{
                            self.VisitHeapObjectHeader(object_header);
                        });
                   }
                }
            }

            // Visit LargePageSpace
            let mut large_page_space = raw_heap.large_page_space();
            self.VisitLargePageSpace(large_page_space);

            let large_space_ref = unsafe{&mut *large_page_space};

            // Visit LargePages
            for page_idx in 0..large_space_ref.number_of_pages(){
                if let Some(page) = large_space_ref.page_at(page_idx){
                    self.VisitLargePage(unsafe { &mut *page });
                    self.VisitHeapObjectHeader(unsafe{ &mut *page.object() });
                }
            }
        }

        fn GetNormalPageSpaceName(index: usize) -> String {
            unsafe {
                DCHECK_NE!(RawHeap_kNumberOfRegularSpaces_minus_1, index);
            }

            if index < RawHeap::kNumberOfRegularSpaces() {
                return format!("NormalPageSpace{}", index);
            }

            return format!("CustomSpace{}", index - RawHeap::kNumberOfRegularSpaces());
        }

        fn InitializeSpace(stats: *mut HeapStatistics, name: String) -> *mut HeapStatistics_SpaceStatistics {
            let stats_ref = unsafe {&mut *stats};
            stats_ref.space_stats.push(HeapStatistics_SpaceStatistics::new());
            let space_stats = stats_ref.space_stats.last_mut().unwrap();
            space_stats.name = name;

            space_stats as *mut HeapStatistics_SpaceStatistics
        }

        fn InitializePage(stats: *mut HeapStatistics_SpaceStatistics) -> *mut HeapStatistics_PageStatistics {
            let stats_ref = unsafe {&mut *stats};
            stats_ref.page_stats.push(HeapStatistics_PageStatistics::new());
            let page_stats = stats_ref.page_stats.last_mut().unwrap();

            page_stats as *mut HeapStatistics_PageStatistics
        }

        fn FinalizePage(space_stats: *mut HeapStatistics_SpaceStatistics, page_stats: &mut *mut HeapStatistics_PageStatistics) {
            if !(*page_stats).is_null() {
                unsafe {
                    DCHECK_NOT_NULL(space_stats);
                    let space_stats_ref = &mut *space_stats;
                    let page_stats_ref = &mut **page_stats;

                    space_stats_ref.committed_size_bytes += page_stats_ref.committed_size_bytes;
                    space_stats_ref.resident_size_bytes += page_stats_ref.resident_size_bytes;
                    space_stats_ref.used_size_bytes += page_stats_ref.used_size_bytes;
                }
            }
            *page_stats = std::ptr::null_mut();
        }

        fn FinalizeSpace(stats: *mut HeapStatistics, space_stats: &mut *mut HeapStatistics_SpaceStatistics, page_stats: &mut *mut HeapStatistics_PageStatistics) {
            Self::FinalizePage(*space_stats, page_stats);

            if !(*space_stats).is_null() {
                unsafe {
                    DCHECK_NOT_NULL(stats);
                    let stats_ref = &mut *stats;
                    let space_stats_ref = &mut **space_stats;

                    stats_ref.committed_size_bytes += space_stats_ref.committed_size_bytes;
                    stats_ref.resident_size_bytes += space_stats_ref.resident_size_bytes;
                    stats_ref.used_size_bytes += space_stats_ref.used_size_bytes;
                }
            }
            *space_stats = std::ptr::null_mut();
        }

        fn RecordObjectType(
            type_map: &mut HashMap<*const std::ffi::c_void, usize>,
            object_statistics: &mut Vec<HeapStatistics::ObjectStatsEntry>,
            header: &mut HeapObjectHeader,
            object_size: usize,
        ) {
            if NameProvider::SupportsCppClassNamesAsObjectNames() {
                let name_ptr = header.GetName().value;

                let type_index = match type_map.get(&name_ptr) {
                    Some(index) => *index,
                    None => {
                        let new_index = type_map.len();
                        type_map.insert(name_ptr, new_index);
                        new_index
                    }
                };

                if object_statistics.len() <= type_index {
                    object_statistics.resize(type_index + 1);
                }

                object_statistics[type_index].allocated_bytes += object_size;
                object_statistics[type_index].object_count += 1;
            }
        }
    }
}
