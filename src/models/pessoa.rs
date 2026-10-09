use serde::Deserialize;
use std::fmt;

use super::recurso::Recurso;

#[derive(Debug, Clone, Deserialize)]
pub struct Pessoa {
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

impl Recurso for Pessoa {
    const CAMINHO: &'static str = "people";
}

impl fmt::Display for Pessoa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (nascimento: {}, gênero: {})", self.name, self.birth_year, self.gender)
    }
}
