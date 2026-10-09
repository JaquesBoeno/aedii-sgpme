use serde::Deserialize;
use std::fmt;

use super::recurso::Recurso;

#[derive(Debug, Clone, Deserialize)]
pub struct Nave {
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
    pub hyperdrive_rating: String,
    #[serde(rename = "MGLT")]
    pub mglt: String,
    pub starship_class: String,
    pub pilots: Vec<String>,
    pub films: Vec<String>,
    pub url: String,
}

impl Recurso for Nave {
    const CAMINHO: &'static str = "starships";
}

impl fmt::Display for Nave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (modelo: {}, classe: {})", self.name, self.model, self.starship_class)
    }
}
