pub mod api;
mod especie;
mod nave;
mod pessoa;
mod planeta;
mod recurso;
mod veiculo;

pub use especie::{ClassificacaoEspecie, DesignacaoEspecie, Especie};
pub use nave::Nave;
pub use pessoa::{Genero, Pessoa};
pub use planeta::Planeta;
pub use recurso::{Named, Recurso};
pub use veiculo::Veiculo;

use crate::data_structs::{HashMap, hashmap};

pub struct Store<T> {
    map: HashMap<usize, T>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: usize, value: T) {
        self.map.put(key, value);
    }

    pub fn get_by_id(&self, key: usize) -> Option<&T> {
        self.map.get(&key)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn collisions(&self) -> usize {
        self.map.collisions_qtt()
    }

    pub fn load_factor(&self) -> f64 {
        self.map.load_factor()
    }

    pub fn iter(&self) -> hashmap::Iter<'_, usize, T> {
        self.map.iter()
    }

    pub fn interval_search<F>(&self, min: f64, max: f64, field: F) -> Vec<&T>
    where
        F: Fn(&T) -> Option<f64>,
    {
        let mut vec: Vec<(f64, &T)> = self
            .map
            .iter()
            .filter_map(|(_, el)| field(el).map(|valor| (valor, el)))
            .filter(|(valor, _)| *valor >= min && *valor <= max)
            .collect();

        vec.sort_by(|a, b| a.0.total_cmp(&b.0));

        vec.into_iter().map(|(_, el)| el).collect()
    }
}

impl<'a, T> IntoIterator for &'a Store<T> {
    type Item = (&'a usize, &'a T);
    type IntoIter = hashmap::Iter<'a, usize, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Named> Store<T> {
    pub fn prefix_search(&self, prefix: &str) -> Vec<&T> {
        let mut resultado = Vec::new();
        for (_, el) in self.map.iter() {
            if el.name().to_lowercase().starts_with(&prefix.to_lowercase()) {
                resultado.push(el);
            }
        }
        resultado
    }
}

pub struct DataBase {
    pub people: Store<Pessoa>,
    pub planets: Store<Planeta>,
    pub species: Store<Especie>,
    pub vehicles: Store<Veiculo>,
    pub starships: Store<Nave>,
}

impl DataBase {
    pub fn new() -> Self {
        Self {
            people: Store::new(),
            planets: Store::new(),
            species: Store::new(),
            vehicles: Store::new(),
            starships: Store::new(),
        }
    }
}
pub fn parse_numero(texto: &str) -> Option<f64> {
    let limpo = texto.trim().replace(',', "");
    let primeiro = limpo.split_whitespace().next()?;
    primeiro.parse::<f64>().ok().filter(|v| v.is_finite())
}

pub fn parse_u64(texto: &str) -> Option<u64> {
    let limpo = texto.trim().replace(',', "");
    let primeiro = limpo.split_whitespace().next()?;
    primeiro.parse::<u64>().ok()
}

pub fn parse_u32(texto: &str) -> Option<u32> {
    let limpo = texto.trim().replace(',', "");
    let primeiro = limpo.split_whitespace().next()?;
    primeiro.parse::<u32>().ok()
}

pub fn parse_texto_opcional(texto: &str) -> Option<String> {
    let limpo = texto.trim();
    match limpo.to_lowercase().as_str() {
        "" | "unknown" | "n/a" | "none" | "indefinite" => None,
        _ => Some(limpo.to_string()),
    }
}

pub fn parse_lista(texto: &str) -> Vec<String> {
    texto.split(',').filter_map(parse_texto_opcional).collect()
}

pub fn extrair_id(url: &str) -> Option<usize> {
    url.trim_end_matches('/')
        .rsplit('/')
        .next()
        .and_then(|id| id.parse::<usize>().ok())
}

pub fn extrair_ids(urls: &[String]) -> Vec<usize> {
    urls.iter().filter_map(|u| extrair_id(u)).collect()
}
