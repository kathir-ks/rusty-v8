//! V8 JavaScript Engine - Rust Port
//! 
//! This is an automated conversion of the V8 JavaScript engine from C++ to Rust.
//! The conversion maintains the original architecture while leveraging Rust's
//! memory safety and performance characteristics.

#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

// Re-export main modules
// Note: You'll need to add actual module declarations based on converted files

pub mod common;
pub mod base;
pub mod runtime;

// Common types and utilities
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
