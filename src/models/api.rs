use serde::Deserialize;

use super::recurso::Recurso;

/// DTOs que espelham exatamente o JSON retornado pela SWAPI (campos em String).
/// São usadas exclusivamente para o fetch e desserialização da API.

#[derive(Debug, Clone, Deserialize)]
pub struct PessoaApi {
    pub name: String,
    pub height: String,
    pub mass: String,
    pub hair_color: String,
    pub skin_color: String,
    pub eye_color: String,
    pub birth_year: String,
    pub gender: String,
    pub homeworld: String,
    pub films: Vec<String>,
    pub species: Vec<String>,
    pub vehicles: Vec<String>,
    pub starships: Vec<String>,
    pub url: String,
}

impl Recurso for PessoaApi {
    const CAMINHO: &'static str = "people";
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlanetaApi {
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

impl Recurso for PlanetaApi {
    const CAMINHO: &'static str = "planets";
}

#[derive(Debug, Clone, Deserialize)]
pub struct EspecieApi {
    pub name: String,
    pub classification: String,
    pub designation: String,
    pub average_height: String,
    pub skin_colors: String,
    pub hair_colors: String,
    pub eye_colors: String,
    pub average_lifespan: String,
    pub homeworld: Option<String>,
    pub language: String,
    pub people: Vec<String>,
    pub films: Vec<String>,
    pub url: String,
}

impl Recurso for EspecieApi {
    const CAMINHO: &'static str = "species";
}

#[derive(Debug, Clone, Deserialize)]
pub struct VeiculoApi {
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

impl Recurso for VeiculoApi {
    const CAMINHO: &'static str = "vehicles";
}

#[derive(Debug, Clone, Deserialize)]
pub struct NaveApi {
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

impl Recurso for NaveApi {
    const CAMINHO: &'static str = "starships";
}
