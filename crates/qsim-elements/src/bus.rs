//! Bus element — network nodes

use qsim_core::{GridElement, StateStore};
use serde::{Deserialize, Serialize};

/// Bus type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BusType {
    /// Slack bus — reference for voltage angle, absorbs power mismatch
    Slack,
    /// PV bus — voltage magnitude and active power specified (generator bus)
    PV,
    /// PQ bus — active and reactive power specified (load bus)
    PQ,
}

/// A bus (node) in the power network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bus {
    /// Bus type
    pub bus_type: BusType,
    /// Voltage magnitude (per-unit)
    pub voltage_magnitude: f64,
    /// Voltage angle (radians)
    pub voltage_angle: f64,
    /// Active power injection (MW, positive = generation)
    pub active_power: f64,
    /// Reactive power injection (MVAr, positive = generation)
    pub reactive_power: f64,
    /// Base voltage (kV)
    pub base_voltage_kv: f64,
}

impl Bus {
    /// Create a slack bus
    pub fn slack(voltage_magnitude: f64) -> Self {
        Self {
            bus_type: BusType::Slack,
            voltage_magnitude,
            voltage_angle: 0.0,
            active_power: 0.0,
            reactive_power: 0.0,
            base_voltage_kv: 1.0,
        }
    }

    /// Create a PV bus (generator)
    pub fn pv(voltage_magnitude: f64, active_power: f64) -> Self {
        Self {
            bus_type: BusType::PV,
            voltage_magnitude,
            voltage_angle: 0.0,
            active_power,
            reactive_power: 0.0,
            base_voltage_kv: 1.0,
        }
    }

    /// Create a PQ bus (load)
    pub fn pq(active_power: f64, reactive_power: f64) -> Self {
        Self {
            bus_type: BusType::PQ,
            voltage_magnitude: 1.0,
            voltage_angle: 0.0,
            active_power,
            reactive_power,
            base_voltage_kv: 1.0,
        }
    }
}

impl GridElement for Bus {
    fn element_type(&self) -> &'static str {
        match self.bus_type {
            BusType::Slack => "Bus::Slack",
            BusType::PV => "Bus::PV",
            BusType::PQ => "Bus::PQ",
        }
    }

    fn apply(&self, _state: &mut StateStore) {
        // Bus state is applied during network construction
    }
}
