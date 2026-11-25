use std::fmt;

use super::api::EspecieApi;
use super::recurso::Named;
use super::{extrair_id, extrair_ids, parse_lista, parse_numero, parse_texto_opcional};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassificacaoEspecie {
    Mamifero,
    Artificial,
    Senciente,
    Gastropode,
    Reptil,
    Anfibio,
    Insetoide,
    Desconhecida,
    Outra(String),
}

impl From<&str> for ClassificacaoEspecie {
    fn from(valor: &str) -> Self {
        match valor.trim().to_lowercase().as_str() {
            "mammal" | "mammals" => Self::Mamifero,
            "artificial" => Self::Artificial,
            "sentient" => Self::Senciente,
            "gastropod" => Self::Gastropode,
            "reptile" | "reptilian" => Self::Reptil,
            "amphibian" => Self::Anfibio,
            "insectoid" => Self::Insetoide,
            "unknown" | "n/a" | "" => Self::Desconhecida,
            other => Self::Outra(other.to_string()),
        }
    }
}

impl fmt::Display for ClassificacaoEspecie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Mamifero => write!(f, "mamífero"),
            Self::Artificial => write!(f, "artificial"),
            Self::Senciente => write!(f, "senciente"),
            Self::Gastropode => write!(f, "gastrópode"),
            Self::Reptil => write!(f, "réptil"),
            Self::Anfibio => write!(f, "anfíbio"),
            Self::Insetoide => write!(f, "insetoide"),
            Self::Desconhecida => write!(f, "desconhecida"),
            Self::Outra(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesignacaoEspecie {
    Senciente,
    NaoSenciente,
    Desconhecida,
    Outra(String),
}

impl From<&str> for DesignacaoEspecie {
    fn from(valor: &str) -> Self {
        match valor.trim().to_lowercase().as_str() {
            "sentient" => Self::Senciente,
            "non-sentient" => Self::NaoSenciente,
            "unknown" | "n/a" | "" => Self::Desconhecida,
            other => Self::Outra(other.to_string()),
        }
    }
}

impl fmt::Display for DesignacaoEspecie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Senciente => write!(f, "senciente"),
            Self::NaoSenciente => write!(f, "não senciente"),
            Self::Desconhecida => write!(f, "desconhecida"),
            Self::Outra(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Especie {
    pub id: usize,
    pub name: String,
    pub classification: ClassificacaoEspecie,
    pub designation: DesignacaoEspecie,
    pub average_height: Option<f64>,
    pub skin_colors: Vec<String>,
    pub hair_colors: Vec<String>,
    pub eye_colors: Vec<String>,
    pub average_lifespan: Option<f64>,
    pub homeworld_id: Option<usize>,
    pub language: Option<String>,
    pub people_ids: Vec<usize>,
    pub film_ids: Vec<usize>,
    pub url: String,
}

impl From<EspecieApi> for Especie {
    fn from(api: EspecieApi) -> Self {
        Self {
            id: extrair_id(&api.url).unwrap_or(0),
            name: api.name,
            classification: ClassificacaoEspecie::from(api.classification.as_str()),
            designation: DesignacaoEspecie::from(api.designation.as_str()),
            average_height: parse_numero(&api.average_height),
            skin_colors: parse_lista(&api.skin_colors),
            hair_colors: parse_lista(&api.hair_colors),
            eye_colors: parse_lista(&api.eye_colors),
            average_lifespan: parse_numero(&api.average_lifespan),
            homeworld_id: api.homeworld.as_deref().and_then(extrair_id),
            language: parse_texto_opcional(&api.language),
            people_ids: extrair_ids(&api.people),
            film_ids: extrair_ids(&api.films),
            url: api.url,
        }
    }
}

impl Named for Especie {
    fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Especie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let idioma = self.language.as_deref().unwrap_or("desconhecido");
        write!(
            f,
            "{} (classificação: {}, idioma: {})",
            self.name, self.classification, idioma
        )
    }
}
