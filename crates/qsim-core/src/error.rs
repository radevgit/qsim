//! Error types for qsim-core

use thiserror::Error;

/// Core error types
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Invalid bus ID: {0}")]
    InvalidBusId(usize),

    #[error("Invalid branch ID: {0}")]
    InvalidBranchId(usize),

    #[error("Topology error: {0}")]
    TopologyError(String),

    #[error("State error: {0}")]
    StateError(String),

    #[error("Simulation error: {0}")]
    SimulationError(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;
