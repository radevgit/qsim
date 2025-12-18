//! Generator element — power generation units

use qsim_core::{GridElement, StateStore};
use serde::{Deserialize, Serialize};

/// A generator connected to a bus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generator {
    /// Connected bus index
    pub bus: usize,
    /// Active power output (MW)
    pub active_power: f64,
    /// Reactive power output (MVAr)
    pub reactive_power: f64,
    /// Voltage setpoint (per-unit)
    pub voltage_setpoint: f64,
    /// Minimum active power (MW)
    pub p_min: f64,
    /// Maximum active power (MW)
    pub p_max: f64,
    /// Minimum reactive power (MVAr)
    pub q_min: f64,
    /// Maximum reactive power (MVAr)
    pub q_max: f64,
    /// Generator status (true = in service)
    pub in_service: bool,
}

impl Generator {
    /// Create a new generator
    pub fn new(bus: usize, active_power: f64, voltage_setpoint: f64) -> Self {
        Self {
            bus,
            active_power,
            reactive_power: 0.0,
            voltage_setpoint,
            p_min: 0.0,
            p_max: active_power * 2.0,
            q_min: -active_power,
            q_max: active_power,
            in_service: true,
        }
    }

    /// Create a generator with limits
    pub fn with_limits(
        bus: usize,
        active_power: f64,
        voltage_setpoint: f64,
        p_min: f64,
        p_max: f64,
        q_min: f64,
        q_max: f64,
    ) -> Self {
        Self {
            bus,
            active_power,
            reactive_power: 0.0,
            voltage_setpoint,
            p_min,
            p_max,
            q_min,
            q_max,
            in_service: true,
        }
    }
}

impl GridElement for Generator {
    fn element_type(&self) -> &'static str {
        "Generator"
    }

    fn apply(&self, state: &mut StateStore) {
        if self.in_service && self.bus < state.bus_count() {
            state.active_power[self.bus] += self.active_power;
            state.reactive_power[self.bus] += self.reactive_power;
        }
    }
}
