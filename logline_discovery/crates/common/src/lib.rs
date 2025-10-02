//! LogLine Discovery Lab Common Types and Utilities
//!
//! This crate provides shared types, error handling, and utilities
//! used across all LogLine Discovery Lab components.

pub mod config;
pub mod error;
pub mod job;
pub mod span;
pub mod triage;

pub use error::{Error, Result};
