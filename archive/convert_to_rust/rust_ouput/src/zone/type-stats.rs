// Converted from V8 C++ source files:
// Header: type-stats.h
// Implementation: type-stats.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod type_stats {
    use std::any::TypeId;
    use std::collections::HashMap;
    use std::fmt;
    use std::mem;

    #[cfg(feature = "v8_enable_precise_zone_stats")]
    pub struct TypeStats {
        map_: HashMap<TypeId, StatsEntry>,
    }

    #[cfg(feature = "v8_enable_precise_zone_stats")]
    impl TypeStats {
        pub fn new() -> Self {
            TypeStats {
                map_: HashMap::new(),
            }
        }

        pub fn add_allocated<TypeTag: 'static>(&mut self, bytes: usize) {
            let type_id = TypeId::of::<TypeTag>();
            let entry = self.map_.entry(type_id).or_insert(StatsEntry::new());
            entry.allocation_count += 1;
            entry.allocated_bytes += bytes;

            let k_is_incomplete = TypeId::of::<TypeTag>() == TypeId::of::<()>()
                || TypeId::of::<TypeTag>() == TypeId::of::<[u8; 0]>();
            if k_is_incomplete {
                entry.instance_size = mem::size_of::<char>();
            } else {
                entry.instance_size = mem::size_of::<TypeTag>();
            }
        }

        pub fn add_deallocated<TypeTag: 'static>(&mut self, bytes: usize) {
            let type_id = TypeId::of::<TypeTag>();
            let entry = self.map_.entry(type_id).or_insert(StatsEntry::new());
            entry.deallocated_bytes += bytes;
        }

        pub fn merge_with(&mut self, other: &TypeStats) {
            for (type_id, other_entry) in &other.map_ {
                self.add(*type_id, other_entry);
            }
        }

        fn add(&mut self, type_id: TypeId, other_entry: &StatsEntry) {
            let entry = self.map_.entry(type_id).or_insert(StatsEntry::new());
            entry.allocation_count += other_entry.allocation_count;
            entry.allocated_bytes += other_entry.allocated_bytes;
            entry.deallocated_bytes += other_entry.deallocated_bytes;
            entry.instance_size = other_entry.instance_size;
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

            for (type_id, entry) in &self.map_ {
                total_allocation_count += entry.allocation_count as u64;
                total_allocated_bytes += entry.allocated_bytes as u64;
                total_deallocated_bytes += entry.deallocated_bytes as u64;

                let type_name = demangler.demangle(*type_id);
                println!(
                    "{:12} | {:12} | {:10} | {:6} | {}",
                    entry.allocated_bytes,
                    entry.deallocated_bytes,
                    entry.allocation_count,
                    entry.instance_size,
                    type_name
                );
            }

            println!(
                "{:12} | {:12} | {:10} | ===== TOTAL STATS =====\n",
                total_allocated_bytes, total_deallocated_bytes, total_allocation_count
            );
        }
    }

    #[derive(Default, Clone, Copy)]
    struct StatsEntry {
        allocation_count: usize,
        allocated_bytes: usize,
        deallocated_bytes: usize,
        instance_size: usize,
    }

    impl StatsEntry {
        fn new() -> Self {
            StatsEntry {
                allocation_count: 0,
                allocated_bytes: 0,
                deallocated_bytes: 0,
                instance_size: 0,
            }
        }
    }

    struct Demangler {
        buffer_: Option<String>,
    }

    impl Demangler {
        fn new() -> Self {
            Demangler { buffer_: None }
        }

        fn demangle(&self, type_id: TypeId) -> String {
             format!("{:?}", type_id)
        }
    }
}
