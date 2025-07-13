// Converted from V8 C++ source files:
// Header: N/A
// Implementation: trace-config.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::io::{self, BufRead};
use std::str;

#[derive(Debug)]
pub struct TraceConfig {
    included_categories_: Vec<String>,
}

impl TraceConfig {
    pub fn CreateDefaultTraceConfig() -> Box<TraceConfig> {
        let mut trace_config = Box::new(TraceConfig {
            included_categories_: Vec::new(),
        });
        trace_config.included_categories_.push("v8".to_string());
        trace_config
    }

    pub fn IsCategoryGroupEnabled(&self, category_group: &str) -> bool {
        let category_stream = category_group.split(',');
        for category in category_stream {
            let category = category.trim();
            if category.is_empty() {
                continue;
            }
            for included_category in &self.included_categories_ {
                if category == included_category {
                    return true;
                }
            }
        }
        false
    }

    pub fn AddIncludedCategory(&mut self, included_category: &str) {
        assert!(!included_category.is_empty());
        self.included_categories_.push(included_category.to_string());
    }
}
