#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(feature = "v8_enable_precise_zone_stats")]
mod type_stats {
    use std::any::TypeId;
    use std::collections::HashMap;
    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;
    use std::sync::Mutex;

    #[derive(Default, Debug, Clone, Copy)]
    pub struct StatsEntry {
        pub allocated_bytes: usize,
        pub deallocated_bytes: usize,
        pub allocation_count: usize,
        pub instance_size: usize,
    }

    #[derive(Default, Debug)]
    pub struct TypeStats {
        map_: Mutex<HashMap<TypeId, StatsEntry>>,
    }

    impl TypeStats {
        pub fn new() -> Self {
            TypeStats {
                map_: Mutex::new(HashMap::new()),
            }
        }

        pub fn add(&self, type_id: TypeId, stats_entry: StatsEntry) {
            let mut map = self.map_.lock().unwrap();
            map.entry(type_id)
                .and_modify(|existing_entry| {
                    existing_entry.allocated_bytes += stats_entry.allocated_bytes;
                    existing_entry.deallocated_bytes += stats_entry.deallocated_bytes;
                    existing_entry.allocation_count += stats_entry.allocation_count;
                })
                .or_insert(stats_entry);
        }

        pub fn merge_with(&self, other: &TypeStats) {
            let other_map = other.map_.lock().unwrap();
            for (&type_id, &stats_entry) in other_map.iter() {
                self.add(type_id, stats_entry);
            }
        }

        pub fn record_allocation(&self, type_id: TypeId, size: usize) {
            let mut map = self.map_.lock().unwrap();
            map.entry(type_id)
                .and_modify(|entry| {
                    entry.allocated_bytes += size;
                    entry.allocation_count += 1;
                })
                .or_insert(StatsEntry {
                    allocated_bytes: size,
                    deallocated_bytes: 0,
                    allocation_count: 1,
                    instance_size: size,
                });
        }

        pub fn record_deallocation(&self, type_id: TypeId, size: usize) {
            let mut map = self.map_.lock().unwrap();
            map.entry(type_id)
                .and_modify(|entry| {
                    entry.deallocated_bytes += size;
                })
                .or_insert(StatsEntry {
                    allocated_bytes: 0,
                    deallocated_bytes: size,
                    allocation_count: 0,
                    instance_size: size, // Or 0 if size is unknown at allocation
                });
        }

        pub fn dump(&self) {
            let demangler = Demangler::new();
            println!("===== TypeStats =====");
            println!("-------------+--------------+------------+--------+--------------");
            println!("       alloc |      dealloc |      count | sizeof | name");
            println!("-------------+--------------+------------+--------+--------------");

            let mut total_allocation_count: u64 = 0;
            let mut total_allocated_bytes: u64 = 0;
            let mut total_deallocated_bytes: u64 = 0;

            let map = self.map_.lock().unwrap();
            for (&type_id, &entry) in map.iter() {
                total_allocation_count += entry.allocation_count as u64;
                total_allocated_bytes += entry.allocated_bytes as u64;
                total_deallocated_bytes += entry.deallocated_bytes as u64;
                println!(
                    "{:12} | {:12} | {:10} | {:6} | {}",
                    entry.allocated_bytes,
                    entry.deallocated_bytes,
                    entry.allocation_count,
                    entry.instance_size,
                    demangler.demangle(type_id)
                );
            }

            println!(
                "{:12} | {:12} | {:10} | ===== TOTAL STATS =====\n",
                total_allocated_bytes, total_deallocated_bytes, total_allocation_count
            );
        }
    }

    struct Demangler {
        // The cxxabi crate is not directly usable in no_std environments
        // This is a stub implementation, as the cxxabi crate cannot be included without
        // adding a dependency on the standard library.  If this file is built in a
        // std-enabled environment, this can be replaced with a real demangler
    }

    impl Demangler {
        fn new() -> Self {
            Demangler {}
        }

        fn demangle(&self, type_id: TypeId) -> String {
            // This is a placeholder; replace with actual demangling if needed.
            // Use std::any::type_name if available.  Requires 'std'
            //std::any::type_name(type_id).to_string()
            format!("{:?}", type_id)
        }
    }
}

#[cfg(not(feature = "v8_enable_precise_zone_stats"))]
mod type_stats {
    // Define empty structs and methods when the feature is disabled.
    #[derive(Default, Debug, Clone, Copy)]
    pub struct StatsEntry {}

    #[derive(Default, Debug)]
    pub struct TypeStats {}

    impl TypeStats {
        pub fn new() -> Self {
            TypeStats {}
        }
        pub fn add(&self, _type_id: std::any::TypeId, _stats_entry: StatsEntry) {}
        pub fn merge_with(&self, _other: &TypeStats) {}
        pub fn record_allocation(&self, _type_id: std::any::TypeId, _size: usize) {}
        pub fn record_deallocation(&self, _type_id: std::any::TypeId, _size: usize) {}
        pub fn dump(&self) {}
    }
}

pub use type_stats::*;