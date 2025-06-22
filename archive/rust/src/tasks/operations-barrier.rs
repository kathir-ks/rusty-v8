use std::sync::{Arc, Condvar, Mutex, MutexGuard};

pub struct OperationsBarrier {
    mutex: Mutex<OperationsBarrierState>,
    release_condition: Condvar,
}

struct OperationsBarrierState {
    operations_count: usize,
    cancelled: bool,
}

impl OperationsBarrier {
    pub fn new() -> Self {
        OperationsBarrier {
            mutex: Mutex::new(OperationsBarrierState {
                operations_count: 0,
                cancelled: false,
            }),
            release_condition: Condvar::new(),
        }
    }

    pub fn try_lock(&self) -> Option<Token> {
        let mut guard = self.mutex.lock().unwrap();
        if guard.cancelled {
            return None;
        }
        guard.operations_count += 1;
        Some(Token { barrier: self })
    }

    pub fn cancel_and_wait(&self) {
        let mut guard = self.mutex.lock().unwrap();
        assert!(!guard.cancelled);
        guard.cancelled = true;
        while guard.operations_count > 0 {
            guard = self.release_condition.wait(guard).unwrap();
        }
    }

    fn release(&self) {
        let mut guard = self.mutex.lock().unwrap();
        guard.operations_count -= 1;
        if guard.operations_count == 0 && guard.cancelled {
            self.release_condition.notify_one();
        }
    }
}

pub struct Token<'a> {
    barrier: &'a OperationsBarrier,
}

impl<'a> Token<'a> {
    // prevent outside creation
    fn new(_barrier: &'a OperationsBarrier) -> Self {
        unimplemented!()
    }
}

impl<'a> Drop for Token<'a> {
    fn drop(&mut self) {
        self.barrier.release();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_operations_barrier() {
        let barrier = Arc::new(OperationsBarrier::new());
        let barrier_clone = barrier.clone();

        let handle = thread::spawn(move || {
            let token = barrier_clone.try_lock().unwrap();
            thread::sleep(Duration::from_millis(100));
            drop(token);
        });

        thread::sleep(Duration::from_millis(10));

        barrier.cancel_and_wait();

        handle.join().unwrap();
    }

     #[test]
    fn test_cancel_before_lock() {
        let barrier = Arc::new(OperationsBarrier::new());
        let barrier_clone = barrier.clone();
        barrier.cancel_and_wait();
        assert!(barrier_clone.try_lock().is_none());
    }
}