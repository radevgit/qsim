//! # qsim-core
//!
//! Core engine, traits, and state management for power grid simulation.
//!
//! ## Key Components
//!
//! - [`GridElement`] — Trait for all grid elements
//! - [`Solver`] — Trait for power flow solvers
//! - [`StateStore`] — Central state storage
//! - [`Topology`] — Graph-based network topology

mod error;
mod state;
mod topology;
mod traits;

pub use error::*;
pub use state::*;
pub use topology::*;
pub use traits::*;
