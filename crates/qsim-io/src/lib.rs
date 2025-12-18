//! # qsim-io
//!
//! JSON I/O, serialization, and persistence for qsim.
//!
//! ## Features
//!
//! - Load/save networks from JSON
//! - Checkpoint simulation state
//! - Export results to JSON/CSV

mod network;
mod error;

pub use network::*;
pub use error::*;
