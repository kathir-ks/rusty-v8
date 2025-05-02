// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::fmt;
use std::str;

/// Represents the trace configuration for the V8 engine.
pub struct TraceConfig {
    included_categories: Vec<String>,
}

impl TraceConfig {
    /// Creates a default `TraceConfig` with the "v8" category included.
    pub fn create_default_trace_config() -> Self {
        let mut trace_config = TraceConfig {
            included_categories: Vec::new(),
        };
        trace_config.included_categories.push("v8".to_string());
        trace_config
    }

    /// Checks if a given category group is enabled based on the included categories.
    pub fn is_category_group_enabled(&self, category_group: &str) -> bool {
        for category in category_group.split(',') {
            let category = category.trim();
            if category.is_empty() {
              continue;
            }
            for included_category in &self.included_categories {
                if category == included_category {
                    return true;
                }
            }
        }
        false
    }

    /// Adds a category to the list of included categories.
    pub fn add_included_category(&mut self, included_category: &str) {
        assert!(!included_category.is_empty());
        self.included_categories.push(included_category.to_string());
    }
}

impl fmt::Debug for TraceConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TraceConfig")
            .field("included_categories", &self.included_categories)
            .finish()
    }
}