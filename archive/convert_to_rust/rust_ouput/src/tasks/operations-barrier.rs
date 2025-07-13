// Converted from V8 C++ source files:
// Header: operations-barrier.h
// Implementation: operations-barrier.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/tasks/operations_barrier.rs

use std::sync::{Arc, Mutex, Condvar};
use std::ops::Deref;

pub struct OperationsBarrier {
    mutex: Arc<Mutex<OperationsBarrierState>>,
    release_condition: Arc<Condvar>,
}

struct OperationsBarrierState {
    cancelled: bool,
    operations_count: usize,
}

impl OperationsBarrier {
    pub fn new() -> Self {
        OperationsBarrier {
            mutex: Arc::new(Mutex::new(OperationsBarrierState {
                cancelled: false,
                operations_count: 0,
            })),
            release_condition: Arc::new(Condvar::new()),
        }
    }

    pub fn try_lock(self: &Arc<Self>) -> Option<Token> {
        let mut state = self.mutex.lock().unwrap();
        if state.cancelled {
            return None;
        }
        state.operations_count += 1;
        Some(Token::new(Arc::clone(self)))
    }

    pub fn cancel_and_wait(&self) {
        let mut state = self.mutex.lock().unwrap();
        assert!(!state.cancelled);
        state.cancelled = true;
        while state.operations_count > 0 {
            state = self.release_condition.wait(state).unwrap();
        }
    }

    pub fn cancelled(&self) -> bool {
        self.mutex.lock().unwrap().cancelled
    }

    fn release(&self) {
        let mut state = self.mutex.lock().unwrap();
        state.operations_count -= 1;
        if state.operations_count == 0 && state.cancelled {
            self.release_condition.notify_one();
        }
    }
}

impl Drop for OperationsBarrier {
    fn drop(&mut self) {
        assert!(self.cancelled());
    }
}

pub struct Token {
    outer: Option<Arc<OperationsBarrier>>,
}

impl Token {
    fn new(outer: Arc<OperationsBarrier>) -> Self {
        Token { outer: Some(outer) }
    }
}

impl Drop for Token {
    fn drop(&mut self) {
        if let Some(outer) = self.outer.take() {
            outer.release();
        }
    }
}

impl Token {
    pub fn is_valid(&self) -> bool {
        self.outer.is_some()
    }
}

impl Deref for Token {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        if self.outer.is_some() {
            &true
        } else {
            &false
        }
    }
}
