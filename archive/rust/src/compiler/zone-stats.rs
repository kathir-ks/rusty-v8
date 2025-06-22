use std::cmp;
use std::collections::HashMap;
use std::ptr::NonNull;

/// ZoneStats module for tracking zone allocations.
pub struct ZoneStats<'a> {
    max_allocated_bytes: usize,
    total_deleted_bytes: usize,
    allocator: &'a AccountingAllocator,
    zones: Vec<NonNull<Zone>>,
    stats: Vec<*mut StatsScope<'a>>,
}

impl<'a> ZoneStats<'a> {
    /// Creates a new ZoneStats instance.
    pub fn new(allocator: &'a AccountingAllocator) -> Self {
        ZoneStats {
            max_allocated_bytes: 0,
            total_deleted_bytes: 0,
            allocator,
            zones: Vec::new(),
            stats: Vec::new(),
        }
    }

    /// Returns the maximum allocated bytes.
    pub fn get_max_allocated_bytes(&self) -> usize {
        cmp::max(self.max_allocated_bytes, self.get_current_allocated_bytes())
    }

    /// Returns the current allocated bytes.
    pub fn get_current_allocated_bytes(&self) -> usize {
        self.zones
            .iter()
            .map(|&zone| unsafe { zone.as_ref().allocation_size() as usize })
            .sum()
    }

    /// Returns the total allocated bytes.
    pub fn get_total_allocated_bytes(&self) -> usize {
        self.total_deleted_bytes + self.get_current_allocated_bytes()
    }

    /// Creates a new empty zone.
    pub fn new_empty_zone(&mut self, zone_name: &str, support_zone_compression: bool) -> NonNull<Zone> {
        let zone = Box::new(Zone::new(self.allocator, zone_name, support_zone_compression));
        let zone_ptr = NonNull::new(Box::into_raw(zone)).unwrap();
        self.zones.push(zone_ptr);
        zone_ptr
    }

    /// Returns a zone, updating statistics.
    pub fn return_zone(&mut self, zone: NonNull<Zone>) {
        let current_total = self.get_current_allocated_bytes();
        self.max_allocated_bytes = cmp::max(self.max_allocated_bytes, current_total);

        for &stat_scope in &self.stats {
            unsafe {
                (*stat_scope).zone_returned(zone);
            }
        }

        if let Some(index) = self.zones.iter().position(|&x| x == zone) {
            self.zones.remove(index);
        } else {
            panic!("Zone not found in zones_"); // Or handle the error more gracefully
        }

        unsafe {
            self.total_deleted_bytes += zone.as_ref().allocation_size() as usize;
            drop(Box::from_raw(zone.as_ptr()));
        }
    }
}

impl<'a> Drop for ZoneStats<'a> {
    fn drop(&mut self) {
        assert!(self.zones.is_empty());
        assert!(self.stats.is_empty());
    }
}

/// AccountingAllocator struct (placeholder).
#[derive(Debug)]
pub struct AccountingAllocator {}

impl AccountingAllocator {
    pub fn new() -> Self {
        AccountingAllocator {}
    }
}

/// Zone struct (placeholder).
#[derive(Debug)]
pub struct Zone {
    name: String,
    support_zone_compression: bool,
    allocator: *const AccountingAllocator, // Store a raw pointer for now (lifetime issues)
    allocation_size: usize,
}

impl Zone {
    /// Creates a new Zone instance.
    pub fn new(allocator: &AccountingAllocator, name: &str, support_zone_compression: bool) -> Self {
        Zone {
            name: name.to_string(),
            support_zone_compression,
            allocator,
            allocation_size: 0,
        }
    }

    /// Returns the allocation size of the zone.
    pub fn allocation_size(&self) -> usize {
        self.allocation_size
    }
}

/// StatsScope struct for managing zone statistics within a scope.
pub struct StatsScope<'a> {
    zone_stats: *mut ZoneStats<'a>,
    total_allocated_bytes_at_start: usize,
    max_allocated_bytes: usize,
    initial_values: HashMap<NonNull<Zone>, usize>,
}

impl<'a> StatsScope<'a> {
    /// Creates a new StatsScope instance.
    pub fn new(zone_stats: *mut ZoneStats<'a>) -> Self {
        let total_allocated_bytes_at_start = unsafe { (*zone_stats).get_total_allocated_bytes() };
        let mut initial_values = HashMap::new();

        unsafe {
            for &zone in &(*zone_stats).zones {
                let size = zone.as_ref().allocation_size() as usize;
                initial_values.insert(zone, size);
            }
            (*zone_stats).stats.push(std::mem::transmute(zone_stats));
        }

        StatsScope {
            zone_stats,
            total_allocated_bytes_at_start,
            max_allocated_bytes: 0,
            initial_values,
        }
    }

    /// Returns the maximum allocated bytes within the scope.
    pub fn get_max_allocated_bytes(&self) -> usize {
        cmp::max(self.max_allocated_bytes, self.get_current_allocated_bytes())
    }

    /// Returns the current allocated bytes within the scope.
    pub fn get_current_allocated_bytes(&self) -> usize {
        let mut total = 0;
        unsafe {
            for &zone in &(*self.zone_stats).zones {
                total += zone.as_ref().allocation_size() as usize;
                if let Some(&initial_size) = self.initial_values.get(&zone) {
                    total -= initial_size;
                }
            }
        }
        total
    }

    /// Returns the total allocated bytes within the scope.
    pub fn get_total_allocated_bytes(&self) -> usize {
        unsafe { (*self.zone_stats).get_total_allocated_bytes() - self.total_allocated_bytes_at_start }
    }

    /// Called when a zone is returned.
    pub fn zone_returned(&mut self, zone: NonNull<Zone>) {
        let current_total = self.get_current_allocated_bytes();
        self.max_allocated_bytes = cmp::max(self.max_allocated_bytes, current_total);
        self.initial_values.remove(&zone);
    }
}

impl<'a> Drop for StatsScope<'a> {
    fn drop(&mut self) {
        unsafe {
            let stats = &mut (*self.zone_stats).stats;
            assert_eq!(stats.last().copied(), Some(self as *const Self as *mut StatsScope<'a>));
            stats.pop();
        }
    }
}