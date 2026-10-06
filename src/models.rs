use serde::Deserialize;
use serde::de::DeserializeOwned;

/// como a api segue um padrao de url, basta mapear o tipo para o caminho do recurso
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

#[derive(Debug, Clone, Deserialize)]
pub struct Filme {
    pub title: String,
    pub episode_id: u32, // único campo numérico da API
    pub opening_crawl: String,
    pub director: String,
    pub producer: String,
    pub release_date: String,
    pub characters: Vec<String>,
    pub planets: Vec<String>,
    pub starships: Vec<String>,
    pub vehicles: Vec<String>,
    pub species: Vec<String>,
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

impl Recurso for Filme {
    const CAMINHO: &'static str = "films";
}