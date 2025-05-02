// Copyright 2014 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Note: This translation is based on the available C++ code. A complete
// Rust translation of the V8 JavaScript engine is a monumental task. This
// focuses on the functionality within the given file.

// Corresponding header file might look like this:
// mod execution;
// mod heap;
// mod date;

mod execution {
    // Placeholder for execution related structs/functions
    pub struct Arguments {
        pub length: usize,
    }

    pub struct Isolate {
        factory: heap::Factory,
    }

    impl Isolate {
        pub fn factory(&self) -> &heap::Factory {
            &self.factory
        }
    }
}

mod heap {
    // Placeholder for heap related structs/functions
    #[derive(Default)]
    pub struct Factory {}

    impl Factory {
        pub fn new_number_from_int64(&self, value: i64) -> Box<Number> {
            Box::new(Number { value })
        }
    }

    #[derive(Debug)]
    pub struct Number {
        pub value: i64,
    }
}

mod date {
    // Placeholder for date related structs/functions
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn current_time_value() -> i64 {
        let now = SystemTime::now();
        let since_the_epoch = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_millis() as i64
    }
}

// runtime function
pub fn runtime_date_current_time(isolate: &execution::Isolate, args: execution::Arguments) -> Box<heap::Number> {
    assert_eq!(args.length, 0);
    isolate.factory().new_number_from_int64(date::current_time_value())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_date_current_time() {
        let isolate = execution::Isolate {
            factory: heap::Factory::default(),
        };
        let args = execution::Arguments { length: 0 };
        let result = runtime_date_current_time(&isolate, args);
        assert!(result.value > 0);
    }
}