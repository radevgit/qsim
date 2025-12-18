//! DC Power Flow Solver
//!
//! Linear approximation assuming:
//! - Voltage magnitudes = 1.0 p.u.
//! - Resistance negligible (R << X)
//! - Small angle differences (sin θ ≈ θ)
//!
//! Solves: P = B × θ

use nalgebra::{DMatrix, DVector};
use qsim_core::{CoreError, Result, Solver, SolverResult, StateStore, Topology};

/// DC Power Flow Solver
///
/// Uses B-matrix method for fast linear power flow solution.
#[derive(Debug, Clone)]
pub struct DcPowerFlowSolver {
    /// Tolerance for convergence check
    pub tolerance: f64,
}

impl DcPowerFlowSolver {
    /// Create a new DC power flow solver
    pub fn new() -> Self {
        Self { tolerance: 1e-6 }
    }

    /// Create with custom tolerance
    pub fn with_tolerance(tolerance: f64) -> Self {
        Self { tolerance }
    }

    /// Build the B matrix from topology
    /// 
    /// B[i][j] = -1/X[i][j] for connected buses
    /// B[i][i] = sum of 1/X for all branches connected to bus i
    pub fn build_b_matrix(&self, _topology: &Topology, state: &StateStore) -> DMatrix<f64> {
        let n = state.bus_count();
        let mut b_matrix = DMatrix::zeros(n, n);
        
        // TODO: Build from actual branch data
        // For now, return identity matrix as placeholder
        for i in 0..n {
            b_matrix[(i, i)] = 1.0;
        }
        
        b_matrix
    }
}

impl Default for DcPowerFlowSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver for DcPowerFlowSolver {
    fn name(&self) -> &'static str {
        "DC Power Flow"
    }

    fn solve(&self, topology: &Topology, state: &mut StateStore) -> Result<SolverResult> {
        let n = state.bus_count();
        
        if n == 0 {
            return Err(CoreError::SimulationError("No buses in network".into()));
        }
        
        if topology.bus_count() != n {
            return Err(CoreError::SimulationError(
                "Topology and state bus count mismatch".into(),
            ));
        }

        // Build B matrix
        let b_matrix = self.build_b_matrix(topology, state);
        
        // Build power injection vector (excluding slack bus)
        let p_vector = DVector::from_vec(state.active_power.clone());
        
        // Solve B × θ = P
        // For DC power flow, we exclude the slack bus (reference angle = 0)
        // Simplified: assume bus 0 is slack
        
        if n > 1 {
            // Reduced system (exclude slack bus)
            let b_reduced = b_matrix.view((1, 1), (n - 1, n - 1)).clone_owned();
            let p_reduced = p_vector.rows(1, n - 1).clone_owned();
            
            // Solve linear system
            match b_reduced.lu().solve(&p_reduced) {
                Some(theta) => {
                    // Update state with computed angles
                    state.voltage_angle[0] = 0.0; // Slack bus reference
                    for i in 0..theta.len() {
                        state.voltage_angle[i + 1] = theta[i];
                    }
                }
                None => {
                    return Err(CoreError::SimulationError(
                        "B matrix is singular".into(),
                    ));
                }
            }
        }
        
        Ok(SolverResult::converged(1, 0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver_creation() {
        let solver = DcPowerFlowSolver::new();
        assert_eq!(solver.name(), "DC Power Flow");
    }

    #[test]
    fn test_empty_network() {
        let solver = DcPowerFlowSolver::new();
        let topology = Topology::new();
        let mut state = StateStore::new(0);
        
        let result = solver.solve(&topology, &mut state);
        assert!(result.is_err());
    }
}
