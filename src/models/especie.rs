use serde::Deserialize;
use std::fmt;

use super::recurso::Recurso;

#[derive(Debug, Clone, Deserialize)]
pub struct Especie {
    pub name: String,
    pub classification: String,
    pub designation: String,
    pub average_height: String,
    pub skin_colors: String,
    pub hair_colors: String,
    pub eye_colors: String,
    pub average_lifespan: String,
    pub homeworld: Option<String>, // pode ser null na API
    pub language: String,
    pub people: Vec<String>,
    pub films: Vec<String>,
    pub url: String,
}

impl Recurso for Especie {
    const CAMINHO: &'static str = "species";
}

impl fmt::Display for Especie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (classificação: {}, idioma: {})", self.name, self.classification, self.language)
    }
}
