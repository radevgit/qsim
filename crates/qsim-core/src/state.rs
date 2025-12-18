//! Central state storage for simulation

/// Central storage for simulation state.
///
/// Uses contiguous `Vec<f64>` arrays for cache-friendly access
/// and efficient parallel computation.
#[derive(Debug, Clone)]
pub struct StateStore {
    /// Voltage magnitudes (per-unit)
    pub voltage_magnitude: Vec<f64>,
    /// Voltage angles (radians)
    pub voltage_angle: Vec<f64>,
    /// Active power injection (MW)
    pub active_power: Vec<f64>,
    /// Reactive power injection (MVAr)
    pub reactive_power: Vec<f64>,
}

impl StateStore {
    /// Create a new state store with given capacity
    pub fn new(bus_count: usize) -> Self {
        Self {
            voltage_magnitude: vec![1.0; bus_count],
            voltage_angle: vec![0.0; bus_count],
            active_power: vec![0.0; bus_count],
            reactive_power: vec![0.0; bus_count],
        }
    }

    /// Number of buses in the state
    pub fn bus_count(&self) -> usize {
        self.voltage_magnitude.len()
    }

    /// Reset all values to defaults
    pub fn reset(&mut self) {
        self.voltage_magnitude.fill(1.0);
        self.voltage_angle.fill(0.0);
        self.active_power.fill(0.0);
        self.reactive_power.fill(0.0);
    }
}

impl Default for StateStore {
    fn default() -> Self {
        Self::new(0)
    }
}
