// Converted from V8 C++ source files:
// Header: v8-external-memory-accounter.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8_external_memory_accounter {
    use std::sync::atomic::{AtomicI64, Ordering};
    use std::sync::Mutex;

    pub struct ExternalMemoryAccounter {
        #[cfg(feature = "v8_enable_memory_accounting_checks")]
        amount_of_external_memory: Mutex<usize>,
        #[cfg(feature = "v8_enable_memory_accounting_checks")]
        isolate: *mut v8::Isolate,
    }

    impl ExternalMemoryAccounter {
        pub fn new() -> Self {
            ExternalMemoryAccounter {
                #[cfg(feature = "v8_enable_memory_accounting_checks")]
                amount_of_external_memory: Mutex::new(0),
                #[cfg(feature = "v8_enable_memory_accounting_checks")]
                isolate: std::ptr::null_mut(),
            }
        }

        pub fn get_total_amount_of_external_allocated_memory_for_testing(
            isolate: *const v8::Isolate,
        ) -> i64 {
            // NOTE: This is a mock implementation.  A real implementation
            //       would need access to internal isolate state.
            //       For now, just return a constant value.
            //       This also needs to be thread safe in a real implementation
            static TOTAL_MEMORY: AtomicI64 = AtomicI64::new(1024);
            TOTAL_MEMORY.load(Ordering::Relaxed)
        }

        pub fn increase(&mut self, isolate: *mut v8::Isolate, size: usize) {
            #[cfg(feature = "v8_enable_memory_accounting_checks")]
            {
                let mut amount = self.amount_of_external_memory.lock().unwrap();
                *amount += size;
                self.isolate = isolate;
            }

            // NOTE: A real implementation would notify the isolate
            //       about the increased external memory.  This would
            //       likely involve calling a method on the isolate.
        }

        pub fn update(&mut self, isolate: *mut v8::Isolate, delta: i64) {
            // NOTE: A real implementation would notify the isolate
            //       about the updated external memory.  This would
            //       likely involve calling a method on the isolate.

            #[cfg(feature = "v8_enable_memory_accounting_checks")]
            {
              // The C++ version doesn't actually use isolate here, so we won't either
              let mut amount = self.amount_of_external_memory.lock().unwrap();
              if delta >= 0 {
                *amount = (*amount).saturating_add(delta as usize);
              } else {
                *amount = (*amount).saturating_sub((-delta) as usize);
              }
            }
        }

        pub fn decrease(&mut self, isolate: *mut v8::Isolate, size: usize) {
            #[cfg(feature = "v8_enable_memory_accounting_checks")]
            {
                let mut amount = self.amount_of_external_memory.lock().unwrap();
                *amount = amount.saturating_sub(size);
                self.isolate = isolate;
            }

            // NOTE: A real implementation would notify the isolate
            //       about the decreased external memory.  This would
            //       likely involve calling a method on the isolate.
        }
    }

    impl Drop for ExternalMemoryAccounter {
        fn drop(&mut self) {
            #[cfg(feature = "v8_enable_memory_accounting_checks")]
            {
                let amount = self.amount_of_external_memory.lock().unwrap();
                if *amount != 0 {
                  eprintln!("ExternalMemoryAccounter dropped with non-zero external memory: {}", *amount);
                   //panic!("ExternalMemoryAccounter dropped with non-zero external memory");
                }
            }
        }
    }

    impl ExternalMemoryAccounter {
      pub fn assign(&mut self, other: &mut Self) {
        #[cfg(feature = "v8_enable_memory_accounting_checks")]
        {
          let other_amount = other.amount_of_external_memory.lock().unwrap();
          let mut amount = self.amount_of_external_memory.lock().unwrap();
          *amount = *other_amount;
          self.isolate = other.isolate;
        }
      }
    }

    mod v8 {
        pub struct Isolate {}
    }
}
