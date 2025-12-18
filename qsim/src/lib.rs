//! # qsim
//!
//! A Rust library for power grid modeling and simulation.
//!
//! ## Features
//!
//! - **Power Flow Analysis** — DC and AC power flow solvers
//! - **Modular Architecture** — Pluggable solvers and elements
//! - **High Performance** — Pure Rust, parallel computation with Rayon
//! - **Graph-Based Topology** — Flexible network representation
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use qsim::prelude::*;
//!
//! // Create a simple 3-bus network
//! let mut network = NetworkData::new("Example");
//!
//! // Add buses
//! network.buses.push(Bus::slack(1.0));
//! network.buses.push(Bus::pv(1.0, 50.0));
//! network.buses.push(Bus::pq(-100.0, -30.0));
//!
//! // Add branches
//! network.branches.push(Branch::line(0, 1, 0.01, 0.1));
//! network.branches.push(Branch::line(1, 2, 0.02, 0.15));
//!
//! // Save to JSON
//! network.to_file("network.json")?;
//! ```
//!
//! ## Crate Structure
//!
//! - `qsim-core` — Core engine, traits, state management
//! - `qsim-elements` — Grid elements (Bus, Branch, Generator, Load)
//! - `qsim-solvers` — Power flow solvers (DC, AC)
//! - `qsim-io` — JSON I/O, serialization, persistence

// Re-export all public APIs
pub use qsim_core as core;
pub use qsim_elements as elements;
pub use qsim_solvers as solvers;
pub use qsim_io as io;

/// Prelude for convenient imports
pub mod prelude {
    // Core types
    pub use qsim_core::{
        CoreError, GridElement, OutputHandler, Result, Solver, SolverResult, StateStore,
        TimeStepper, Topology, BusId, BranchId,
    };

    // Elements
    pub use qsim_elements::{Branch, Bus, BusType, Generator, Load};

    // Solvers
    pub use qsim_solvers::DcPowerFlowSolver;

    // I/O
    pub use qsim_io::NetworkData;
}
