//! Core traits for qsim

use crate::{Result, StateStore, Topology};

/// Trait for grid elements (buses, branches, generators, loads)
pub trait GridElement: Send + Sync {
    /// Element type name for debugging
    fn element_type(&self) -> &'static str;

    /// Apply element's contribution to the state
    fn apply(&self, state: &mut StateStore);
}

/// Trait for power flow solvers
pub trait Solver: Send + Sync {
    /// Solver name for debugging
    fn name(&self) -> &'static str;

    /// Solve power flow for the given topology and state
    fn solve(&self, topology: &Topology, state: &mut StateStore) -> Result<SolverResult>;
}

/// Result from a solver execution
#[derive(Debug, Clone)]
pub struct SolverResult {
    /// Number of iterations (for iterative solvers)
    pub iterations: usize,
    /// Final convergence error
    pub convergence_error: f64,
    /// Whether solver converged
    pub converged: bool,
}

impl SolverResult {
    /// Create a successful result
    pub fn converged(iterations: usize, error: f64) -> Self {
        Self {
            iterations,
            convergence_error: error,
            converged: true,
        }
    }

    /// Create a failed result
    pub fn failed(iterations: usize, error: f64) -> Self {
        Self {
            iterations,
            convergence_error: error,
            converged: false,
        }
    }
}

/// Trait for time stepping in simulations
pub trait TimeStepper: Send + Sync {
    /// Advance simulation by one step
    fn step(&mut self, state: &mut StateStore, dt: f64) -> Result<()>;
}

/// Trait for output/observation handlers
pub trait OutputHandler: Send + Sync {
    /// Called after each simulation step
    fn on_step(&mut self, step: usize, time: f64, state: &StateStore);

    /// Called when simulation completes
    fn on_complete(&mut self, state: &StateStore);
}
