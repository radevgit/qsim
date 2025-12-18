//! Network serialization

use qsim_elements::{Branch, Bus, Generator, Load};
use serde::{Deserialize, Serialize};

/// Network definition for JSON serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkData {
    /// Network name
    #[serde(default)]
    pub name: String,
    /// Base MVA for per-unit conversion
    #[serde(default = "default_base_mva")]
    pub base_mva: f64,
    /// Buses
    pub buses: Vec<Bus>,
    /// Branches
    pub branches: Vec<Branch>,
    /// Generators
    #[serde(default)]
    pub generators: Vec<Generator>,
    /// Loads
    #[serde(default)]
    pub loads: Vec<Load>,
}

fn default_base_mva() -> f64 {
    100.0
}

impl NetworkData {
    /// Create an empty network
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            base_mva: 100.0,
            buses: Vec::new(),
            branches: Vec::new(),
            generators: Vec::new(),
            loads: Vec::new(),
        }
    }

    /// Load network from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Save network to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Load network from file
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, std::io::Error> {
        let json = std::fs::read_to_string(path)?;
        serde_json::from_str(&json).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Save network to file
    pub fn to_file(&self, path: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)
    }
}

impl Default for NetworkData {
    fn default() -> Self {
        Self::new("Unnamed Network")
    }
}
