use serde::Deserialize;
use std::fmt;

use super::recurso::{Named, Recurso};

#[derive(Debug, Clone, Deserialize)]
pub struct Veiculo {
    pub name: String,
    pub model: String,
    pub manufacturer: String,
    pub cost_in_credits: String,
    pub length: String,
    pub max_atmosphering_speed: String,
    pub crew: String,
    pub passengers: String,
    pub cargo_capacity: String,
    pub consumables: String,
    pub vehicle_class: String,
    pub pilots: Vec<String>,
    pub films: Vec<String>,
    pub url: String,
}

impl Named for Veiculo {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Recurso for Veiculo {
    const CAMINHO: &'static str = "vehicles";
}

impl fmt::Display for Veiculo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (modelo: {}, classe: {})", self.name, self.model, self.vehicle_class)
    }
}
