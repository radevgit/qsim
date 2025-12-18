//! Branch element — lines and transformers

use qsim_core::{GridElement, StateStore};
use serde::{Deserialize, Serialize};

/// A branch (line or transformer) connecting two buses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    /// From bus index
    pub from_bus: usize,
    /// To bus index
    pub to_bus: usize,
    /// Resistance (per-unit)
    pub resistance: f64,
    /// Reactance (per-unit)
    pub reactance: f64,
    /// Total line charging susceptance (per-unit)
    pub susceptance: f64,
    /// Transformer tap ratio (1.0 for lines)
    pub tap_ratio: f64,
    /// Transformer phase shift (radians)
    pub phase_shift: f64,
    /// Branch status (true = in service)
    pub in_service: bool,
}

impl Branch {
    /// Create a transmission line
    pub fn line(from_bus: usize, to_bus: usize, resistance: f64, reactance: f64) -> Self {
        Self {
            from_bus,
            to_bus,
            resistance,
            reactance,
            susceptance: 0.0,
            tap_ratio: 1.0,
            phase_shift: 0.0,
            in_service: true,
        }
    }

    /// Create a transmission line with charging
    pub fn line_with_charging(
        from_bus: usize,
        to_bus: usize,
        resistance: f64,
        reactance: f64,
        susceptance: f64,
    ) -> Self {
        Self {
            from_bus,
            to_bus,
            resistance,
            reactance,
            susceptance,
            tap_ratio: 1.0,
            phase_shift: 0.0,
            in_service: true,
        }
    }

    /// Create a transformer
    pub fn transformer(
        from_bus: usize,
        to_bus: usize,
        resistance: f64,
        reactance: f64,
        tap_ratio: f64,
    ) -> Self {
        Self {
            from_bus,
            to_bus,
            resistance,
            reactance,
            susceptance: 0.0,
            tap_ratio,
            phase_shift: 0.0,
            in_service: true,
        }
    }

    /// Calculate series impedance magnitude
    pub fn impedance(&self) -> f64 {
        (self.resistance.powi(2) + self.reactance.powi(2)).sqrt()
    }

    /// Calculate series admittance (1/Z)
    pub fn admittance(&self) -> (f64, f64) {
        let z2 = self.resistance.powi(2) + self.reactance.powi(2);
        if z2 > 0.0 {
            (self.resistance / z2, -self.reactance / z2)
        } else {
            (0.0, 0.0)
        }
    }
}

impl GridElement for Branch {
    fn element_type(&self) -> &'static str {
        if (self.tap_ratio - 1.0).abs() < 1e-6 {
            "Branch::Line"
        } else {
            "Branch::Transformer"
        }
    }

    fn apply(&self, _state: &mut StateStore) {
        // Branch contributions applied during matrix construction
    }
}
