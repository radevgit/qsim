//! Load element — power consumption

use qsim_core::{GridElement, StateStore};
use serde::{Deserialize, Serialize};

/// A load connected to a bus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Load {
    /// Connected bus index
    pub bus: usize,
    /// Active power consumption (MW, positive = consumption)
    pub active_power: f64,
    /// Reactive power consumption (MVAr, positive = consumption)
    pub reactive_power: f64,
    /// Load status (true = in service)
    pub in_service: bool,
}

impl Load {
    /// Create a new load
    pub fn new(bus: usize, active_power: f64, reactive_power: f64) -> Self {
        Self {
            bus,
            active_power,
            reactive_power,
            in_service: true,
        }
    }

    /// Create a purely resistive load
    pub fn resistive(bus: usize, active_power: f64) -> Self {
        Self::new(bus, active_power, 0.0)
    }

    /// Create a load with power factor
    pub fn with_power_factor(bus: usize, apparent_power: f64, power_factor: f64) -> Self {
        let active_power = apparent_power * power_factor;
        let reactive_power = apparent_power * (1.0 - power_factor.powi(2)).sqrt();
        Self::new(bus, active_power, reactive_power)
    }
}

impl GridElement for Load {
    fn element_type(&self) -> &'static str {
        "Load"
    }

    fn apply(&self, state: &mut StateStore) {
        if self.in_service && self.bus < state.bus_count() {
            // Loads consume power (negative injection)
            state.active_power[self.bus] -= self.active_power;
            state.reactive_power[self.bus] -= self.reactive_power;
        }
    }
}
