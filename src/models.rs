use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::fmt;

/// como a api segue um padrao de url, basta mapear a struct para o caminho do recurso
pub trait Recurso: DeserializeOwned {
    const CAMINHO: &'static str;
}


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

impl Recurso for Pessoa {
    const CAMINHO: &'static str = "people";
}

impl Recurso for Planeta {
    const CAMINHO: &'static str = "planets";
}

impl Recurso for Especie {
    const CAMINHO: &'static str = "species";
}

impl Recurso for Veiculo {
    const CAMINHO: &'static str = "vehicles";
}

impl Recurso for Nave {
    const CAMINHO: &'static str = "starships";
}

impl fmt::Display for Pessoa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (nascimento: {}, gênero: {})", self.name, self.birth_year, self.gender)
    }
}

impl fmt::Display for Planeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (clima: {}, população: {})", self.name, self.climate, self.population)
    }
}

impl fmt::Display for Especie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (classificação: {}, idioma: {})", self.name, self.classification, self.language)
    }
}

impl fmt::Display for Veiculo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (modelo: {}, classe: {})", self.name, self.model, self.vehicle_class)
    }
}

impl fmt::Display for Nave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (modelo: {}, classe: {})", self.name, self.model, self.starship_class)
    }
}