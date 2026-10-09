use serde::Deserialize;
use std::fmt;

use super::recurso::Recurso;

#[derive(Debug, Clone, Deserialize)]
pub struct Planeta {
    pub name: String,
    pub rotation_period: String,
    pub orbital_period: String,
    pub diameter: String,
    pub climate: String,
    pub gravity: String,
    pub terrain: String,
    pub surface_water: String,
    pub population: String,
    pub residents: Vec<String>,
    pub films: Vec<String>,
    pub url: String,
}

impl Recurso for Planeta {
    const CAMINHO: &'static str = "planets";
}

impl fmt::Display for Planeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (clima: {}, população: {})", self.name, self.climate, self.population)
    }
}
