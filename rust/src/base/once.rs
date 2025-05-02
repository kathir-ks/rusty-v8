use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Once;
use std::thread;
use std::time::Duration;

pub mod base {
    use super::*;

    pub type OnceType = AtomicU8;

    pub const ONCE_STATE_UNINITIALIZED: u8 = 0;
    pub const ONCE_STATE_EXECUTING_FUNCTION: u8 = 1;
    pub const ONCE_STATE_DONE: u8 = 2;

    pub fn call_once_impl(once: &OnceType, init_func: impl FnOnce()) {
        // Fast path. The provided function was already executed.
        if once.load(Ordering::Acquire) == ONCE_STATE_DONE {
            return;
        }

        // The function execution did not complete yet. The once object can be in one
        // of the two following states:
        //   - UNINITIALIZED: We are the first thread calling this function.
        //   - EXECUTING_FUNCTION: Another thread is already executing the function.
        //
        // First, try to change the state from UNINITIALIZED to EXECUTING_FUNCTION
        // atomically.
        let mut expected = ONCE_STATE_UNINITIALIZED;
        if once.compare_exchange(
            expected,
            ONCE_STATE_EXECUTING_FUNCTION,
            Ordering::AcqRel,
            Ordering::Acquire,
        )
        .is_ok()
        {
            // We are the first thread to call this function, so we have to call the
            // function.
            init_func();
            once.store(ONCE_STATE_DONE, Ordering::Release);
        } else {
            // Another thread has already started executing the function. We need to
            // wait until it completes the initialization.
            while once.load(Ordering::Acquire) == ONCE_STATE_EXECUTING_FUNCTION {
                #[cfg(target_os = "windows")]
                {
                    thread::sleep(Duration::from_millis(0));
                }
                #[cfg(all(not(target_os = "windows"), target_os = "linux"))] //Example target, needs adaptation for V8_OS_STARBOARD
                {
                    thread::yield_now();
                }
                #[cfg(not(any(target_os = "windows", target_os = "linux")))]
                {
                    thread::yield_now(); // General yield
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::sync::atomic::{AtomicU32, Ordering};
        use std::sync::Arc;
        use std::thread;

        #[test]
        fn test_call_once_impl() {
            let once = Arc::new(AtomicU8::new(ONCE_STATE_UNINITIALIZED));
            let counter = Arc::new(AtomicU32::new(0));

            let mut handles = vec![];

            for _ in 0..10 {
                let once_clone = once.clone();
                let counter_clone = counter.clone();

                let handle = thread::spawn(move || {
                    call_once_impl(&once_clone, || {
                        counter_clone.fetch_add(1, Ordering::SeqCst);
                    });
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            assert_eq!(counter.load(Ordering::SeqCst), 1);
            assert_eq!(once.load(Ordering::SeqCst), ONCE_STATE_DONE);
        }
    }
}