use std::fmt;

use super::api::VeiculoApi;
use super::recurso::Entity;
use super::{extrair_id, extrair_ids, parse_numero, parse_texto_opcional, parse_u32, parse_u64};

#[derive(Debug, Clone)]
pub struct Veiculo {
    pub id: usize,
    pub name: String,
    pub model: String,
    pub manufacturer: String,
    pub cost_in_credits: Option<u64>,
    pub length: Option<f64>,
    pub max_atmosphering_speed: Option<f64>,
    pub crew: Option<u32>,
    pub passengers: Option<u32>,
    pub cargo_capacity: Option<u64>,
    pub consumables: Option<String>,
    pub vehicle_class: String,
    pub pilot_ids: Vec<usize>,
    pub film_ids: Vec<usize>,
    pub url: String,
}

impl From<VeiculoApi> for Veiculo {
    fn from(api: VeiculoApi) -> Self {
        Self {
            id: extrair_id(&api.url).unwrap_or(0),
            name: api.name,
            model: api.model,
            manufacturer: api.manufacturer,
            cost_in_credits: parse_u64(&api.cost_in_credits),
            length: parse_numero(&api.length),
            max_atmosphering_speed: parse_numero(&api.max_atmosphering_speed),
            crew: parse_u32(&api.crew),
            passengers: parse_u32(&api.passengers),
            cargo_capacity: parse_u64(&api.cargo_capacity),
            consumables: parse_texto_opcional(&api.consumables),
            vehicle_class: api.vehicle_class,
            pilot_ids: extrair_ids(&api.pilots),
            film_ids: extrair_ids(&api.films),
            url: api.url,
        }
    }
}

impl Entity for Veiculo {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> usize {
        self.id
    }
}

impl fmt::Display for Veiculo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} (modelo: {}, classe: {})",
            self.name, self.model, self.vehicle_class
        )
    }
}
