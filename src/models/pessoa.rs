use std::fmt;

use super::api::PessoaApi;
use super::recurso::Named;
use super::{extrair_id, extrair_ids, parse_lista, parse_numero, parse_texto_opcional};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Genero {
    Masculino,
    Feminino,
    Hermafrodita,
    Nenhum,
    NaoAplicavel,
    Desconhecido,
    Outro(String),
}

impl From<&str> for Genero {
    fn from(valor: &str) -> Self {
        match valor.trim().to_lowercase().as_str() {
            "male" => Self::Masculino,
            "female" => Self::Feminino,
            "hermaphrodite" => Self::Hermafrodita,
            "none" => Self::Nenhum,
            "n/a" => Self::NaoAplicavel,
            "unknown" | "" => Self::Desconhecido,
            other => Self::Outro(other.to_string()),
        }
    }
}

impl fmt::Display for Genero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Masculino => write!(f, "masculino"),
            Self::Feminino => write!(f, "feminino"),
            Self::Hermafrodita => write!(f, "hermafrodita"),
            Self::Nenhum => write!(f, "nenhum"),
            Self::NaoAplicavel => write!(f, "n/a"),
            Self::Desconhecido => write!(f, "desconhecido"),
            Self::Outro(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pessoa {
    pub id: usize,
    pub name: String,
    pub height: Option<f64>,
    pub mass: Option<f64>,
    pub hair_colors: Vec<String>,
    pub skin_colors: Vec<String>,
    pub eye_colors: Vec<String>,
    pub birth_year: Option<String>,
    pub gender: Genero,
    pub homeworld_id: Option<usize>,
    pub film_ids: Vec<usize>,
    pub species_ids: Vec<usize>,
    pub vehicle_ids: Vec<usize>,
    pub starship_ids: Vec<usize>,
    pub url: String,
}

impl From<PessoaApi> for Pessoa {
    fn from(api: PessoaApi) -> Self {
        Self {
            id: extrair_id(&api.url).unwrap_or(0),
            name: api.name,
            height: parse_numero(&api.height),
            mass: parse_numero(&api.mass),
            hair_colors: parse_lista(&api.hair_color),
            skin_colors: parse_lista(&api.skin_color),
            eye_colors: parse_lista(&api.eye_color),
            birth_year: parse_texto_opcional(&api.birth_year),
            gender: Genero::from(api.gender.as_str()),
            homeworld_id: extrair_id(&api.homeworld),
            film_ids: extrair_ids(&api.films),
            species_ids: extrair_ids(&api.species),
            vehicle_ids: extrair_ids(&api.vehicles),
            starship_ids: extrair_ids(&api.starships),
            url: api.url,
        }
    }
}

impl Named for Pessoa {
    fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Pessoa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nascimento = self.birth_year.as_deref().unwrap_or("desconhecido");
        write!(
            f,
            "{} (nascimento: {}, gênero: {})",
            self.name, nascimento, self.gender
        )
    }
}
