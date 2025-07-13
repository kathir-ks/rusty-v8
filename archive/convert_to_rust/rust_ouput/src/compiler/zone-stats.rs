// Converted from V8 C++ source files:
// Header: zone-stats.h
// Implementation: zone-stats.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod zone {
    pub struct Zone {
        name: String,
        allocation_size: usize,
    }

    impl Zone {
        pub fn new(name: &str) -> Self {
            Zone {
                name: name.to_string(),
                allocation_size: 0,
            }
        }

        pub fn allocation_size(&self) -> usize {
            self.allocation_size
        }
    }
}

use std::collections::HashMap;
use std::sync::Mutex;

pub struct V8_EXPORT_PRIVATE {}
pub struct V8_NODISCARD {}

pub struct AccountingAllocator {}

impl AccountingAllocator {
    pub fn new() -> Self {
        AccountingAllocator {}
    }
}

pub struct ZoneStats {
    zones_: Mutex<Vec<*mut zone::Zone>>,
    stats_: Mutex<Vec<*mut StatsScope>>,
    max_allocated_bytes_: Mutex<usize>,
    total_deleted_bytes_: Mutex<usize>,
    allocator_: *mut AccountingAllocator,
}

impl ZoneStats {
    pub fn new(allocator: *mut AccountingAllocator) -> Self {
        ZoneStats {
            zones_: Mutex::new(Vec::new()),
            stats_: Mutex::new(Vec::new()),
            max_allocated_bytes_: Mutex::new(0),
            total_deleted_bytes_: Mutex::new(0),
            allocator_: allocator,
        }
    }

    pub fn get_max_allocated_bytes(&self) -> usize {
        *self.max_allocated_bytes_.lock().unwrap()
    }

    pub fn get_total_allocated_bytes(&self) -> usize {
        *self.total_deleted_bytes_.lock().unwrap() + self.get_current_allocated_bytes()
    }

    pub fn get_current_allocated_bytes(&self) -> usize {
        let zones = self.zones_.lock().unwrap();
        let mut total = 0;
        for zone in zones.iter() {
            total += unsafe { (*(*zone)).allocation_size() };
        }
        total
    }

    fn new_empty_zone(&self, zone_name: &str, _support_zone_compression: bool) -> *mut zone::Zone {
        let zone = Box::into_raw(Box::new(zone::Zone::new(zone_name)));
        self.zones_.lock().unwrap().push(zone);
        zone
    }

    fn return_zone(&self, zone: *mut zone::Zone) {
        let current_total = self.get_current_allocated_bytes();
        let mut max_allocated_bytes = self.max_allocated_bytes_.lock().unwrap();
        *max_allocated_bytes = std::cmp::max(*max_allocated_bytes, current_total);

        let stats = self.stats_.lock().unwrap();
        for stat_scope in stats.iter() {
            unsafe {
                (*(*stat_scope)).zone_returned(zone);
            }
        }

        let mut zones = self.zones_.lock().unwrap();
        if let Some(pos) = zones.iter().position(|&x| x == zone) {
            zones.remove(pos);
        }

        let mut total_deleted_bytes = self.total_deleted_bytes_.lock().unwrap();
        unsafe {
            *total_deleted_bytes += (*zone).allocation_size();
            drop(Box::from_raw(zone));
        }
    }
}

pub struct Scope<'a> {
    zone_name_: String,
    zone_stats_: *mut ZoneStats,
    zone_: *mut zone::Zone,
    support_zone_compression_: bool,
    _phantom: std::marker::PhantomData<&'a ZoneStats>,
}

impl<'a> Scope<'a> {
    pub fn new(zone_stats: *mut ZoneStats, zone_name: &str, support_zone_compression: bool) -> Self {
        Scope {
            zone_name_: zone_name.to_string(),
            zone_stats_: zone_stats,
            zone_: std::ptr::null_mut(),
            support_zone_compression_: support_zone_compression,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn zone(&mut self) -> *mut zone::Zone {
        if self.zone_.is_null() {
            self.zone_ = unsafe {
                (*self.zone_stats_).new_empty_zone(&self.zone_name_, self.support_zone_compression_)
            };
        }
        self.zone_
    }

    pub fn destroy(&mut self) {
        if !self.zone_.is_null() {
            unsafe {
                (*self.zone_stats_).return_zone(self.zone_);
            }
            self.zone_ = std::ptr::null_mut();
        }
    }

    pub fn zone_stats(&self) -> *mut ZoneStats {
        self.zone_stats_
    }
}

impl<'a> Drop for Scope<'a> {
    fn drop(&mut self) {
        self.destroy();
    }
}

pub struct StatsScope {
    zone_stats_: *mut ZoneStats,
    initial_values_: Mutex<HashMap<*mut zone::Zone, usize>>,
    total_allocated_bytes_at_start_: usize,
    max_allocated_bytes_: Mutex<usize>,
}

impl StatsScope {
    pub fn new(zone_stats: *mut ZoneStats) -> Self {
        let total_allocated_bytes_at_start_ = unsafe { (*zone_stats).get_total_allocated_bytes() };
        let mut initial_values = HashMap::new();
        let zones = unsafe { (*zone_stats).zones_.lock().unwrap() };
        for zone in zones.iter() {
            let size = unsafe { (*(*zone)).allocation_size() };
            initial_values.insert(*zone, size);
        }

        let mut stats = unsafe { (*zone_stats).stats_.lock().unwrap() };
        let stats_scope = StatsScope {
            zone_stats_: zone_stats,
            initial_values_: Mutex::new(initial_values),
            total_allocated_bytes_at_start_: total_allocated_bytes_at_start_,
            max_allocated_bytes_: Mutex::new(0),
        };
        stats.push(Box::into_raw(Box::new(stats_scope)));
        unsafe {
            let stats_scope_ptr = stats.last().unwrap();
            (*(*stats_scope_ptr)).zone_stats_ = zone_stats;
        }
        unsafe { *Box::into_raw(Box::new(stats_scope)) }
    }

    pub fn get_max_allocated_bytes(&self) -> usize {
        std::cmp::max(*self.max_allocated_bytes_.lock().unwrap(), self.get_current_allocated_bytes())
    }

    pub fn get_current_allocated_bytes(&self) -> usize {
        let zone_stats = unsafe { &(*self.zone_stats_) };
        let zones = zone_stats.zones_.lock().unwrap();
        let mut total = 0;
        for zone in zones.iter() {
            total += unsafe { (*(*zone)).allocation_size() };
            let initial_values = self.initial_values_.lock().unwrap();
            if let Some(&initial_value) = initial_values.get(zone) {
                total -= initial_value;
            }
        }
        total
    }

    pub fn get_total_allocated_bytes(&self) -> usize {
        let zone_stats = unsafe { &(*self.zone_stats_) };
        zone_stats.get_total_allocated_bytes() - self.total_allocated_bytes_at_start_
    }

    pub fn zone_returned(&self, zone: *mut zone::Zone) {
        let current_total = self.get_current_allocated_bytes();
        let mut max_allocated_bytes = self.max_allocated_bytes_.lock().unwrap();
        *max_allocated_bytes = std::cmp::max(*max_allocated_bytes, current_total);
        let mut initial_values = self.initial_values_.lock().unwrap();
        initial_values.remove(&zone);
    }
}

impl Drop for StatsScope {
    fn drop(&mut self) {
        let zone_stats = unsafe { &mut (*self.zone_stats_) };
        let mut stats = zone_stats.stats_.lock().unwrap();
        if let Some(pos) = stats.iter().position(|&x| x as *const _ == self as *const _) {
            stats.remove(pos);
        }
    }
}
