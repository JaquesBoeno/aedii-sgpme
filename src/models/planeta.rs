use std::fmt;

use super::api::PlanetaApi;
use super::recurso::Entity;
use super::{extrair_id, extrair_ids, parse_lista, parse_numero};

#[derive(Debug, Clone)]
pub struct Planeta {
    pub id: usize,
    pub name: String,
    pub rotation_period: Option<f64>,
    pub orbital_period: Option<f64>,
    pub diameter: Option<f64>,
    pub climate: Vec<String>,
    pub gravity: Option<f64>,
    pub terrain: Vec<String>,
    pub surface_water: Option<f64>,
    pub population: Option<u64>,
    pub resident_ids: Vec<usize>,
    pub film_ids: Vec<usize>,
    pub url: String,
}

impl From<PlanetaApi> for Planeta {
    fn from(api: PlanetaApi) -> Self {
        Self {
            id: extrair_id(&api.url).unwrap_or(0),
            name: api.name,
            rotation_period: parse_numero(&api.rotation_period),
            orbital_period: parse_numero(&api.orbital_period),
            diameter: parse_numero(&api.diameter),
            climate: parse_lista(&api.climate),
            gravity: parse_numero(&api.gravity),
            terrain: parse_lista(&api.terrain),
            surface_water: parse_numero(&api.surface_water),
            population: parse_numero(&api.population),
            resident_ids: extrair_ids(&api.residents),
            film_ids: extrair_ids(&api.films),
            url: api.url,
        }
    }
}

impl Entity for Planeta {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> usize {
        self.id
    }
}

impl fmt::Display for Planeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let clima = if self.climate.is_empty() {
            "desconhecido".to_string()
        } else {
            self.climate.join(", ")
        };
        let populacao = self
            .population
            .map_or_else(|| "desconhecida".to_string(), |p| p.to_string());
        write!(
            f,
            "{} (clima: {}, população: {})",
            self.name, clima, populacao
        )
    }
}
